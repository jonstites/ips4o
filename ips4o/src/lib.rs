//! In-place (parallel) super scalar samplesort (IPS⁴o), in Rust.
//!
//! **Status:** work in progress. The sequential in-place sort works; the
//! parallel sort comes next. [`reference`] is a safe, out-of-place version
//! of the same algorithm, used for testing.
//!
//! Based on Axtmann, Witt, Ferizovic and Sanders, *In-Place Parallel Super
//! Scalar Samplesort (IPS⁴o)*, ESA 2017.
//!
//! # Behavior
//!
//! Like [`slice::sort_unstable`], the sorts here are unstable. Working memory
//! is about 1 MiB of buffers (511 blocks of 2 KiB, or of one element if
//! elements are larger), plus two bytes per block of input. Sampling is
//! randomized, so the order of equal elements may differ between calls.
//!
//! If the comparator panics, the panic propagates after the slice has been
//! restored to *some* permutation of its original contents. If the
//! comparator does not implement a total order, the result is an
//! unspecified permutation, or a panic. Either way, nothing is leaked,
//! dropped twice, or accessed out of bounds.

#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]

use std::cmp::Ordering;

mod classifier;
mod config;
mod inplace;
pub mod reference;
mod util;

#[cfg(test)]
mod test_util;
#[cfg(test)]
mod tests;

use config::DefaultConfig;
use util::random_seed;

/// Sorts the slice without preserving the order of equal elements.
///
/// ```
/// let mut v = [5, 1, 4, 2, 3];
/// ips4o::sort_unstable(&mut v);
/// assert_eq!(v, [1, 2, 3, 4, 5]);
/// ```
pub fn sort_unstable<T: Ord>(v: &mut [T]) {
    inplace::sort::<T, _, DefaultConfig>(v, &mut T::cmp, random_seed());
}

/// Sorts the slice with a comparator, without preserving the order of equal
/// elements.
///
/// ```
/// let mut v = [5, 1, 4, 2, 3];
/// ips4o::sort_unstable_by(&mut v, |a, b| b.cmp(a));
/// assert_eq!(v, [5, 4, 3, 2, 1]);
/// ```
pub fn sort_unstable_by<T, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    inplace::sort::<T, _, DefaultConfig>(v, &mut compare, random_seed());
}

/// Sorts the slice with a key extraction function, without preserving the
/// order of equal elements. The key function may be called many times per
/// element.
///
/// ```
/// let mut v = [-5i32, 1, -4, 2, 3];
/// ips4o::sort_unstable_by_key(&mut v, |x| x.abs());
/// assert_eq!(v, [1, 2, 3, -4, -5]);
/// ```
pub fn sort_unstable_by_key<T, K, F>(v: &mut [T], mut key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    sort_unstable_by(v, |a, b| key(a).cmp(&key(b)));
}
