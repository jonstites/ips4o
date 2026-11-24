//! Phase 3: block permutation.

use std::ptr;

use super::Scratch;

/// Where block permutation left each bucket.
pub(super) struct Placed {
    /// For each bucket, one past the last block (by block index) placed in
    /// its area.
    pub(super) write: Vec<usize>,
    /// The bucket whose last block went to the overflow buffer, if any.
    pub(super) overflow_bucket: Option<usize>,
}

/// Moves every full block into its bucket's *area*: the blocks from
/// `ceil(start / B)` to `ceil(end / B)`, where `[start, end)` is the bucket's
/// final range and `B` the block size. An area can hold at least
/// `floor(size / B)` blocks, which is at least the bucket's block count (see
/// `local::classify`), so blocks never run out of room.
///
/// Each bucket keeps two block pointers into its area:
/// - `write`: blocks before it are placed;
/// - `read`: blocks in `[write, read)` are unprocessed (some bucket's block,
///   with id in `block_ids`), and slots in `[read, area end)` are empty.
///
/// For each bucket in turn, we take its last unprocessed block into a swap
/// buffer and carry it to its destination bucket. If the destination's next
/// slot holds a block of some other bucket, we swap it out and carry that
/// one next. If the slot is empty, we drop the block there and the chain
/// ends. Every block moves at most twice.
///
/// One slot can extend past `len`: the last block of the last area, when
/// `len` is not a multiple of `B`. A block headed there goes to the overflow
/// buffer instead, and cleanup sorts it out.
///
/// # Safety
///
/// The state that `local::classify` returns, for the same `data`, `len` and
/// `scratch`, with `written` its return value and `bucket_starts` the prefix
/// sums of `scratch.bucket_sizes`.
pub(super) unsafe fn permute_blocks<T>(
    data: *mut T,
    len: usize,
    written: usize,
    bucket_starts: &[usize],
    scratch: &mut Scratch<T>,
) -> Placed {
    let bs = scratch.block_size;
    let num_buckets = bucket_starts.len() - 1;
    let filled = written / bs;
    let area_start = |b: usize| bucket_starts[b].div_ceil(bs);

    let mut write: Vec<usize> = (0..num_buckets).map(area_start).collect();
    let mut read: Vec<usize> = (0..num_buckets)
        .map(|b| filled.clamp(area_start(b), area_start(b + 1)))
        .collect();

    let swap = scratch.swap_ptr();
    let overflow = scratch.overflow_ptr();
    let ids = &scratch.block_ids;
    // Pointer arithmetic only; each copy below justifies its own bounds.
    let block = |slot: usize| data.wrapping_add(slot * bs);
    let swap_block = |which: usize| swap.wrapping_add(which * bs);
    let mut overflow_bucket = None;

    for primary in 0..num_buckets {
        while write[primary] < read[primary] {
            read[primary] -= 1;
            let slot = read[primary];
            let mut current = 0;
            // SAFETY: `slot` is unprocessed, so it holds a full block inside
            // `data[..written]`; it becomes empty (`read` moved past it).
            unsafe { ptr::copy_nonoverlapping(block(slot), swap_block(current), bs) };
            let mut dest = ids[slot] as usize;

            loop {
                // Skip blocks that are already in the right bucket.
                while write[dest] < read[dest] && ids[write[dest]] as usize == dest {
                    write[dest] += 1;
                }
                let slot = write[dest];
                write[dest] += 1;

                if slot < read[dest] {
                    // An unprocessed block of another bucket: swap it out.
                    let other = 1 - current;
                    // SAFETY: `slot` holds a full block in `data[..written]`.
                    // It moves to the free swap block, and the carried block
                    // takes its place.
                    unsafe {
                        ptr::copy_nonoverlapping(block(slot), swap_block(other), bs);
                        ptr::copy_nonoverlapping(swap_block(current), block(slot), bs);
                    }
                    dest = ids[slot] as usize;
                    current = other;
                } else {
                    // An empty slot. It lies in `dest`'s area because no
                    // bucket has more blocks than its area holds; checked
                    // anyway, since a violation would write out of bounds.
                    assert!(
                        slot < area_start(dest + 1),
                        "bucket {dest} overflowed its area"
                    );
                    if (slot + 1) * bs > len {
                        assert!(overflow_bucket.is_none(), "second overflow block");
                        // SAFETY: the overflow buffer holds one block and is empty.
                        unsafe { ptr::copy_nonoverlapping(swap_block(current), overflow, bs) };
                        overflow_bucket = Some(dest);
                    } else {
                        // SAFETY: `slot` is empty and fits within `data[..len]`.
                        unsafe { ptr::copy_nonoverlapping(swap_block(current), block(slot), bs) };
                    }
                    break;
                }
            }
        }
    }

    Placed {
        write,
        overflow_bucket,
    }
}
