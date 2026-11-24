//! Phase 4: parallel block permutation.
//!
//! The same algorithm as `inplace::permute`, run by several tasks at once,
//! each starting at a different bucket. Each bucket's two block pointers
//! live in one `AtomicU64`, written pointer in the high half and read
//! pointer in the low half:
//!
//! - blocks below `write` are placed;
//! - blocks in `[write, read)` are unprocessed (some bucket's full block);
//! - slots from `read` to the end of the area are empty.
//!
//! A task *reads* a block by atomically decrementing `read` (if
//! `write < read`), and *writes* by atomically incrementing `write`. The
//! returned position is then exclusively the task's:
//!
//! - an unprocessed slot below the old `read`: no reader can claim it any
//!   more, because readers only take `read - 1 >= write`;
//! - an empty slot at or above `read`: but a reader may *still be copying*
//!   the block that was there. Each bucket counts the readers in the middle
//!   of a copy, incremented before `read` moves and decremented after the
//!   copy; a writer that lands on an empty slot waits until that count is
//!   zero. The reader it waits for is already copying, so it finishes
//!   without waiting on anything.
//!
//! All atomics use `SeqCst`. The counts are per bucket per partitioning
//! step, so contention is low and the cost doesn't matter.
//!
//! Block ids are only read here, never written: a slot's id is read while
//! the slot is unprocessed, and placed slots are never read again.

use std::hint;
use std::ptr;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

use super::SendPtr;
use crate::inplace::uninit_vec;

/// Where block permutation left each bucket (block indices).
pub(super) struct Placed {
    /// One past the last block placed in each bucket's area.
    pub(super) write: Vec<usize>,
    /// The bucket whose last block went to the overflow buffer, if any.
    pub(super) overflow_bucket: Option<usize>,
}

struct BucketPointers {
    /// `write << 32 | read`.
    packed: AtomicU64,
    /// Readers in the middle of copying a block out of this bucket.
    readers: AtomicUsize,
}

impl BucketPointers {
    /// Claims the last unprocessed block, if any, and registers as a reader.
    /// Call [`BucketPointers::done_reading`] after copying it.
    fn claim_read(&self) -> Option<usize> {
        self.readers.fetch_add(1, SeqCst);
        let mut current = self.packed.load(SeqCst);
        loop {
            let (write, read) = (current >> 32, current & 0xFFFF_FFFF);
            if write >= read {
                self.readers.fetch_sub(1, SeqCst);
                return None;
            }
            match self
                .packed
                .compare_exchange_weak(current, current - 1, SeqCst, SeqCst)
            {
                Ok(_) => return Some(read as usize - 1),
                Err(actual) => current = actual,
            }
        }
    }

    fn done_reading(&self) {
        self.readers.fetch_sub(1, SeqCst);
    }

    /// Claims the next write slot, returning it and whether it held an
    /// unprocessed block (rather than being empty).
    fn claim_write(&self) -> (usize, bool) {
        let old = self.packed.fetch_add(1 << 32, SeqCst);
        let (write, read) = (old >> 32, old & 0xFFFF_FFFF);
        (write as usize, write < read)
    }

    /// Waits until no reader is copying out of this bucket.
    fn wait_for_readers(&self) {
        let mut spins = 0u32;
        while self.readers.load(SeqCst) != 0 {
            if spins < 64 {
                hint::spin_loop();
                spins += 1;
            } else {
                std::thread::yield_now();
            }
        }
    }
}

