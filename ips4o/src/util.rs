//! Small helpers shared across the implementation.

use std::cmp::Ordering;
use std::hash::{BuildHasher, RandomState};
use std::io::Write;
use std::mem;

/// SplitMix64. Fast, tiny, and good enough for sampling.
pub(crate) struct Rng(u64);

impl Rng {
    pub(crate) fn new(seed: u64) -> Self {
        Rng(seed)
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Returns a value in `0..n` using multiply-shift (bias at most `n / 2^64`).
    pub(crate) fn below(&mut self, n: usize) -> usize {
        ((self.next_u64() as u128 * n as u128) >> 64) as usize
    }
}

/// A fresh random seed for one sort call.
///
/// `RandomState` reads OS randomness once per thread and then derives new keys
/// cheaply, so this costs a hash, not a syscall. With random sampling no fixed
/// input is reliably bad, which is the setting IPS⁴o's analysis assumes.
pub(crate) fn random_seed() -> u64 {
    RandomState::new().hash_one(0u8)
}

/// The strict "less than" view of `compare` that the partitioning code works
/// with. This is exactly how std derives `is_less` from a comparator.
pub(crate) fn is_less<T, F>(compare: &mut F) -> impl FnMut(&T, &T) -> bool + '_
where
    F: FnMut(&T, &T) -> Ordering,
{
    move |a, b| compare(a, b) == Ordering::Less
}

/// Sorts a small slice with std's unstable sort and the caller's own
/// comparator, so the base case behaves exactly like calling std directly.
pub(crate) fn base_case_sort<T, F>(v: &mut [T], compare: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    v.sort_unstable_by(compare);
}

/// Aborts the process if dropped, which only happens while unwinding: the
/// normal path calls [`AbortOnUnwind::defuse`].
///
/// Wraps phases that only move memory. They don't call user code, so they
/// can only panic on an internal bug (a failed assertion). If one did panic
/// mid-move, unwinding would drop elements that also exist elsewhere, or
/// that are uninitialized. Aborting is the only safe response.
pub(crate) struct AbortOnUnwind;

impl AbortOnUnwind {
    pub(crate) fn defuse(self) {
        mem::forget(self);
    }
}

impl Drop for AbortOnUnwind {
    fn drop(&mut self) {
        let _ = writeln!(
            std::io::stderr(),
            "ips4o: internal invariant violated while moving elements; aborting"
        );
        std::process::abort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_vary_between_calls() {
        let seeds: Vec<u64> = (0..8).map(|_| random_seed()).collect();
        assert!(seeds.windows(2).any(|w| w[0] != w[1]), "{seeds:?}");
    }
}
