//! Shared test fixtures: tiny configurations, input patterns, a drop-tracking
//! element type, and panic helpers.

use std::cell::Cell;
use std::panic::{self, AssertUnwindSafe};
use std::rc::Rc;
use std::sync::Once;

use crate::config::Config;
use crate::util::Rng;

/// Tiny configurations. Small inputs drive these through many levels of
/// partitioning, which keeps the tests (and Miri) fast.
pub(crate) struct TwoBuckets;
impl Config for TwoBuckets {
    const LOG_MAX_BUCKETS: u32 = 1;
    const BASE_CASE_SIZE: usize = 1;
    const BUCKET_TARGET_SIZE: usize = 1;
    const OVERSAMPLING_PERCENT: usize = 20;
    const EQUAL_BUCKETS_THRESHOLD: usize = 1;
}

pub(crate) struct FourBuckets;
impl Config for FourBuckets {
    const LOG_MAX_BUCKETS: u32 = 2;
    const BASE_CASE_SIZE: usize = 2;
    const BUCKET_TARGET_SIZE: usize = 1;
    const OVERSAMPLING_PERCENT: usize = 50;
    const EQUAL_BUCKETS_THRESHOLD: usize = 1;
}

pub(crate) struct EightBuckets;
impl Config for EightBuckets {
    const LOG_MAX_BUCKETS: u32 = 3;
    const BASE_CASE_SIZE: usize = 4;
    const BUCKET_TARGET_SIZE: usize = 2;
    const OVERSAMPLING_PERCENT: usize = 100;
    const EQUAL_BUCKETS_THRESHOLD: usize = 2;
}

/// Input lengths to test: everything small, plus a few around powers of two.
pub(crate) fn lengths() -> Vec<usize> {
    if cfg!(miri) {
        (0..24).chain([31, 32, 33, 64, 100]).collect()
    } else {
        (0..=300).chain([511, 512, 513, 1000, 1024, 4097]).collect()
    }
}

pub(crate) fn random_keys(n: usize, seed: u64) -> Vec<u64> {
    let mut rng = Rng::new(seed);
    (0..n).map(|_| rng.next_u64()).collect()
}

/// Named input distributions.
pub(crate) fn patterns(n: usize, seed: u64) -> Vec<(&'static str, Vec<u64>)> {
    let random = random_keys(n, seed);
    let n64 = n as u64;
    let sqrt = (n as f64).sqrt() as u64 + 1;
    vec![
        ("random", random.clone()),
        ("random_mod_3", random.iter().map(|x| x % 3).collect()),
        ("random_mod_sqrt", random.iter().map(|x| x % sqrt).collect()),
        ("all_equal", vec![7; n]),
        ("ascending", (0..n64).collect()),
        ("descending", (0..n64).rev().collect()),
        ("sawtooth", (0..n64).map(|i| i % sqrt).collect()),
        ("organ_pipe", (0..n64).map(|i| i.min(n64 - i)).collect()),
        (
            "two_values_skewed",
            random.iter().map(|x| (x % 100 == 0) as u64).collect(),
        ),
    ]
}

/// Counts drops of [`Tracked`] values.
#[derive(Clone)]
pub(crate) struct DropCounter(Rc<Cell<usize>>);

impl DropCounter {
    pub(crate) fn new() -> Self {
        DropCounter(Rc::new(Cell::new(0)))
    }

    pub(crate) fn get(&self) -> usize {
        self.0.get()
    }
}

/// An element that records its drop, remembers its original position, and
/// has inline interior mutability (so it is not `Freeze`).
pub(crate) struct Tracked {
    pub(crate) key: u64,
    pub(crate) id: usize,
    pub(crate) scratch: Cell<Option<Box<u64>>>,
    drops: DropCounter,
}

impl Drop for Tracked {
    fn drop(&mut self) {
        self.drops.0.set(self.drops.0.get() + 1);
    }
}

pub(crate) fn tracked(keys: &[u64], drops: &DropCounter) -> Vec<Tracked> {
    keys.iter()
        .enumerate()
        .map(|(id, &key)| Tracked {
            key,
            id,
            scratch: Cell::new(Some(Box::new(key))),
            drops: drops.clone(),
        })
        .collect()
}

/// Asserts that `out` holds each of the `n` original ids exactly once.
pub(crate) fn assert_permutation(out: &[Tracked], n: usize) {
    assert_eq!(out.len(), n);
    let mut seen = vec![false; n];
    for t in out {
        assert!(!seen[t.id], "id {} appears twice", t.id);
        seen[t.id] = true;
    }
}

/// Asserts that `out` is a permutation of the input whose keys are sorted.
pub(crate) fn assert_sorted_permutation(out: &[Tracked], keys: &[u64]) {
    assert_permutation(out, keys.len());
    for t in out {
        assert_eq!(t.key, keys[t.id], "element changed its key");
    }
    assert!(out.windows(2).all(|w| w[0].key <= w[1].key), "not sorted");
}

/// Panic payload for deliberate test panics; the panic hook stays quiet for it.
struct TestPanic;

pub(crate) fn test_panic() -> ! {
    panic::panic_any(TestPanic)
}

fn install_quiet_hook() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let default = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if !info.payload().is::<TestPanic>() {
                default(info);
            }
        }));
    });
}

/// Runs `f`, catching a [`test_panic`] (and only that).
pub(crate) fn catch_test_panic<R>(f: impl FnOnce() -> R) -> Result<R, ()> {
    install_quiet_hook();
    panic::catch_unwind(AssertUnwindSafe(f)).map_err(|payload| {
        if !payload.is::<TestPanic>() {
            panic::resume_unwind(payload);
        }
    })
}

/// Runs `f`, catching any panic.
pub(crate) fn catch_any_panic<R>(f: impl FnOnce() -> R) -> Result<R, ()> {
    panic::catch_unwind(AssertUnwindSafe(f)).map_err(|_| ())
}
