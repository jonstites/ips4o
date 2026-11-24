//! Working memory for the in-place sort, allocated once per sort call and
//! reused by every partitioning step.

use std::mem::MaybeUninit;

pub(crate) struct Scratch<T> {
    /// Elements per block.
    pub(super) block_size: usize,
    /// One block-sized buffer per bucket, back to back.
    buffers: Vec<MaybeUninit<T>>,
    /// Number of elements currently in each bucket's buffer.
    pub(super) buffer_fill: Vec<usize>,
    /// Total elements classified into each bucket.
    pub(super) bucket_sizes: Vec<usize>,
    /// Bucket of each full block written back during local classification,
    /// indexed by block position. Recording ids means the later phases never
    /// re-classify, so they never call the comparator.
    pub(super) block_ids: Vec<u16>,
    /// Two blocks for swapping during block permutation.
    swap: Vec<MaybeUninit<T>>,
    /// Holds the one block that may not fit at the end of the input.
    overflow: Vec<MaybeUninit<T>>,
}

impl<T> Scratch<T> {
    pub(crate) fn new(block_size: usize) -> Self {
        Scratch {
            block_size,
            buffers: Vec::new(),
            buffer_fill: Vec::new(),
            bucket_sizes: Vec::new(),
            block_ids: Vec::new(),
            swap: uninit_vec(2 * block_size),
            overflow: uninit_vec(block_size),
        }
    }

    /// Sizes everything for a step with `num_buckets` buckets over `len`
    /// elements. All buffers must be empty.
    pub(super) fn prepare(&mut self, num_buckets: usize, len: usize) {
        let need = num_buckets * self.block_size;
        if self.buffers.len() < need {
            self.buffers.resize_with(need, MaybeUninit::uninit);
        }
        self.buffer_fill.clear();
        self.buffer_fill.resize(num_buckets, 0);
        self.bucket_sizes.clear();
        self.bucket_sizes.resize(num_buckets, 0);
        self.block_ids.clear();
        self.block_ids.resize(len / self.block_size, 0);
    }

    /// Start of bucket `0`'s buffer; bucket `b`'s starts `b * block_size`
    /// elements later.
    pub(super) fn buffers_ptr(&mut self) -> *mut T {
        self.buffers.as_mut_ptr().cast()
    }

    pub(super) fn swap_ptr(&mut self) -> *mut T {
        self.swap.as_mut_ptr().cast()
    }

    pub(super) fn overflow_ptr(&mut self) -> *mut T {
        self.overflow.as_mut_ptr().cast()
    }
}

fn uninit_vec<T>(len: usize) -> Vec<MaybeUninit<T>> {
    let mut v = Vec::with_capacity(len);
    v.resize_with(len, MaybeUninit::uninit);
    v
}
