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

use crate::classifier::{BucketLayout, Classifier, select_splitters};
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
    let Partition {
        layout,
        buckets,
        splitters,
    } = partition::<T, F, C>(v, compare, rng);

    // Recurse, and put each splitter back after its bucket.
    let mut splitters = splitters.into_iter();
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

/// The result of one partitioning step.
pub(crate) struct Partition<T> {
    pub(crate) layout: BucketLayout,
    pub(crate) buckets: Vec<Vec<T>>,
    /// In sorted order. Splitter `r` goes after the bucket that
    /// `layout.splitter_after` maps to `r`.
    pub(crate) splitters: Vec<T>,
}

/// One partitioning step. Requires `v.len() > C::BASE_CASE_SIZE`.
pub(crate) fn partition<T, F, C>(mut v: Vec<T>, compare: &mut F, rng: &mut Rng) -> Partition<T>
where
    F: FnMut(&T, &T) -> Ordering,
    C: Config,
{
    // Sorted splitters end up in v[..m]. Move them into the classifier.
    let layout = select_splitters::<T, F, C>(&mut v, compare, rng);
    let rest = v.split_off(layout.num_splitters());
    let classifier = Classifier::new(v, layout.has_equal_buckets());

    // The splitters are not part of `rest`, so every bucket is strictly
    // smaller than `v`, and the recursion makes progress even if the
    // comparator is garbage.
    let mut buckets: Vec<Vec<T>> = (0..layout.num_buckets()).map(|_| Vec::new()).collect();
    let mut is_less = is_less(compare);
    for x in rest {
        let b = classifier.classify(&x, &mut is_less);
        buckets[b].push(x);
    }
    Partition {
        layout,
        buckets,
        splitters: classifier.into_sorted_splitters(),
    }
}

#[cfg(test)]
mod tests {
    // The shared suite in `crate::tests` covers this implementation too.
    use super::*;

    #[test]
    fn public_api() {
        let v: Vec<i32> = (0..1000).map(|i| (i * 7919) % 1000).collect();
        let out = sort_unstable(v.clone());
        assert!(out.is_sorted());
        let out = sort_unstable_by(v, |a, b| b.cmp(a));
        assert!(out.iter().rev().is_sorted());
    }
}
