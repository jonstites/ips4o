//! Out-of-place reference implementation, in entirely safe Rust.
//!
//! It runs the same algorithm as the in-place sort: sample, build the
//! classifier, partition into buckets, recurse. Buckets are separate `Vec`s,
//! though, instead of blocks shuffled around inside the input. That makes it
//! a readable executable spec and a test oracle for the in-place version. It
//! is not meant to be fast.
//!
//! Because it owns the elements (`Vec<T>` in, `Vec<T>` out), every move is an
//! ordinary Rust move. If the comparator panics, the partially sorted data is
//! dropped by normal unwinding, with every element dropped exactly once.

use std::cmp::Ordering;
use std::mem;

use crate::classifier::{Classifier, select_splitters};
use crate::config::{Config, DefaultConfig};
use crate::util::{Rng, base_case_sort, is_less, random_seed};

/// Sorts `v` (unstable), returning the sorted vector.
pub fn sort_unstable<T: Ord>(v: Vec<T>) -> Vec<T> {
    sort_with_config::<T, _, DefaultConfig>(v, &mut T::cmp, random_seed())
}

/// Sorts `v` (unstable) with a comparator, returning the sorted vector.
///
/// Sampling is randomized, so the relative order of equal elements may
/// differ between calls.
///
/// If `compare` does not implement a total order, the result is some
/// unspecified permutation of the input, or the call panics. Either way it is
/// memory-safe and no element is leaked or dropped twice.
pub fn sort_unstable_by<T, F>(v: Vec<T>, mut compare: F) -> Vec<T>
where
    F: FnMut(&T, &T) -> Ordering,
{
    sort_with_config::<T, _, DefaultConfig>(v, &mut compare, random_seed())
}

/// The sort with explicit configuration and sampling seed (for tests).
pub(crate) fn sort_with_config<T, F, C>(v: Vec<T>, compare: &mut F, seed: u64) -> Vec<T>
where
    F: FnMut(&T, &T) -> Ordering,
    C: Config,
{
    // Matches std: sorting zero-sized values is a no-op that never calls the
    // comparator.
    if mem::size_of::<T>() == 0 {
        return v;
    }
    let n = v.len();
    let mut rng = Rng::new(seed);
    // Each level should shrink the input by a factor of at least 2. After
    // twice the ideal depth, give up on sampling and use std's sort. Random
    // sampling makes this vanishingly rare; it is cheap insurance that bounds
    // the worst case at O(n log n) regardless.
    let depth_limit = 2 * n.max(2).ilog2();
    recurse::<T, F, C>(v, compare, &mut rng, depth_limit)
}

