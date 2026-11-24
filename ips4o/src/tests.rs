//! The shared test suite, run against both the reference and the in-place
//! sort under each test configuration, plus a differential test between them.

use std::cmp::Ordering;
use std::mem;

use crate::config::{Config, DefaultConfig, block_size};
use crate::test_util::*;
use crate::util::Rng;
use crate::{inplace, reference};

/// A sort implementation under test.
trait Sorter {
    /// Whether the input still holds every element after a comparator panic.
    /// The reference sort consumes its `Vec`, so it drops them instead.
    const KEEPS_ELEMENTS_ON_PANIC: bool;

    fn sort<T, F, C>(v: &mut Vec<T>, compare: &mut F, seed: u64)
    where
        F: FnMut(&T, &T) -> Ordering,
        C: Config;
}

struct Reference;
impl Sorter for Reference {
    const KEEPS_ELEMENTS_ON_PANIC: bool = false;

    fn sort<T, F, C>(v: &mut Vec<T>, compare: &mut F, seed: u64)
    where
        F: FnMut(&T, &T) -> Ordering,
        C: Config,
    {
        *v = reference::sort_with_config::<T, F, C>(mem::take(v), compare, seed);
    }
}

struct InPlace;
impl Sorter for InPlace {
    const KEEPS_ELEMENTS_ON_PANIC: bool = true;

    fn sort<T, F, C>(v: &mut Vec<T>, compare: &mut F, seed: u64)
    where
        F: FnMut(&T, &T) -> Ordering,
        C: Config,
    {
        inplace::sort::<T, F, C>(v, compare, seed);
    }
}

fn by_key(a: &Tracked, b: &Tracked) -> Ordering {
    a.key.cmp(&b.key)
}

fn sorts_patterns<S: Sorter, C: Config>() {
    for n in lengths() {
        for (name, input) in patterns(n, n as u64) {
            let mut expected = input.clone();
            expected.sort_unstable();
            let mut v = input;
            S::sort::<_, _, C>(&mut v, &mut u64::cmp, n as u64);
            assert_eq!(v, expected, "pattern {name}, n={n}");
        }
    }
}

fn sorts_tracked_with_duplicates<S: Sorter, C: Config>() {
    for n in lengths() {
        for modulus in [1, 2, 3, 10, 1000] {
            let drops = DropCounter::new();
            let keys: Vec<u64> = random_keys(n, n as u64)
                .into_iter()
                .map(|k| k % modulus)
                .collect();
            let mut v = tracked(&keys, &drops);
            S::sort::<_, _, C>(&mut v, &mut by_key, n as u64);
            assert_sorted_permutation(&v, &keys);
            drop(v);
            assert_eq!(drops.get(), n, "n={n} modulus={modulus}");
        }
    }
}

/// The comparator mutates elements through `&T` (frees a `Box` held in a
/// `Cell`). If the sort ever compared against a bitwise copy of an element
/// while the original was still live, this would double-free, and Miri would
/// catch it.
fn survives_interior_mutation<S: Sorter, C: Config>() {
    for n in lengths() {
        let drops = DropCounter::new();
        let keys = random_keys(n, n as u64 ^ 0xabc);
        let mut v = tracked(&keys, &drops);
        let mut evil = |a: &Tracked, b: &Tracked| {
            drop(a.scratch.take());
            b.scratch.set(Some(Box::new(b.key)));
            by_key(a, b)
        };
        S::sort::<_, _, C>(&mut v, &mut evil, n as u64);
        assert_sorted_permutation(&v, &keys);
        drop(v);
        assert_eq!(drops.get(), n);
    }
}

/// Panics at various points, including in the middle of local
/// classification. The in-place sort must leave a full permutation behind.
fn comparator_panics<S: Sorter, C: Config>() {
    for n in lengths().into_iter().filter(|&n| n > 0).step_by(7) {
        let keys = random_keys(n, n as u64 ^ 0x5eed);
        for panic_at in [0, 1, n / 3, n / 2, n, 2 * n, 3 * n] {
            let drops = DropCounter::new();
            let mut v = tracked(&keys, &drops);
            let mut calls = 0;
            let mut compare = |a: &Tracked, b: &Tracked| {
                if calls == panic_at {
                    test_panic();
                }
                calls += 1;
                by_key(a, b)
            };
            let result = catch_test_panic(|| S::sort::<_, _, C>(&mut v, &mut compare, n as u64));
            match result {
                // Not enough comparisons happened to hit `panic_at`.
                Ok(()) => assert_sorted_permutation(&v, &keys),
                Err(()) if S::KEEPS_ELEMENTS_ON_PANIC => assert_permutation(&v, n),
                Err(()) => {}
            }
            drop(v);
            assert_eq!(drops.get(), n, "n={n} panic_at={panic_at}");
        }
    }
}

