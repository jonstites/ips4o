//! Splitter selection and the branchless bucket classifier.
//!
//! # Bucket semantics
//!
//! With sorted splitters `s_1 <= ... <= s_m` (`m = 2^L - 1`, and
//! `s_0 = -inf`, `s_{m+1} = +inf`), element `x` lands in leaf `j` such that
//! `s_j < x <= s_{j+1}`.
//!
//! Without equality buckets, bucket `j` is leaf `j` (`2^L` buckets).
//!
//! With equality buckets, leaf `j` is split in two: bucket `2j + 1` holds the
//! elements equal to `s_{j+1}` and bucket `2j` holds the rest (strictly between
//! the splitters). The last leaf has no right splitter and becomes bucket `2m`,
//! giving `2^(L+1) - 1` buckets. Equality buckets need no further sorting.
//!
//! # Splitter ownership (design "Option C")
//!
//! The splitters are *moved* out of the input into the classifier: there is
//! only ever one live instance of each, so comparisons through `&T` are
//! observed consistently even for types with interior mutability. The caller
//! gets them back in sorted order with [`Classifier::into_sorted_splitters`]
//! and puts splitter `r` right after the bucket given by
//! [`BucketLayout::splitter_after`].
//!
//! Duplicate splitters are allowed and simply produce empty buckets. This
//! avoids the C++ trick of padding the tree with copies of the largest
//! splitter.

use std::cmp::Ordering;

use crate::config::{self, Config};
use crate::util::{Rng, base_case_sort, is_less};

/// The bucket structure of one partitioning step. Cheap to copy, so callers
/// can keep it after the classifier has been consumed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BucketLayout {
    log_buckets: u32,
    equal_buckets: bool,
}

impl BucketLayout {
    pub(crate) fn new(log_buckets: u32, equal_buckets: bool) -> Self {
        assert!((1..usize::BITS - 1).contains(&log_buckets));
        BucketLayout {
            log_buckets,
            equal_buckets,
        }
    }

    /// `m`, the number of splitters (`2^L - 1`).
    pub(crate) fn num_splitters(self) -> usize {
        (1 << self.log_buckets) - 1
    }

    pub(crate) fn num_buckets(self) -> usize {
        if self.equal_buckets {
            2 * self.num_splitters() + 1
        } else {
            self.num_splitters() + 1
        }
    }

    pub(crate) fn has_equal_buckets(self) -> bool {
        self.equal_buckets
    }

    /// Whether every element of `bucket` compares equal to a splitter (so the
    /// bucket needs no recursion).
    pub(crate) fn is_equality_bucket(self, bucket: usize) -> bool {
        self.equal_buckets && bucket % 2 == 1
    }

    /// The rank (1-based, in sorted order) of the splitter that goes directly
    /// after `bucket` in the output, if any. Ranks come out in increasing
    /// order as `bucket` increases, and each rank in `1..=m` appears once.
    pub(crate) fn splitter_after(self, bucket: usize) -> Option<usize> {
        let m = self.num_splitters();
        if self.equal_buckets {
            (bucket % 2 == 1).then_some(bucket.div_ceil(2))
        } else {
            (bucket < m).then_some(bucket + 1)
        }
    }
}

/// Owns the `2^L - 1` splitters, laid out as an implicit binary search tree
/// (Eytzinger order): 1-based node `b` has children `2b` and `2b + 1` and
/// lives at `tree[b - 1]`.
pub(crate) struct Classifier<T> {
    tree: Vec<T>,
    layout: BucketLayout,
}

impl<T> Classifier<T> {
    /// Builds a classifier from splitters already in sorted order.
    ///
    /// # Panics
    ///
    /// If `sorted_splitters.len() + 1` is not a power of two `>= 2`.
    pub(crate) fn new(sorted_splitters: Vec<T>, equal_buckets: bool) -> Self {
        let m = sorted_splitters.len();
        assert!(
            m >= 1 && (m + 1).is_power_of_two(),
            "need 2^L - 1 splitters, got {m}"
        );
        let layout = BucketLayout::new((m + 1).ilog2(), equal_buckets);

        // Permute sorted order into tree order. `m` is at most a few hundred,
        // so the `Option` shuffle costs nothing next to classification.
        let mut slots: Vec<Option<T>> = sorted_splitters.into_iter().map(Some).collect();
        let tree = (1..=m)
            .map(|node| {
                slots[rank_of_node(node, layout.log_buckets) - 1]
                    .take()
                    .expect("tree order is a permutation")
            })
            .collect();
        Classifier { tree, layout }
    }

