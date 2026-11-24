//! Phases 4 and 5: cleanup and splitter reinsertion.

use std::ops::Range;
use std::ptr;

use super::Scratch;
use super::permute::Placed;
use crate::classifier::BucketLayout;

/// Completes each bucket `[start, end)`. After block permutation, a bucket
/// looks like this (`|` marks block boundaries, `B` the block size):
///
/// ```text
///        start  area                       blocks_end  end
///   ... ---|-----|-----------------------------|--------|------ ...
///          [head)[ full blocks of this bucket )[ tail  )
/// ```
///
/// - The **head** `[start, area)` lies in the previous area, which may have
///   *spilled* its last block into it. Processing buckets in order empties
///   it first: each bucket moves its own spill out before the next bucket
///   is handled.
/// - The **spill** is the part of this bucket's last block past `end`. It
///   is shorter than the head: the bucket has at most `size / B` blocks, so
///   `blocks_end - area <= size`, that is, `blocks_end - end <= area - start`.
/// - The **tail** `[blocks_end, end)` is empty.
///
/// The head and tail are filled from the spill, the overflow block (if this
/// bucket owns it), and the bucket's buffer. These add up exactly, because
/// they are this bucket's elements that are not yet in its range.
///
/// # Safety
///
/// The state that `permute_blocks` returns (`placed`), for the same `data`
/// and `scratch`, with `bucket_starts` as passed to it.
pub(super) unsafe fn cleanup<T>(
    data: *mut T,
    bucket_starts: &[usize],
    placed: &Placed,
    scratch: &mut Scratch<T>,
) {
    let bs = scratch.block_size;
    let buffers = scratch.buffers_ptr();
    let overflow = scratch.overflow_ptr();

    for bucket in 0..bucket_starts.len() - 1 {
        let start = bucket_starts[bucket];
        let end = bucket_starts[bucket + 1];
        let area = start.div_ceil(bs) * bs;
        let owns_overflow = placed.overflow_bucket == Some(bucket);
        // Physical end of the blocks placed in this area. The overflow block,
        // if any, was logically placed last but lives in the overflow buffer.
        let blocks_end = placed.write[bucket] * bs - if owns_overflow { bs } else { 0 };

        let head = start..area.min(end);
        let tail = blocks_end.min(end).max(head.end)..end;
        let mut holes = Holes {
            data,
            ranges: [head, tail],
        };

        let spill = end.max(area)..blocks_end;
        if !spill.is_empty() {
            // SAFETY: the spill is this bucket's elements, initialized, in
            // the next bucket's (or buckets') range, outside the holes.
            unsafe { holes.fill(data.add(spill.start), spill.len()) };
        }
        if owns_overflow {
            // SAFETY: the overflow buffer holds one full block of this bucket.
            unsafe { holes.fill(overflow, bs) };
        }
        let fill = &mut scratch.buffer_fill[bucket];
        // SAFETY: the bucket's buffer holds `fill` initialized elements.
        unsafe { holes.fill(buffers.add(bucket * bs), *fill) };
        *fill = 0;
        assert!(holes.is_full(), "bucket {bucket} has unfilled holes");
    }
}

/// Up to two runs of uninitialized slots in `data`, filled in order.
pub(crate) struct Holes<T> {
    pub(crate) data: *mut T,
    pub(crate) ranges: [Range<usize>; 2],
}

impl<T> Holes<T> {
    /// Moves `count` elements from `src` into the next holes.
    ///
    /// # Safety
    ///
    /// `src[..count]` must be initialized, must not overlap the holes, and is
    /// moved from. The holes must be uninitialized slots of `data`.
    pub(crate) unsafe fn fill(&mut self, mut src: *const T, mut count: usize) {
        for range in &mut self.ranges {
            let n = count.min(range.len());
            // SAFETY: per the contract, `n` initialized source elements move
            // into `n` uninitialized, non-overlapping slots.
            unsafe { ptr::copy_nonoverlapping(src, self.data.add(range.start), n) };
            range.start += n;
            // SAFETY: `n <= count`, so this stays within the source.
            src = unsafe { src.add(n) };
            count -= n;
        }
        assert_eq!(count, 0, "more elements than holes");
    }

    pub(crate) fn is_full(&self) -> bool {
        self.ranges.iter().all(Range::is_empty)
    }
}

/// Closes the `num_splitters` holes at the front of the slice, putting
/// splitter `r` after the bucket `layout.splitter_after` names.
///
/// Before:  `[ holes | bucket 0 | bucket 1 | ... ]`
/// After:   `[ bucket 0 | s_1 | bucket 1 | s_2 | ... ]`
///
/// Order within a bucket doesn't matter, so shifting a bucket left by `h`
/// slots only takes moving its last `min(size, h)` elements into the holes
/// in front of it. In total that is at most `m^2 / 2` element moves.
///
/// # Safety
///
/// `base[..num_splitters]` must be uninitialized, followed by the
/// partitioned data with bucket sizes from `bucket_starts`. `splitters` must
/// be the `num_splitters` splitters in sorted order.
pub(crate) unsafe fn reinsert_splitters<T>(
    base: *mut T,
    num_splitters: usize,
    bucket_starts: &[usize],
    splitters: Vec<T>,
    layout: BucketLayout,
) {
    assert_eq!(splitters.len(), num_splitters);
    let mut splitters = splitters.into_iter();
    // Holes occupy `[hole, hole + holes)`, directly before the next bucket.
    let mut hole = 0;
    let mut holes = num_splitters;

    for bucket in 0..layout.num_buckets() {
        let size = bucket_starts[bucket + 1] - bucket_starts[bucket];
        let moved = size.min(holes);
        // SAFETY: the bucket occupies `[hole + holes, hole + holes + size)`.
        // Its last `moved` elements go to `[hole, hole + moved)`, which are
        // holes. The ranges don't overlap because `2 * moved <= holes + size`.
        unsafe {
            ptr::copy_nonoverlapping(base.add(hole + holes + size - moved), base.add(hole), moved);
        }
        // The bucket now occupies `[hole, hole + size)`, the holes follow it.
        hole += size;
        if layout.splitter_after(bucket).is_some() {
            assert!(holes > 0, "more splitter slots than splitters");
            let splitter = splitters.next().expect("one splitter per slot");
            // SAFETY: `hole` is the first of the `holes > 0` holes.
            unsafe { base.add(hole).write(splitter) };
            hole += 1;
            holes -= 1;
        }
    }
    assert!(holes == 0 && splitters.len() == 0, "splitters left over");
}
