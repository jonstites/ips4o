//! Shared test fixtures: tiny configurations, input patterns, a drop-tracking
//! element type, and panic helpers.

use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, Mutex, Once};

use crate::util::Rng;

pub(crate) use crate::config::small::{EightBuckets, FourBuckets, TwoBuckets};

/// Input lengths to test: everything small, plus a few around powers of two.
pub(crate) fn lengths() -> Vec<usize> {
    if cfg!(miri) {
        // Every length up to 9, then around powers of two.
        (0..=9).chain([13, 15, 16, 17, 31, 33, 64, 100]).collect()
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
        (
            "descending_with_duplicates",
            (0..n64).rev().map(|i| i / 3).collect(),
        ),
        (
            "ascending_then_one_out_of_place",
            (0..n64)
                .map(|i| if i + 1 == n64 { 0 } else { i + 1 })
                .collect(),
        ),
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
pub(crate) struct DropCounter(Arc<AtomicUsize>);

impl DropCounter {
    pub(crate) fn new() -> Self {
        DropCounter(Arc::new(AtomicUsize::new(0)))
    }

    pub(crate) fn get(&self) -> usize {
        self.0.load(SeqCst)
    }
}

/// An element that records its drop, remembers its original position, and
/// has inline interior mutability (so it is not `Freeze`). It is `Send` and
/// `Sync`, so the parallel sort can use it too.
pub(crate) struct Tracked {
    pub(crate) key: u64,
    pub(crate) id: usize,
    pub(crate) scratch: Mutex<Option<Box<u64>>>,
    drops: DropCounter,
}

impl Drop for Tracked {
    fn drop(&mut self) {
        self.drops.0.fetch_add(1, SeqCst);
    }
}

pub(crate) fn tracked(keys: &[u64], drops: &DropCounter) -> Vec<Tracked> {
    keys.iter()
        .enumerate()
        .map(|(id, &key)| Tracked {
            key,
            id,
            scratch: Mutex::new(Some(Box::new(key))),
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
