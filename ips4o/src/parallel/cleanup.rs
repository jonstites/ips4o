//! Phase 5: parallel cleanup.
//!
//! The sequential cleanup (`inplace::cleanup`) handles buckets in order,
//! because each bucket's *spill* (the part of its last block past its end)
//! sits in the next bucket's head, which must be emptied before the next
//! bucket can fill it. Here that order dependency is removed with two
//! passes, each parallel over buckets:
//!
//! 1. Move every bucket's spill (fewer than `B` elements) into its slot of
//!    a scratch array. Afterwards every head is empty.
//! 2. Fill each bucket's head and tail from its spill, the overflow block
//!    (if it owns it), and its buffer in every stripe.
//!
//! Buckets' holes, spills and buffers are pairwise disjoint, so the tasks
//! never touch the same memory.

use std::ptr;

use rayon::prelude::*;

use super::SendPtr;
use super::permute::Placed;
use crate::inplace::cleanup::Holes;
use crate::inplace::uninit_vec;

/// # Safety
///
/// The state `permute_blocks` returns (`placed`), for `data`, with
/// `buffers[s]`/`fills[s]` the buffers of stripe `s` (bucket `b`'s buffer
/// at `buffers[s] + b * bs`, holding `fills[s][b]` elements) and `overflow`
/// the overflow block.
pub(super) unsafe fn cleanup<T: Send>(
    data: SendPtr<T>,
    bs: usize,
    bucket_starts: &[usize],
    placed: &Placed,
    overflow: SendPtr<T>,
    buffers: &[SendPtr<T>],
    fills: &[&[usize]],
) {
    let num_buckets = bucket_starts.len() - 1;
    // Per bucket: where its area starts, and where its placed blocks
    // physically end (the overflow block, if any, lives elsewhere).
    let bounds = |b: usize| {
        let area = bucket_starts[b].div_ceil(bs) * bs;
        let owns_overflow = placed.overflow_bucket == Some(b);
        let blocks_end = placed.write[b] * bs - if owns_overflow { bs } else { 0 };
        (area, blocks_end, owns_overflow)
    };

    // Pass 1: spills out of the way.
    let mut spills = uninit_vec::<T>(num_buckets * bs);
    let spill_lens: Vec<usize> = spills
        .par_chunks_mut(bs)
        .enumerate()
        .map(|(b, spill)| {
            let (area, blocks_end, _) = bounds(b);
            let from = bucket_starts[b + 1].max(area);
            // Only touch `data` if there is a spill: for the last buckets,
            // `area` can lie past the end of `data`, and even computing a
            // pointer there is undefined behavior.
            if blocks_end <= from {
                return 0;
            }
            let count = blocks_end - from;
            // SAFETY: the spill is this bucket's elements, in
            // `[from, blocks_end)` with `blocks_end <= len`; no other
            // bucket's spill overlaps it. It moves into this bucket's own
            // slot (`count < bs`).
            unsafe {
                ptr::copy_nonoverlapping(
                    data.get().add(from),
                    spill.as_mut_ptr().cast::<T>(),
                    count,
                )
            };
            count
        })
        .collect();

    // Pass 2: fill every bucket's holes.
    let spills = SendPtr(spills.as_mut_ptr().cast::<T>());
    (0..num_buckets).into_par_iter().for_each(|b| {
        let (start, end) = (bucket_starts[b], bucket_starts[b + 1]);
        let (area, blocks_end, owns_overflow) = bounds(b);
        let head = start..area.min(end);
        let tail = blocks_end.min(end).max(head.end)..end;
        let mut holes = Holes {
            data: data.get(),
            ranges: [head, tail],
        };
        // SAFETY (all fills): each source holds this bucket's elements and
        // is moved from; the holes are this bucket's empty slots. Different
        // buckets use disjoint holes and sources.
        unsafe {
            holes.fill(spills.get().add(b * bs), spill_lens[b]);
            if owns_overflow {
                holes.fill(overflow.get(), bs);
            }
            for (buffer, fill) in buffers.iter().zip(fills) {
                holes.fill(buffer.get().add(b * bs), fill[b]);
            }
        }
        assert!(holes.is_full(), "bucket {b} has unfilled holes");
    });
}
