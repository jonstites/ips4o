//! The parallel sort, on rayon.
//!
//! A parallel partitioning step ([`partition`]) runs the sequential phases
//! with the work split across `t` *stripes* (block-aligned slices of the
//! input) and rayon tasks:
//!
//! 1. **Splitters**, sequentially, as in the sequential sort.
//! 2. **Local classification**, one task per stripe, each with its own
//!    buffers (`inplace::local::classify` on the stripe).
//! 3. **Bookkeeping**, sequentially: bucket sizes, and within each bucket's
//!    area, full blocks moved to the front over the gaps that stripes left
//!    (see [`compact_areas`]).
//! 4. **Block permutation**, `t` tasks sharing atomic bucket pointers
//!    ([`permute`]).
//! 5. **Cleanup**, parallel over buckets ([`cleanup`]).
//! 6. **Splitter reinsertion**, sequentially.
//!
//! Then the buckets are sorted in parallel: big ones with another parallel
//! partitioning step, small ones with the sequential sort.
//!
//! Each phase is its own `rayon::scope`, and the end of the scope is the
//! barrier. No task ever waits for a task that hasn't started; the one wait
//! inside a phase (in block permutation) is on a copy already in progress.
//!
//! # Panic safety
//!
//! As in the sequential sort, only phases 1 and 2 call the comparator. In
//! phase 2, a stripe whose task panics restores itself (its own guard).
//! `rayon::scope` runs every spawned task to completion before passing the
//! panic on; we catch it, restore the stripes that finished
//! ([`restore_stripe`]), and resume unwinding, which lets the splitter guard
//! put the splitters back. Phases 3-6 abort on unwind.

mod cleanup;
mod permute;

use std::cmp::Ordering;
use std::mem::{self, ManuallyDrop};
use std::panic::{self, AssertUnwindSafe};
use std::ptr;
use std::sync::Mutex;

use rayon::prelude::*;

use crate::classifier::{Classifier, select_splitters};
use crate::config::{Config, base_case_size, block_size};
use crate::inplace::cleanup::reinsert_splitters;
use crate::inplace::{self, Partition, Scratch, SplitterGuard, local};
use crate::util::{AbortOnUnwind, Rng};

/// A raw pointer that may cross threads. Every use documents why the
/// accesses from different threads don't conflict.
pub(crate) struct SendPtr<T>(pub(crate) *mut T);

impl<T> SendPtr<T> {
    /// Use this rather than `.0` in closures: a closure that mentions `.0`
    /// captures the raw pointer itself, which isn't `Send`.
    pub(crate) fn get(&self) -> *mut T {
        self.0
    }
}

impl<T> Clone for SendPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for SendPtr<T> {}
// SAFETY: the pointer only moves `T`s between threads (needs `T: Send`);
// each use site keeps the accesses disjoint or synchronized.
unsafe impl<T: Send> Send for SendPtr<T> {}
// SAFETY: as above.
unsafe impl<T: Send> Sync for SendPtr<T> {}

/// Scratch space shared by all tasks of one parallel sort. A stripe or a
/// sequentially sorted bucket takes a `Scratch` and gives it back when done,
/// so buffers are allocated once per sort, not once per partitioning step
/// (which cost tens of thousands of page faults per large sort).
pub(crate) struct ScratchPool<T> {
    block_size: usize,
    free: Mutex<Vec<Scratch<T>>>,
}

impl<T> ScratchPool<T> {
    pub(crate) fn new(block_size: usize) -> Self {
        ScratchPool {
            block_size,
            free: Mutex::new(Vec::new()),
        }
    }

    fn take(&self) -> Scratch<T> {
        let free = self.free.lock().unwrap_or_else(|e| e.into_inner()).pop();
        free.unwrap_or_else(|| Scratch::new(self.block_size))
    }

    /// Returns `scratch`, whose buffers must be empty.
    fn give(&self, scratch: Scratch<T>) {
        self.free
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(scratch);
    }
}