    /// The splitter with 1-based sorted `rank`.
    pub(crate) fn splitter(&self, rank: usize) -> &T {
        &self.tree[node_of_rank(rank, self.layout.log_buckets) - 1]
    }

    /// Returns the bucket of `x`. The result is in `0..num_buckets()` for
    /// *any* `is_less`, even one that is not a strict weak order, because the
    /// index is built from exactly `L` (or `L + 1`) single-bit decisions.
    #[inline]
    pub(crate) fn classify<F>(&self, x: &T, is_less: &mut F) -> usize
    where
        F: FnMut(&T, &T) -> bool,
    {
        let log_buckets = self.layout.log_buckets;
        let mut b = 1;
        for _ in 0..log_buckets {
            // Go right iff splitter < x, so leaves hold `s_j < x <= s_{j+1}`.
            b = 2 * b + is_less(&self.tree[b - 1], x) as usize;
        }
        let leaf = b - (1 << log_buckets);

        if self.layout.equal_buckets {
            // We already know x <= s_{leaf+1}; it is equal iff !(x < s_{leaf+1}).
            let rank = leaf + 1;
            let equal = rank <= self.tree.len() && !is_less(x, self.splitter(rank));
            2 * leaf + equal as usize
        } else {
            leaf
        }
    }

    /// Consumes the classifier, returning the splitters in sorted order.
    pub(crate) fn into_sorted_splitters(self) -> Vec<T> {
        let log_buckets = self.layout.log_buckets;
        let m = self.tree.len();
        let mut slots: Vec<Option<T>> = self.tree.into_iter().map(Some).collect();
        (1..=m)
            .map(|rank| {
                slots[node_of_rank(rank, log_buckets) - 1]
                    .take()
                    .expect("tree order is a permutation")
            })
            .collect()
    }
}

/// Sorted rank (1-based) of tree node `node` (1-based) in a complete tree of
/// depth `log_buckets`.
fn rank_of_node(node: usize, log_buckets: u32) -> usize {
    let depth = node.ilog2();
    let index_in_level = node - (1 << depth);
    (2 * index_in_level + 1) << (log_buckets - 1 - depth)
}

/// Tree node (1-based) holding sorted rank `rank` (1-based): the inverse of
/// [`rank_of_node`]. A rank with `t` trailing zeros sits `t` levels above the
/// leaves.
fn node_of_rank(rank: usize, log_buckets: u32) -> usize {
    let tz = rank.trailing_zeros();
    (1 << (log_buckets - 1 - tz)) + (rank >> (tz + 1))
}

/// Picks the splitters for partitioning `v`.
///
/// Draws a random sample, sorts it, and moves every `step`-th sample element
/// to `v[..m]` in sorted order. The rest of the sample is left in `v[m..]` as
/// ordinary elements. Decides whether to use equality buckets.
///
/// Requires `v.len() > C::BASE_CASE_SIZE`.
pub(crate) fn select_splitters<T, F, C>(v: &mut [T], compare: &mut F, rng: &mut Rng) -> BucketLayout
where
    F: FnMut(&T, &T) -> Ordering,
    C: Config,
{
    let n = v.len();
    let log_buckets = config::log_buckets::<C>(n);
    let m = (1usize << log_buckets) - 1;
    let step = config::sample_step::<C>(n, m);
    let sample_len = step * m;

    select_sample(v, sample_len, rng);
    // TODO: once the in-place sort exists, sort large samples with it.
    base_case_sort(&mut v[..sample_len], compare);

    // Compact splitters `s_r = sample[r * step - 1]` to the front, keeping
    // their order. Source positions are strictly increasing and never below
    // the destination, so no splitter is disturbed before it is moved.
    for r in 1..=m {
        v.swap(r - 1, r * step - 1);
    }

    let mut is_less = is_less(compare);
    let duplicates = v[..m].windows(2).filter(|w| !is_less(&w[0], &w[1])).count();
    let threshold = C::EQUAL_BUCKETS_THRESHOLD.min(m / 2).max(1);
    BucketLayout::new(log_buckets, duplicates >= threshold)
}

