//! Hostile comparators: they may panic at any call, or lie (answer at
//! random). Whatever happens, the sort must leave every element in the
//! slice exactly once, and each must be dropped exactly once. When the
//! comparator is honest and doesn't panic, the result must be sorted.
//!
//! Elements own a heap allocation, so a lost element leaks (and the drop
//! count catches it) and a duplicated one is freed twice (AddressSanitizer
//! catches it).

#![no_main]

#[allow(dead_code)]
mod common;

use std::cmp::Ordering;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Arc;
use std::sync::Once;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct Input {
    config: u8,
    parallel: bool,
    seed: u64,
    keys: Vec<u8>,
    /// Panic on this call to the comparator.
    panic_at: Option<u16>,
    /// Lie on calls whose hash (with this salt) falls below `lie_rate`.
    lie: Option<(u64, u8)>,
}

struct Element {
    key: u8,
    id: usize,
    heap: Box<usize>,
    drops: Arc<AtomicUsize>,
}

impl Drop for Element {
    fn drop(&mut self) {
        self.drops.fetch_add(1, SeqCst);
    }
}

/// Payload of the comparator's deliberate panics.
struct Deliberate;

/// libfuzzer-sys installs a panic hook that aborts, to report panics as
/// crashes. Let our own panics, and std's legitimate "not a total order"
/// panic, unwind instead. Any other panic still aborts; so does any panic in
/// the sort's memory-moving phases, which run under an abort-on-unwind guard.
fn allow_expected_panics() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let fuzzer_hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            let payload = info.payload();
            let message = payload
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| payload.downcast_ref::<String>().map(String::as_str));
            let expected =
                payload.is::<Deliberate>() || message.is_some_and(|m| m.contains("total order"));
            if !expected {
                fuzzer_hook(info);
            }
        }));
    });
}

fn mix(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

fuzz_target!(|input: Input| {
    allow_expected_panics();
    let n = input.keys.len();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut v: Vec<Element> = input
        .keys
        .iter()
        .enumerate()
        .map(|(id, &key)| Element {
            key,
            id,
            heap: Box::new(id),
            drops: drops.clone(),
        })
        .collect();

    let calls = AtomicUsize::new(0);
    let compare = |a: &Element, b: &Element| {
        let call = calls.fetch_add(1, SeqCst);
        if input.panic_at.is_some_and(|p| p as usize == call) {
            panic::panic_any(Deliberate);
        }
        if let Some((salt, rate)) = input.lie {
            let h = mix(salt ^ call as u64);
            if (h & 0xFF) < rate as u64 {
                return [Ordering::Less, Ordering::Equal, Ordering::Greater][(h >> 8) as usize % 3];
            }
        }
        a.key.cmp(&b.key)
    };

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        common::pool().install(|| {
            ips4o::fuzzing::sort(input.config, input.parallel, &mut v, &compare, input.seed)
        })
    }));

    // Every element exactly once, intact.
    assert_eq!(v.len(), n);
    let mut seen = vec![false; n];
    for e in &v {
        assert_eq!(*e.heap, e.id, "element corrupted");
        assert!(!seen[e.id], "element {} appears twice", e.id);
        seen[e.id] = true;
    }
    let honest = input.lie.is_none_or(|(_, rate)| rate == 0);
    if result.is_ok() && honest {
        assert!(v.windows(2).all(|w| w[0].key <= w[1].key), "not sorted");
    }
    drop(v);
    assert_eq!(drops.load(SeqCst), n, "elements leaked or dropped twice");
});