/// Moves every full block into its bucket's area, with `tasks` tasks.
///
/// # Safety
///
/// For each bucket `b`, its area (blocks `ceil(start_b / B)` to
/// `ceil(end_b / B)`) holds full blocks from its start up to `filled[b]`,
/// with buckets in `ids`, and is empty after. No bucket has more full blocks
/// in total than its area holds. `overflow` has room for one block.
#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn permute_blocks<T: Send>(
    data: SendPtr<T>,
    len: usize,
    bs: usize,
    bucket_starts: &[usize],
    filled: &[usize],
    ids: &[u16],
    overflow: SendPtr<T>,
    tasks: usize,
) -> Placed {
    let num_buckets = bucket_starts.len() - 1;
    let area_start = |b: usize| bucket_starts[b].div_ceil(bs);
    let pointers: Vec<BucketPointers> = (0..num_buckets)
        .map(|b| BucketPointers {
            packed: AtomicU64::new(((area_start(b) as u64) << 32) | filled[b] as u64),
            readers: AtomicUsize::new(0),
        })
        .collect();
    let overflow_bucket = AtomicUsize::new(usize::MAX);

    rayon::scope(|s| {
        for task in 0..tasks {
            let pointers = &pointers;
            let overflow_bucket = &overflow_bucket;
            s.spawn(move |_| {
                let mut swap = uninit_vec::<T>(2 * bs);
                let swap = swap.as_mut_ptr().cast::<T>();
                // Pointer arithmetic only; each copy justifies its bounds.
                let block = |slot: usize| data.get().wrapping_add(slot * bs);
                let swap_block = |which: usize| swap.wrapping_add(which * bs);

                let first = task * num_buckets / tasks;
                for primary in (first..num_buckets).chain(0..first) {
                    while let Some(slot) = pointers[primary].claim_read() {
                        let mut current = 0;
                        // SAFETY: we claimed `slot`, an unprocessed full
                        // block; no one else reads or writes it until we
                        // call `done_reading`.
                        unsafe { ptr::copy_nonoverlapping(block(slot), swap_block(current), bs) };
                        let mut dest = ids[slot] as usize;
                        pointers[primary].done_reading();

                        loop {
                            let (slot, unprocessed) = pointers[dest].claim_write();
                            if unprocessed {
                                if ids[slot] as usize == dest {
                                    // Already in the right bucket.
                                    continue;
                                }
                                let other = 1 - current;
                                // SAFETY: the slot is ours (see module docs):
                                // swap its block out, ours in.
                                unsafe {
                                    ptr::copy_nonoverlapping(block(slot), swap_block(other), bs);
                                    ptr::copy_nonoverlapping(swap_block(current), block(slot), bs);
                                }
                                dest = ids[slot] as usize;
                                current = other;
                            } else {
                                // An empty slot. It's in `dest`'s area because
                                // no bucket has more blocks than its area
                                // holds; checked anyway, since a violation
                                // would write out of bounds.
                                assert!(
                                    slot < area_start(dest + 1),
                                    "bucket {dest} overflowed its area"
                                );
                                pointers[dest].wait_for_readers();
                                if (slot + 1) * bs > len {
                                    let previous = overflow_bucket.compare_exchange(
                                        usize::MAX,
                                        dest,
                                        SeqCst,
                                        SeqCst,
                                    );
                                    assert!(previous.is_ok(), "second overflow block");
                                    // SAFETY: only one block overflows, so
                                    // the overflow buffer is ours and empty.
                                    unsafe {
                                        ptr::copy_nonoverlapping(
                                            swap_block(current),
                                            overflow.get(),
                                            bs,
                                        )
                                    };
                                } else {
                                    // SAFETY: the slot is empty, ours, and
                                    // no one is copying out of it any more.
                                    unsafe {
                                        ptr::copy_nonoverlapping(
                                            swap_block(current),
                                            block(slot),
                                            bs,
                                        )
                                    };
                                }
                                break;
                            }
                        }
                    }
                }
            });
        }
    });

    let overflow_bucket = overflow_bucket.into_inner();
    Placed {
        write: pointers
            .iter()
            .map(|p| (p.packed.load(SeqCst) >> 32) as usize)
            .collect(),
        overflow_bucket: (overflow_bucket != usize::MAX).then_some(overflow_bucket),
    }
}