fn recurse<T, F, C>(mut v: Vec<T>, compare: &mut F, rng: &mut Rng, depth_limit: u32) -> Vec<T>
where
    F: FnMut(&T, &T) -> Ordering,
    C: Config,
{
    if v.len() <= C::BASE_CASE_SIZE || depth_limit == 0 {
        base_case_sort(&mut v, compare);
        return v;
    }
    let n = v.len();

    // Sorted splitters end up in v[..m]. Move them into the classifier.
    let layout = select_splitters::<T, F, C>(&mut v, compare, rng);
    let rest = v.split_off(layout.num_splitters());
    let classifier = Classifier::new(v, layout.has_equal_buckets());

    // Partition. The splitters are not part of `rest`, so every bucket is
    // strictly smaller than `v`, and the recursion makes progress even if
    // the comparator is garbage.
    let mut buckets: Vec<Vec<T>> = (0..layout.num_buckets()).map(|_| Vec::new()).collect();
    {
        let mut is_less = is_less(compare);
        for x in rest {
            let b = classifier.classify(&x, &mut is_less);
            buckets[b].push(x);
        }
    }

    // Recurse, and put each splitter back after its bucket.
    let mut splitters = classifier.into_sorted_splitters().into_iter();
    let mut out = Vec::with_capacity(n);
    for (b, bucket) in buckets.into_iter().enumerate() {
        if layout.is_equality_bucket(b) {
            out.extend(bucket);
        } else {
            out.extend(recurse::<T, F, C>(bucket, compare, rng, depth_limit - 1));
        }
        if layout.splitter_after(b).is_some() {
            out.extend(splitters.next());
        }
    }
    debug_assert!(splitters.next().is_none());
    debug_assert_eq!(out.len(), n);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    fn sort<C: Config>(v: Vec<u64>) -> Vec<u64> {
        let seed = v.len() as u64;
        sort_with_config::<_, _, C>(v, &mut u64::cmp, seed)
    }

    fn by_key(a: &Tracked, b: &Tracked) -> Ordering {
        a.key.cmp(&b.key)
    }

    fn sorts_patterns<C: Config>() {
        for n in lengths() {
            for (name, input) in patterns(n, n as u64) {
                let mut expected = input.clone();
                expected.sort_unstable();
                assert_eq!(sort::<C>(input), expected, "pattern {name}, n={n}");
            }
        }
    }

    fn sorts_tracked_with_duplicates<C: Config>() {
        for n in lengths() {
            for modulus in [1, 2, 3, 10, 1000] {
                let drops = DropCounter::new();
                let keys: Vec<u64> = random_keys(n, n as u64)
                    .into_iter()
                    .map(|k| k % modulus)
                    .collect();
                let out =
                    sort_with_config::<_, _, C>(tracked(&keys, &drops), &mut by_key, n as u64);
                assert_sorted_permutation(&out, &keys);
                drop(out);
                assert_eq!(drops.get(), n, "n={n} modulus={modulus}");
            }
        }
    }

    /// The comparator mutates elements through `&T` (frees a `Box` held in a
    /// `Cell`). If the sort ever compared against a bitwise copy of an
    /// element while the original was still live, this would double-free,
    /// and Miri would catch it.
    fn survives_interior_mutation<C: Config>() {
        for n in lengths() {
            let drops = DropCounter::new();
            let keys = random_keys(n, n as u64 ^ 0xabc);
            let mut evil = |a: &Tracked, b: &Tracked| {
                drop(a.scratch.take());
                b.scratch.set(Some(Box::new(b.key)));
                by_key(a, b)
            };
            let out = sort_with_config::<_, _, C>(tracked(&keys, &drops), &mut evil, n as u64);
            assert_sorted_permutation(&out, &keys);
            drop(out);
            assert_eq!(drops.get(), n);
        }
    }

    fn panic_in_comparator_drops_everything_once<C: Config>() {
        for n in lengths().into_iter().filter(|&n| n > 0).step_by(7) {
            let keys = random_keys(n, n as u64 ^ 0x5eed);
            for panic_at in [0, 1, n / 2, n, 3 * n] {
                let drops = DropCounter::new();
                let input = tracked(&keys, &drops);
                let mut calls = 0;
                let result = catch_test_panic(|| {
                    let mut compare = |a: &Tracked, b: &Tracked| {
                        if calls == panic_at {
                            test_panic();
                        }
                        calls += 1;
                        by_key(a, b)
                    };
                    sort_with_config::<_, _, C>(input, &mut compare, n as u64)
                });
                if let Ok(out) = result {
                    // Not enough comparisons happened to hit `panic_at`.
                    assert_sorted_permutation(&out, &keys);
                    drop(out);
                }
                assert_eq!(drops.get(), n, "n={n} panic_at={panic_at}");
            }
        }
    }

    /// A comparator that is not an order at all. std's sort may panic on
    /// this (it is allowed to), and so may we. Otherwise the output must be
    /// a permutation of the input.
    fn garbage_comparator_is_safe<C: Config>() {
        for n in lengths().into_iter().step_by(3) {
            let keys = random_keys(n, n as u64);
            let drops = DropCounter::new();
            let input = tracked(&keys, &drops);
            let mut rng = Rng::new(n as u64);
            let result = catch_any_panic(|| {
                let orderings = [Ordering::Less, Ordering::Equal, Ordering::Greater];
                let mut garbage = |_: &Tracked, _: &Tracked| orderings[rng.below(3)];
                sort_with_config::<_, _, C>(input, &mut garbage, n as u64)
            });
            if let Ok(out) = result {
                assert_permutation(&out, n);
                drop(out);
            }
            assert_eq!(drops.get(), n);
        }
    }

    #[test]
    fn zero_sized_types_never_call_the_comparator() {
        let v = vec![(); 1000];
        let out = sort_with_config::<_, _, DefaultConfig>(v, &mut |_, _| panic!("called"), 0);
        assert_eq!(out.len(), 1000);
    }

    /// Equality buckets should finish all-equal input in one partitioning
    /// pass: about (L + 1) comparisons per element, plus the sample.
    /// Without them, the input sinks into one bucket at every level.
    #[test]
    #[cfg_attr(miri, ignore = "too slow under Miri")]
    fn all_equal_input_is_linear() {
        let n = 50_000;
        let mut comparisons = 0usize;
        let mut compare = |a: &u64, b: &u64| {
            comparisons += 1;
            a.cmp(b)
        };
        let out = sort_with_config::<_, _, DefaultConfig>(vec![7u64; n], &mut compare, 0);
        assert_eq!(out.len(), n);
        assert!(
            comparisons < 12 * n,
            "{comparisons} comparisons for {n} equal elements"
        );
    }

    #[test]
    fn public_api() {
        let v: Vec<i32> = (0..1000).map(|i| (i * 7919) % 1000).collect();
        let out = sort_unstable(v.clone());
        assert!(out.is_sorted());
        let out = sort_unstable_by(v, |a, b| b.cmp(a));
        assert!(out.iter().rev().is_sorted());
    }

    #[test]
    #[cfg_attr(miri, ignore = "too slow under Miri")]
    fn large_inputs_default_config() {
        for n in [10_000, 100_000, 1 << 20] {
            for (name, input) in patterns(n, 42) {
                let mut expected = input.clone();
                expected.sort_unstable();
                assert_eq!(
                    sort::<DefaultConfig>(input),
                    expected,
                    "pattern {name}, n={n}"
                );
            }
        }
    }

    /// Runs the full suite against each test configuration.
    macro_rules! suite {
        ($($module:ident => $config:ty,)*) => {$(
            mod $module {
                use super::*;
                #[test] fn patterns() { sorts_patterns::<$config>() }
                #[test] fn tracked_with_duplicates() { sorts_tracked_with_duplicates::<$config>() }
                #[test] fn interior_mutation() { survives_interior_mutation::<$config>() }
                #[test] fn comparator_panics() { panic_in_comparator_drops_everything_once::<$config>() }
                #[test] fn garbage_comparator() { garbage_comparator_is_safe::<$config>() }
            }
        )*};
    }

    suite! {
        two_buckets => TwoBuckets,
        four_buckets => FourBuckets,
        eight_buckets => EightBuckets,
        default_config => DefaultConfig,
    }
}
