//! The in-place sequential sort.
//!
//! One partitioning step ([`partition`]) has five phases:
//!
//! 1. **Splitters.** Sample, sort the sample, gather the `m = 2^L - 1`
//!    splitters in `v[..m]`, and move them into the classifier. `v[..m]` is
//!    now logically uninitialized; the step partitions `data = v[m..]`.
//! 2. **Local classification** ([`local`]). Stream through `data`, putting
//!    each element into its bucket's buffer. Full buffers are written back to
//!    the front of `data` as blocks, and their bucket ids are recorded.
//! 3. **Block permutation** ([`permute`]). Move every full block into the
//!    block-aligned area of its bucket.
//! 4. **Cleanup** ([`cleanup`]). Fix the unaligned edges of each bucket and
//!    empty the buffers into the remaining gaps.
//! 5. **Splitter reinsertion** ([`cleanup::reinsert_splitters`]). Close the
//!    `m` holes at the front, putting each splitter after its bucket.
//!
//! # Safety model
//!
//! Every element exists exactly once at all times: in the slice, a bucket
//! buffer, a swap or overflow block, or the classifier. Only phases 1 and 2
//! call the comparator:
//!
//! - Phase 1 only swaps elements, so the slice stays a permutation.
//! - In phase 2, guards restore the slice if the comparator panics: buffer
//!   contents go back into the holes, and the splitters back to `v[..m]`.
//!
//! Phases 3–5 only copy memory. Because bucket ids were recorded in phase 2,
//! they never consult the comparator, and the counting arguments that keep
//! them in bounds hold whatever it returned. They run under
//! [`AbortOnUnwind`], so an internal bug aborts the process instead of
//! unwinding past half-moved data.

mod cleanup;
mod local;
mod permute;
mod scratch;

use std::cmp::Ordering;
use std::mem::{self, ManuallyDrop};
use std::ptr;

use crate::classifier::{BucketLayout, Classifier, select_splitters};
use crate::config::{Config, block_size};
use crate::util::{AbortOnUnwind, Rng, base_case_sort, is_less};

pub(crate) use scratch::Scratch;

/// Sorts `v` in place.
pub(crate) fn sort<T, F, C>(v: &mut [T], compare: &mut F, seed: u64)
where
    F: FnMut(&T, &T) -> Ordering,
    C: Config,
{
    // Matches std: sorting zero-sized values never calls the comparator.
    if mem::size_of::<T>() == 0 || v.len() < 2 {
        return;
    }
    if sort_if_presorted(v, compare) {
        return;
    }
    let mut rng = Rng::new(seed);
    let mut scratch = Scratch::new(block_size::<T, C>());
    // See `reference::sort_with_config` for the reasoning.
    let depth_limit = 2 * v.len().ilog2();
    recurse::<T, F, C>(v, compare, &mut rng, &mut scratch, depth_limit);
}

/// Handles input that is already sorted, or sorted in reverse, in O(n).
///
/// The first and last elements decide which order to check for, and the
/// scan stops at the first pair out of that order, so unsorted input costs
/// only a few comparisons. A non-increasing input is reversed, which is fine
/// even with equal elements because the sort is unstable. Returns whether
/// `v` is now sorted. Panic-safe: only compares, and reverses with swaps.
fn sort_if_presorted<T, F>(v: &mut [T], compare: &mut F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut is_less = is_less(compare);
    let n = v.len();
    if is_less(&v[n - 1], &v[0]) {
        if v.windows(2).all(|w| !is_less(&w[0], &w[1])) {
            v.reverse();
            return true;
        }
    } else if v.windows(2).all(|w| !is_less(&w[1], &w[0])) {
        return true;
    }
    false
}

fn recurse<T, F, C>(
    v: &mut [T],
    compare: &mut F,
    rng: &mut Rng,
    scratch: &mut Scratch<T>,
    depth_limit: u32,
) where
    F: FnMut(&T, &T) -> Ordering,
    C: Config,
{
    if v.len() <= C::BASE_CASE_SIZE || depth_limit == 0 {
        base_case_sort(v, compare);
        return;
    }
    let Partition {
        layout,
        bucket_starts,
    } = partition::<T, F, C>(v, compare, rng, scratch);

    // Splitters now sit between buckets, so bucket `b` is shifted right by
    // the number of splitters before it.
    let mut shift = 0;
    for b in 0..layout.num_buckets() {
        if !layout.is_equality_bucket(b) {
            let bucket = &mut v[bucket_starts[b] + shift..bucket_starts[b + 1] + shift];
            recurse::<T, F, C>(bucket, compare, rng, scratch, depth_limit - 1);
        }
        if layout.splitter_after(b).is_some() {
            shift += 1;
        }
    }
}