/// Moves `count` uniformly random elements of `v` to `v[..count]`
/// (a partial Fisher–Yates shuffle).
fn select_sample<T>(v: &mut [T], count: usize, rng: &mut Rng) {
    let n = v.len();
    assert!(count <= n);
    for i in 0..count {
        v.swap(i, i + rng.below(n - i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_L: u32 = if cfg!(miri) { 5 } else { 10 };

    #[test]
    fn rank_node_roundtrip() {
        for l in 1..=MAX_L {
            let m = (1usize << l) - 1;
            let mut seen = vec![false; m + 1];
            for node in 1..=m {
                let rank = rank_of_node(node, l);
                assert!((1..=m).contains(&rank));
                assert!(!seen[rank], "rank {rank} used twice");
                seen[rank] = true;
                assert_eq!(node_of_rank(rank, l), node);
            }
        }
    }

    #[test]
    fn tree_layout() {
        let c = Classifier::new(vec![10, 20, 30, 40, 50, 60, 70], false);
        assert_eq!(c.tree, [40, 20, 60, 10, 30, 50, 70]);
        assert_eq!(c.into_sorted_splitters(), [10, 20, 30, 40, 50, 60, 70]);
    }

    /// Reference classification by linear scan of sorted splitters.
    fn expected_bucket(splitters: &[i64], x: i64, equal_buckets: bool) -> usize {
        let leaf = splitters.iter().filter(|&&s| s < x).count();
        if !equal_buckets {
            leaf
        } else if leaf < splitters.len() && splitters[leaf] == x {
            2 * leaf + 1
        } else {
            2 * leaf
        }
    }

    fn check_against_scan(splitters: Vec<i64>, equal_buckets: bool) {
        let lo = splitters[0] - 2;
        let hi = splitters[splitters.len() - 1] + 2;
        let c = Classifier::new(splitters.clone(), equal_buckets);
        let layout = c.layout;
        for x in lo..=hi {
            let b = c.classify(&x, &mut |a, b| a < b);
            assert!(b < layout.num_buckets());
            assert_eq!(
                b,
                expected_bucket(&splitters, x, equal_buckets),
                "x={x} splitters={splitters:?}"
            );
        }
    }

    #[test]
    fn classify_distinct_splitters() {
        for l in 1..=MAX_L {
            let m = (1i64 << l) - 1;
            let splitters: Vec<i64> = (1..=m).map(|r| 3 * r).collect();
            check_against_scan(splitters.clone(), false);
            check_against_scan(splitters, true);
        }
    }

    #[test]
    fn classify_duplicate_splitters() {
        let mut rng = Rng::new(7);
        for l in 1..=MAX_L {
            let m = 1usize << l;
            for _ in 0..4 {
                let mut splitters: Vec<i64> = (1..m).map(|_| rng.below(m / 2 + 1) as i64).collect();
                splitters.sort();
                check_against_scan(splitters.clone(), false);
                check_against_scan(splitters, true);
            }
        }
    }

    #[test]
    fn classify_in_range_for_garbage_comparator() {
        let mut rng = Rng::new(11);
        for equal_buckets in [false, true] {
            let c = Classifier::new((0..63).collect::<Vec<u32>>(), equal_buckets);
            for x in 0..200 {
                let b = c.classify(&x, &mut |_, _| rng.next_u64() & 1 == 0);
                assert!(b < c.layout.num_buckets());
            }
        }
    }

    #[test]
    fn splitter_after_visits_each_rank_once() {
        for l in 1..=MAX_L {
            for eq in [false, true] {
                let layout = BucketLayout::new(l, eq);
                let ranks: Vec<usize> = (0..layout.num_buckets())
                    .filter_map(|b| layout.splitter_after(b))
                    .collect();
                assert_eq!(ranks, (1..=layout.num_splitters()).collect::<Vec<_>>());
                // Every equality bucket is followed by the splitter it equals.
                for b in 0..layout.num_buckets() {
                    if layout.is_equality_bucket(b) {
                        assert!(layout.splitter_after(b).is_some());
                    }
                }
            }
        }
    }

    #[test]
    fn select_sample_is_a_permutation() {
        let mut rng = Rng::new(3);
        let mut v: Vec<u32> = (0..100).collect();
        select_sample(&mut v, 30, &mut rng);
        v.sort();
        assert_eq!(v, (0..100).collect::<Vec<_>>());
    }
}