/// A comparator that is not an order at all. std's sort may panic on this
/// (it is allowed to), and so may we. Either way, no element may be lost or
/// duplicated.
fn garbage_comparator<S: Sorter, C: Config>() {
    for n in lengths().into_iter().step_by(3) {
        let keys = random_keys(n, n as u64);
        let drops = DropCounter::new();
        let mut v = tracked(&keys, &drops);
        let mut rng = Rng::new(n as u64);
        let orderings = [Ordering::Less, Ordering::Equal, Ordering::Greater];
        let mut garbage = |_: &Tracked, _: &Tracked| orderings[rng.below(3)];
        let result = catch_any_panic(|| S::sort::<_, _, C>(&mut v, &mut garbage, n as u64));
        if result.is_ok() || S::KEEPS_ELEMENTS_ON_PANIC {
            assert_permutation(&v, n);
        }
        drop(v);
        assert_eq!(drops.get(), n);
    }
}

/// Equality buckets should finish all-equal input in one partitioning pass:
/// about (L + 1) comparisons per element, plus the sample. Without them, the
/// input sinks into one bucket at every level.
fn all_equal_input_is_linear<S: Sorter>() {
    let n = 50_000;
    let mut comparisons = 0usize;
    let mut compare = |a: &u64, b: &u64| {
        comparisons += 1;
        a.cmp(b)
    };
    let mut v = vec![7u64; n];
    S::sort::<_, _, DefaultConfig>(&mut v, &mut compare, 0);
    assert_eq!(v.len(), n);
    assert!(
        comparisons < 12 * n,
        "{comparisons} comparisons for {n} equal elements"
    );
}

/// Sorted, reverse-sorted and all-equal input take one pass (at most
/// `n - 1` comparisons, plus one to pick the direction).
fn presorted_input_is_linear() {
    let n = 10_000u64;
    let inputs = [
        (0..n).collect::<Vec<_>>(),
        (0..n).rev().collect(),
        vec![7; n as usize],
    ];
    for input in inputs {
        let mut comparisons = 0u64;
        let mut compare = |a: &u64, b: &u64| {
            comparisons += 1;
            a.cmp(b)
        };
        let mut v = input;
        InPlace::sort::<_, _, DefaultConfig>(&mut v, &mut compare, 0);
        assert!(v.is_sorted());
        assert!(comparisons <= n, "{comparisons} comparisons");
    }
}

#[test]
fn inplace_presorted_input_is_linear() {
    presorted_input_is_linear();
}

fn large_inputs<S: Sorter>() {
    for n in [10_000, 100_000, 1 << 20] {
        for (name, input) in patterns(n, 42) {
            let mut expected = input.clone();
            expected.sort_unstable();
            let mut v = input;
            S::sort::<_, _, DefaultConfig>(&mut v, &mut u64::cmp, n as u64);
            assert_eq!(v, expected, "pattern {name}, n={n}");
        }
    }
}

fn zero_sized_types_never_call_the_comparator<S: Sorter>() {
    let mut v = vec![(); 1000];
    S::sort::<_, _, DefaultConfig>(&mut v, &mut |_, _| panic!("called"), 0);
    assert_eq!(v.len(), 1000);
}

