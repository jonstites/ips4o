//! Phase 2: local classification.

use std::cmp::Ordering;
use std::mem;
use std::ptr;

use super::Scratch;
use crate::classifier::Classifier;
use crate::util::is_less;

/// Streams through `data[..len]`, moving each element into its bucket's
/// buffer. Whenever a buffer is full and needs room, it is written back to
/// the front of `data` as a block, and the block's bucket is recorded.
///
/// The invariant that makes this work in place: at all times,
///
/// ```text
/// data: [ full blocks | holes | not yet read ]
///       0           write   read            len
/// ```
///
/// and the number of holes equals the number of elements in the buffers. So
/// a full buffer (`block_size` elements) always fits in the holes.
///
/// Returns `written`, the number of elements in full blocks (a multiple of
/// the block size). On return:
/// - `data[..written]` holds full blocks; block `i` belongs to bucket
///   `scratch.block_ids[i]`;
/// - `data[written..len]` is uninitialized;
/// - bucket `b`'s buffer holds `scratch.buffer_fill[b]` elements;
/// - `scratch.bucket_sizes[b]` is the number of elements in bucket `b`.
///
/// If `compare` panics, `data[..len]` is restored to a permutation of its
/// initial contents and the buffers are empty.
///
/// A bucket with `k` elements gets at most `k / block_size` full blocks
/// (in fact at most `(k - 1) / block_size`, since a buffer is only flushed
/// when one more element arrives). Later phases rely on the first bound.
///
/// # Safety
///
/// `data[..len]` must be initialized and valid for reads and writes, and
/// `scratch` must have been prepared for `classifier`'s bucket count and
/// `len` elements, with empty buffers.
pub(super) unsafe fn classify<T, F>(
    data: *mut T,
    len: usize,
    classifier: &Classifier<T>,
    compare: &mut F,
    scratch: &mut Scratch<T>,
) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let block_size = scratch.block_size;
    let buffers = scratch.buffers_ptr();
    let mut state = Guard {
        data,
        buffers,
        block_size,
        fill: &mut scratch.buffer_fill,
        write: 0,
        read: 0,
    };
    let bucket_sizes = &mut scratch.bucket_sizes;
    let block_ids = &mut scratch.block_ids;
    let mut is_less = is_less(compare);

    while state.read < len {
        // SAFETY: `read < len` and the element at `read` has not moved yet.
        let x = unsafe { &*data.add(state.read) };
        // The only call into user code. Nothing has moved for this element
        // yet, so if it panics the guard's view of the state is exact.
        let bucket = classifier.classify(x, &mut is_less);

        // From here to the end of the iteration nothing can panic once
        // elements start moving: every index is checked before the first copy.
        bucket_sizes[bucket] += 1;
        // SAFETY: `bucket < num_buckets`, so this is inside the buffers.
        let buffer = unsafe { buffers.add(bucket * block_size) };
        if state.fill[bucket] == block_size {
            block_ids[state.write / block_size] = bucket as u16;
            // SAFETY: the holes `[write, read)` number as many as the
            // buffered elements, of which this buffer alone has
            // `block_size`. So the block fits, and the buffer's elements
            // move into uninitialized slots.
            unsafe { ptr::copy_nonoverlapping(buffer, data.add(state.write), block_size) };
            state.write += block_size;
            state.fill[bucket] = 0;
        }
        let fill = state.fill[bucket];
        // SAFETY: the buffer has room (`fill < block_size`); the element
        // moves out of `data`, and `read += 1` turns its slot into a hole.
        unsafe { ptr::copy_nonoverlapping(data.add(state.read), buffer.add(fill), 1) };
        state.fill[bucket] = fill + 1;
        state.read += 1;
    }

    let written = state.write;
    // Success: the buffers stay full for the next phases.
    mem::forget(state);
    written
}

/// The live state of local classification. Dropping it (only on unwind)
/// empties every buffer into the holes `[write, read)`.
struct Guard<'a, T> {
    data: *mut T,
    buffers: *mut T,
    block_size: usize,
    fill: &'a mut [usize],
    write: usize,
    read: usize,
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        for (bucket, fill) in self.fill.iter_mut().enumerate() {
            // SAFETY: the holes `[write, read)` number exactly as many as the
            // buffered elements, so all buffers fit back in.
            unsafe {
                ptr::copy_nonoverlapping(
                    self.buffers.add(bucket * self.block_size),
                    self.data.add(self.write),
                    *fill,
                );
            }
            self.write += *fill;
            *fill = 0;
        }
    }
}