/// The result of one partitioning step.
pub(crate) struct Partition {
    pub(crate) layout: BucketLayout,
    /// Prefix sums of the bucket sizes (`num_buckets + 1` entries). Bucket
    /// `b` ends up at `bucket_starts[b]` plus the number of splitters placed
    /// before it.
    pub(crate) bucket_starts: Vec<usize>,
}

/// Partitions `v` into buckets with splitters in between. Requires
/// `v.len() > C::BASE_CASE_SIZE`.
pub(crate) fn partition<T, F, C>(
    v: &mut [T],
    compare: &mut F,
    rng: &mut Rng,
    scratch: &mut Scratch<T>,
) -> Partition
where
    F: FnMut(&T, &T) -> Ordering,
    C: Config,
{
    // Phase 1: choose splitters (only swaps, so panics are harmless).
    let layout = select_splitters::<T, F, C>(v, compare, rng);
    let m = layout.num_splitters();
    let n = v.len();
    // From here on, all access goes through `base`, until we return.
    let base = v.as_mut_ptr();

    // Move the splitters out. Nothing here calls user code; the bomb turns
    // an internal panic into an abort before the guard exists.
    let bomb = AbortOnUnwind;
    let mut sorted = Vec::with_capacity(m);
    // SAFETY: `v[..m]` holds `m` initialized elements, and `sorted` has room
    // for `m`. Ownership moves to `sorted`; `v[..m]` counts as uninitialized
    // until the splitters are put back, by reinsertion or by the guard.
    unsafe {
        ptr::copy_nonoverlapping(base, sorted.as_mut_ptr(), m);
        sorted.set_len(m);
    }
    let splitters = SplitterGuard {
        classifier: ManuallyDrop::new(Classifier::new(sorted, layout.has_equal_buckets())),
        slots: base,
    };
    bomb.defuse();

    // Phase 2: local classification (calls the comparator; has its own guard).
    // SAFETY: `m <= n`, so `data[..len]` is the initialized rest of `v`.
    let data = unsafe { base.add(m) };
    let len = n - m;
    scratch.prepare(layout.num_buckets(), len);
    // SAFETY: as above; `scratch` was just prepared and its buffers are empty.
    let written = unsafe { local::classify(data, len, &splitters.classifier, compare, scratch) };

    // Phases 3-5 never call the comparator and must not unwind.
    let bomb = AbortOnUnwind;
    let mut bucket_starts = Vec::with_capacity(layout.num_buckets() + 1);
    bucket_starts.push(0);
    for &size in &scratch.bucket_sizes {
        bucket_starts.push(bucket_starts[bucket_starts.len() - 1] + size);
    }
    assert_eq!(bucket_starts[layout.num_buckets()], len);

    // SAFETY: `data`, `len`, `written` and `scratch` are in the state
    // `local::classify` documents, and `bucket_starts` are its bucket sizes'
    // prefix sums.
    let placed = unsafe { permute::permute_blocks(data, len, written, &bucket_starts, scratch) };
    // SAFETY: the state `permute_blocks` documents.
    unsafe { cleanup::cleanup(data, &bucket_starts, &placed, scratch) };
    let classifier = splitters.into_inner();
    // SAFETY: `data` is fully partitioned and `v[..m]` is the hole left by
    // the splitters.
    unsafe {
        cleanup::reinsert_splitters(
            base,
            m,
            &bucket_starts,
            classifier.into_sorted_splitters(),
            layout,
        )
    };
    bomb.defuse();

    Partition {
        layout,
        bucket_starts,
    }
}

/// Owns the splitters while they are out of the slice. If dropped (that is,
/// if the comparator panics during local classification), it writes them
/// back into the slots they came from.
struct SplitterGuard<T> {
    classifier: ManuallyDrop<Classifier<T>>,
    /// The `m` uninitialized slots at the front of the slice.
    slots: *mut T,
}

impl<T> SplitterGuard<T> {
    /// Disarms the guard, handing over the classifier.
    fn into_inner(self) -> Classifier<T> {
        let mut this = ManuallyDrop::new(self);
        // SAFETY: `this` is never dropped, so the classifier is taken once.
        unsafe { ManuallyDrop::take(&mut this.classifier) }
    }
}

impl<T> Drop for SplitterGuard<T> {
    fn drop(&mut self) {
        // SAFETY: drop runs once, and `into_inner` forgets `self`, so this is
        // the only take.
        let classifier = unsafe { ManuallyDrop::take(&mut self.classifier) };
        for (i, splitter) in classifier.into_sorted_splitters().into_iter().enumerate() {
            // SAFETY: `slots` has room for exactly the `m` splitters, and
            // those slots are uninitialized.
            unsafe { self.slots.add(i).write(splitter) };
        }
    }
}
