//! In-place (parallel) super scalar samplesort (IPS⁴o), in Rust.
//!
//! **Status:** work in progress. So far there is the classifier and a safe,
//! out-of-place [`reference`] implementation of the algorithm. The in-place
//! sequential and parallel sorts come next.
//!
//! Based on Axtmann, Witt, Ferizovic and Sanders, *In-Place Parallel Super
//! Scalar Samplesort (IPS⁴o)*, ESA 2017.

#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]

mod classifier;
mod config;
pub mod reference;
mod util;

#[cfg(test)]
mod test_util;