/// Sorts `v` in parallel on the current rayon pool.
pub(crate) fn sort<T, F, C>(v: &mut [T], compare: &F, seed: u64)
where
    T: Send + Sync,
    F: Fn(&T, &T) -> Ordering + Sync,
    C: Config,
{
    if mem::size_of::<T>() == 0 || v.len() < 2 {
        return;
    }
    if inplace::sort_if_presorted(v, &mut |a, b| compare(a, b)) {
        return;
    }
    let depth_limit = 2 * v.len().ilog2();
    let pool = ScratchPool::new(block_size::<T, C>());
    recurse::<T, F, C>(v, compare, seed, depth_limit, &pool);
}

/// Number of stripes to partition `len` elements with: one per thread, but
/// at least `PAR_MIN_STRIPE` elements each. Below 2, sort sequentially.
fn stripes_for<C: Config>(len: usize) -> usize {
    rayon::current_num_threads().min(len / C::PAR_MIN_STRIPE)
}

fn recurse<T, F, C>(v: &mut [T], compare: &F, seed: u64, depth_limit: u32, pool: &ScratchPool<T>)
where
    T: Send + Sync,
    F: Fn(&T, &T) -> Ordering + Sync,
    C: Config,
{
    let stripes = stripes_for::<C>(v.len());
    if stripes < 2 || depth_limit == 0 || v.len() <= base_case_size::<T, C>() {
        let mut scratch = pool.take();
        inplace::sort_unchecked_with::<T, _, C>(
            v,
            &mut |a, b| compare(a, b),
            seed,
            depth_limit,
            &mut scratch,
        );
        pool.give(scratch);
        return;
    }

    let mut rng = Rng::new(seed);
    let Partition {
        layout,
        bucket_starts,
    } = partition::<T, F, C>(v, compare, &mut rng, stripes, pool);

    // Cut `v` into the buckets that still need sorting. Splitters sit
    // between buckets, so bucket `b` is shifted right by the number of
    // splitters before it.
    let mut buckets = Vec::new();
    let mut rest = v;
    let mut consumed = 0;
    let mut shift = 0;
    for b in 0..layout.num_buckets() {
        let start = bucket_starts[b] + shift;
        let end = bucket_starts[b + 1] + shift;
        let (_, tail) = mem::take(&mut rest).split_at_mut(start - consumed);
        let (bucket, tail) = tail.split_at_mut(end - start);
        rest = tail;
        consumed = end;
        if !layout.is_equality_bucket(b) && bucket.len() > 1 {
            buckets.push((bucket, rng.next_u64()));
        }
        if layout.splitter_after(b).is_some() {
            shift += 1;
        }
    }

    buckets.into_par_iter().for_each(|(bucket, seed)| {
        recurse::<T, F, C>(bucket, compare, seed, depth_limit - 1, pool)
    });
}

