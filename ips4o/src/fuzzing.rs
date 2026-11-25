//! Hooks for the fuzz targets in `fuzz/`, behind the `fuzzing` feature.
//! Not part of the public API.
//!
//! The fuzz targets are a separate crate, so they can only reach the
//! library through public items. This module exposes the sorts and one
//! partitioning step under the tiny test configurations, which drive short
//! inputs through every phase (many levels, tiny blocks, many stripes).

use std::cmp::Ordering;

use crate::config::small::{EightBuckets, FourBuckets, TwoBuckets};
use crate::config::{Config, DefaultConfig, base_case_size, block_size};
use crate::util::Rng;
use crate::{inplace, parallel, reference};

/// Calls `$body` with `$c` set to the configuration that `$config` picks
/// (any byte maps to one of four).
macro_rules! with_config {
    ($config:expr, $c:ident => $body:expr) => {
        match $config % 4 {
            0 => {
                type $c = TwoBuckets;
                $body
            }
            1 => {
                type $c = FourBuckets;
                $body
            }
            2 => {
                type $c = EightBuckets;
                $body
            }
            _ => {
                type $c = DefaultConfig;
                $body
            }
        }
    };
}

/// Sorts `v` with the in-place or parallel sort (the latter on the current
/// rayon pool) under configuration `config`, with sampling seed `seed`.
pub fn sort<T, F>(config: u8, parallel: bool, v: &mut [T], compare: &F, seed: u64)
where
    T: Send + Sync,
    F: Fn(&T, &T) -> Ordering + Sync,
{
    with_config!(config, C => {
        if parallel {
            parallel::sort::<T, F, C>(v, compare, seed);
        } else {
            inplace::sort::<T, _, C>(v, &mut |a, b| compare(a, b), seed);
        }
    })
}

/// Runs one partitioning step on `input`, in place (`stripes == 0`) or in
/// parallel with up to `stripes` stripes, and checks the result against the
/// reference with the same seed: same buckets (as multisets), same
/// splitters in between. Inputs too short to partition pass trivially.
pub fn check_partition(config: u8, input: &[u64], seed: u64, stripes: usize) -> Result<(), String> {
    with_config!(config, C => check_partition_with::<C>(input, seed, stripes))
}

fn check_partition_with<C: Config>(input: &[u64], seed: u64, stripes: usize) -> Result<(), String> {
    if input.len() <= base_case_size::<u64, C>() {
        return Ok(());
    }
    let expected =
        reference::partition::<_, _, C>(input.to_vec(), &mut u64::cmp, &mut Rng::new(seed));
    let mut v = input.to_vec();
    let got = if stripes == 0 {
        let mut scratch = inplace::Scratch::new(block_size::<u64, C>());
        inplace::partition::<_, _, C>(&mut v, &mut u64::cmp, &mut Rng::new(seed), &mut scratch)
    } else {
        let pool = parallel::ScratchPool::new(block_size::<u64, C>());
        parallel::partition::<_, _, C>(&mut v, &u64::cmp, &mut Rng::new(seed), stripes, &pool)
    };

    if got.layout != expected.layout {
        return Err(format!(
            "layout {:?}, expected {:?}",
            got.layout, expected.layout
        ));
    }
    let mut splitters = expected.splitters.iter();
    let mut pos = 0;
    for (b, want) in expected.buckets.iter().enumerate() {
        let size = got.bucket_starts[b + 1] - got.bucket_starts[b];
        if size != want.len() {
            return Err(format!("bucket {b}: size {size}, expected {}", want.len()));
        }
        let mut have = v[pos..pos + size].to_vec();
        have.sort_unstable();
        let mut want = want.clone();
        want.sort_unstable();
        if have != want {
            return Err(format!("bucket {b}: contents {have:?}, expected {want:?}"));
        }
        pos += size;
        if got.layout.splitter_after(b).is_some() {
            let expected = splitters.next();
            if Some(&v[pos]) != expected {
                return Err(format!(
                    "splitter after bucket {b}: {}, expected {expected:?}",
                    v[pos]
                ));
            }
            pos += 1;
        }
    }
    if pos != v.len() {
        return Err(format!("buckets cover {pos} of {} elements", v.len()));
    }
    Ok(())
}