/// With the same seed, both implementations pick the same splitters, so one
/// partitioning step must produce the same buckets: same sizes, same
/// elements (in any order within a bucket), same splitters in between.
fn same_partition_as_reference<C: Config>() {
    for n in lengths().into_iter().filter(|&n| n > C::BASE_CASE_SIZE) {
        for (name, input) in patterns(n, n as u64) {
            let seed = n as u64 ^ 0xd1ff;
            let expected =
                reference::partition::<_, _, C>(input.clone(), &mut u64::cmp, &mut Rng::new(seed));

            let mut v = input;
            let mut scratch = inplace::Scratch::new(block_size::<u64, C>());
            let got = inplace::partition::<_, _, C>(
                &mut v,
                &mut u64::cmp,
                &mut Rng::new(seed),
                &mut scratch,
            );

            let context = format!("pattern {name}, n={n}");
            assert_eq!(got.layout, expected.layout, "{context}");
            let mut splitters = expected.splitters.iter();
            let mut pos = 0;
            for (b, want) in expected.buckets.iter().enumerate() {
                let size = got.bucket_starts[b + 1] - got.bucket_starts[b];
                assert_eq!(size, want.len(), "{context}: size of bucket {b}");
                let mut have = v[pos..pos + size].to_vec();
                have.sort_unstable();
                let mut want = want.clone();
                want.sort_unstable();
                assert_eq!(have, want, "{context}: contents of bucket {b}");
                pos += size;
                if got.layout.splitter_after(b).is_some() {
                    assert_eq!(
                        Some(&v[pos]),
                        splitters.next(),
                        "{context}: splitter after bucket {b}"
                    );
                    pos += 1;
                }
            }
            assert_eq!(pos, n, "{context}");
        }
    }
}

/// Instantiates the suite for each implementation and configuration.
macro_rules! suite {
    ($($module:ident => $config:ty,)*) => {
        mod reference_sort {
            use super::*;
            $(mod $module {
                use super::*;
                #[test] #[cfg_attr(miri, ignore = "safe code: nothing for Miri to check")] fn patterns() { sorts_patterns::<Reference, $config>() }
                #[test] #[cfg_attr(miri, ignore = "safe code: nothing for Miri to check")] fn tracked_with_duplicates() { sorts_tracked_with_duplicates::<Reference, $config>() }
                #[test] #[cfg_attr(miri, ignore = "safe code: nothing for Miri to check")] fn interior_mutation() { survives_interior_mutation::<Reference, $config>() }
                #[test] #[cfg_attr(miri, ignore = "safe code: nothing for Miri to check")] fn panics() { comparator_panics::<Reference, $config>() }
                #[test] #[cfg_attr(miri, ignore = "safe code: nothing for Miri to check")] fn garbage() { garbage_comparator::<Reference, $config>() }
            })*
        }
        mod inplace_sort {
            use super::*;
            $(mod $module {
                use super::*;
                #[test] fn patterns() { sorts_patterns::<InPlace, $config>() }
                #[test] fn tracked_with_duplicates() { sorts_tracked_with_duplicates::<InPlace, $config>() }
                #[test] fn interior_mutation() { survives_interior_mutation::<InPlace, $config>() }
                #[test] fn panics() { comparator_panics::<InPlace, $config>() }
                #[test] fn garbage() { garbage_comparator::<InPlace, $config>() }
                #[test] fn same_partition_as_reference() { super::super::same_partition_as_reference::<$config>() }
            })*
        }
    };
}

suite! {
    two_buckets => TwoBuckets,
    four_buckets => FourBuckets,
    eight_buckets => EightBuckets,
    default_config => DefaultConfig,
}

macro_rules! per_impl {
    ($($name:ident => $sorter:ty,)*) => {$(
        mod $name {
            use super::*;
            #[test]
            #[cfg_attr(miri, ignore = "too slow under Miri")]
            fn all_equal_linear() { all_equal_input_is_linear::<$sorter>() }
            #[test]
            #[cfg_attr(miri, ignore = "too slow under Miri")]
            fn large() { large_inputs::<$sorter>() }
            #[test]
            fn zero_sized() { zero_sized_types_never_call_the_comparator::<$sorter>() }
        }
    )*};
}

per_impl! {
    reference_only => Reference,
    inplace_only => InPlace,
}

#[test]
fn public_api() {
    let mut v: Vec<i32> = (0..10_000).map(|i| (i * 7919) % 1000 - 500).collect();
    let mut expected = v.clone();
    expected.sort_unstable();
    crate::sort_unstable(&mut v);
    assert_eq!(v, expected);
    crate::sort_unstable_by(&mut v, |a, b| b.cmp(a));
    assert!(v.iter().rev().is_sorted());
    crate::sort_unstable_by_key(&mut v, |x| x.abs());
    assert!(v.windows(2).all(|w| w[0].abs() <= w[1].abs()));
}