/// One parallel partitioning step over at most `stripes` stripes (fewer if
/// the input has fewer blocks). Same result layout as `inplace::partition`; with the same `rng`, it picks
/// the same splitters and produces the same buckets.
pub(crate) fn partition<T, F, C>(
    v: &mut [T],
    compare: &F,
    rng: &mut Rng,
    stripes: usize,
    pool: &ScratchPool<T>,
) -> Partition
where
    T: Send + Sync,
    F: Fn(&T, &T) -> Ordering + Sync,
    C: Config,
{
    // Phase 1: splitters (only swaps, so panics are harmless).
    let layout = select_splitters::<T, _, C>(v, &mut |a, b| compare(a, b), rng);
    let m = layout.num_splitters();
    let n = v.len();
    let num_buckets = layout.num_buckets();
    let bs = block_size::<T, C>();
    // From here on, all access goes through `base`, until we return.
    let base = v.as_mut_ptr();

    let bomb = AbortOnUnwind;
    let mut sorted = Vec::with_capacity(m);
    // SAFETY: as in `inplace::partition`: the splitters move into `sorted`,
    // and `v[..m]` is uninitialized until reinsertion or the guard.
    unsafe {
        ptr::copy_nonoverlapping(base, sorted.as_mut_ptr(), m);
        sorted.set_len(m);
    }
    let splitters = SplitterGuard {
        classifier: ManuallyDrop::new(Classifier::new(sorted, layout.has_equal_buckets())),
        slots: base,
    };
    bomb.defuse();

    // SAFETY: `m <= n`.
    let data = SendPtr(unsafe { base.add(m) });
    let len = n - m;
    // Block-aligned stripes; the last one takes the remainder.
    let stripe_len = len.div_ceil(stripes).div_ceil(bs) * bs;
    let stripes = len.div_ceil(stripe_len);
    assert!(
        len / bs < u32::MAX as usize,
        "too many blocks for 32-bit bucket pointers"
    );

    // Phase 2: classify the stripes in parallel.
    let mut scratches: Vec<Scratch<T>> = (0..stripes).map(|_| pool.take()).collect();
    let mut written = vec![0; stripes];
    let mut done = vec![false; stripes];
    let classifier: &Classifier<T> = &splitters.classifier;
    let classified = panic::catch_unwind(AssertUnwindSafe(|| {
        rayon::scope(|s| {
            let jobs = scratches.iter_mut().zip(&mut written).zip(&mut done);
            for (i, ((scratch, written), done)) in jobs.enumerate() {
                let start = i * stripe_len;
                let stripe = stripe_len.min(len - start);
                s.spawn(move |_| {
                    scratch.prepare(num_buckets, stripe);
                    // SAFETY: stripes are disjoint parts of `data`, each
                    // initialized; this task is the only one touching
                    // stripe `i` and `scratch`.
                    *written = unsafe {
                        local::classify(
                            data.get().add(start),
                            stripe,
                            classifier,
                            &mut |a, b| compare(a, b),
                            scratch,
                        )
                    };
                    *done = true;
                });
            }
        })
    }));
    if let Err(payload) = classified {
        // `rayon::scope` has run every task to completion, so nothing else
        // touches the stripes now.
        // SAFETY: the state each finished stripe is in (see `restore_stripe`).
        for i in (0..stripes).filter(|&i| done[i]) {
            unsafe {
                restore_stripe(
                    data.get().add(i * stripe_len),
                    written[i],
                    &mut scratches[i],
                )
            };
        }
        // The splitter guard puts the splitters back while unwinding.
        panic::resume_unwind(payload);
    }

    // Phases 3-6 never call the comparator and must not unwind.
    let bomb = AbortOnUnwind;

    // Phase 3: bucket sizes, the global block ids, and compacted areas.
    let mut bucket_starts = vec![0; num_buckets + 1];
    for scratch in &scratches {
        for (b, &size) in scratch.bucket_sizes.iter().enumerate() {
            bucket_starts[b + 1] += size;
        }
    }
    for b in 0..num_buckets {
        bucket_starts[b + 1] += bucket_starts[b];
    }
    assert_eq!(bucket_starts[num_buckets], len);

    let mut ids = vec![0u16; len / bs];
    for (i, scratch) in scratches.iter().enumerate() {
        let first = i * stripe_len / bs;
        let count = written[i] / bs;
        ids[first..first + count].copy_from_slice(&scratch.block_ids[..count]);
    }
    let full = |block: usize| {
        let stripe = block * bs / stripe_len;
        block * bs < stripe * stripe_len + written[stripe]
    };
    // SAFETY: the state `local::classify` leaves in each stripe.
    let filled = unsafe { compact_areas(data.get(), len, bs, &bucket_starts, &mut ids, full) };

    // Phase 4: block permutation.
    let mut overflow = inplace::uninit_vec::<T>(bs);
    // SAFETY: areas hold their full blocks at the front (`filled`), ids
    // describe them, and the rest of each area is empty.
    let placed = unsafe {
        permute::permute_blocks(
            data,
            len,
            bs,
            &bucket_starts,
            &filled,
            &ids,
            SendPtr(overflow.as_mut_ptr().cast()),
            stripes,
        )
    };

    // Phase 5: cleanup.
    let buffers: Vec<SendPtr<T>> = scratches
        .iter_mut()
        .map(|s| SendPtr(s.buffers_ptr()))
        .collect();
    let fills: Vec<&[usize]> = scratches.iter().map(|s| &s.buffer_fill[..]).collect();
    // SAFETY: the state `permute_blocks` returns, with every stripe's buffers.
    unsafe {
        cleanup::cleanup(
            data,
            bs,
            &bucket_starts,
            &placed,
            SendPtr(overflow.as_mut_ptr().cast()),
            &buffers,
            &fills,
        )
    };

    // Phase 6: splitters back between the buckets.
    let classifier = splitters.into_inner();
    // SAFETY: `data` is fully partitioned and `v[..m]` is the splitters' hole.
    unsafe {
        reinsert_splitters(
            base,
            m,
            &bucket_starts,
            classifier.into_sorted_splitters(),
            layout,
        )
    };
    bomb.defuse();

    // The buffers are empty again; keep them for the next step.
    for scratch in scratches {
        pool.give(scratch);
    }
    Partition {
        layout,
        bucket_starts,
    }
}

/// Within each bucket's area, moves the full blocks to the front, so that
/// each area is `[full blocks | empty]` as block permutation expects. After
/// local classification, full blocks only sit at the start of each stripe,
/// so an area spanning a stripe boundary has a gap in the middle. Returns,
/// for each bucket, the end of its full blocks (a block index).
///
/// `full(block)` says whether a block position holds a full block.
///
/// # Safety
///
/// `data[..len]` as local classification leaves it in every stripe, with
/// `ids` the bucket of each full block (by global block position).
unsafe fn compact_areas<T>(
    data: *mut T,
    len: usize,
    bs: usize,
    bucket_starts: &[usize],
    ids: &mut [u16],
    full: impl Fn(usize) -> bool,
) -> Vec<usize> {
    let num_buckets = bucket_starts.len() - 1;
    let blocks = len / bs;
    (0..num_buckets)
        .map(|b| {
            let lo = bucket_starts[b].div_ceil(bs);
            let hi = bucket_starts[b + 1].div_ceil(bs).min(blocks);
            let mut write = lo;
            for block in lo..hi {
                if full(block) {
                    if block != write {
                        // SAFETY: `block` is full, `write < block` is empty
                        // (everything full before `block` is already packed
                        // below `write`); both are whole blocks in `data`.
                        unsafe {
                            ptr::copy_nonoverlapping(data.add(block * bs), data.add(write * bs), bs)
                        };
                        ids[write] = ids[block];
                    }
                    write += 1;
                }
            }
            write
        })
        .collect()
}

/// Undoes local classification for a stripe that finished it: its holes
/// start at `written`, and its buffers hold exactly enough elements to fill
/// them. (A stripe whose own task panicked restored itself already, and an
/// unstarted stripe was never touched.)
///
/// # Safety
///
/// `stripe` and `scratch` must be in the state `local::classify` returns.
unsafe fn restore_stripe<T>(stripe: *mut T, written: usize, scratch: &mut Scratch<T>) {
    let bs = scratch.block_size;
    let buffers = scratch.buffers_ptr();
    let mut hole = written;
    for (bucket, fill) in scratch.buffer_fill.iter_mut().enumerate() {
        // SAFETY: the holes number exactly as many as the buffered elements.
        unsafe { ptr::copy_nonoverlapping(buffers.add(bucket * bs), stripe.add(hole), *fill) };
        hole += *fill;
        *fill = 0;
    }
}
