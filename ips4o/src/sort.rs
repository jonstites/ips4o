#![feature("unstable")]
#![feature(is_sorted)]

//use std::cmp;
use std::mem;
//use std::ptr;

extern crate rand;
use rand::distributions::{Distribution, Uniform};

extern crate rayon_core;
/*
/// When dropped, copies from `src` into `dest`.
struct CopyOnDrop<T> {
    src: *mut T,
    dest: *mut T,
}

impl<T> Drop for CopyOnDrop<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::copy_nonoverlapping(self.src, self.dest, 1);
        }
    }
}

/// Shifts the last element to the left until it encounters a smaller or equal element.
fn shift_tail<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    let len = v.len();
    unsafe {
        // If the last two elements are out-of-order...
        if len >= 2 && is_less(v.get_unchecked(len - 1), v.get_unchecked(len - 2)) {
            // Read the last element into a stack-allocated variable. If a following comparison
            // operation panics, `hole` will get dropped and automatically write the element back
            // into the slice.
            let mut tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(len - 1)));
            let mut hole = CopyOnDrop { src: &mut *tmp, dest: v.get_unchecked_mut(len - 2) };
            ptr::copy_nonoverlapping(v.get_unchecked(len - 2), v.get_unchecked_mut(len - 1), 1);

            for i in (0..len - 2).rev() {
                if !is_less(&*tmp, v.get_unchecked(i)) {
                    break;
                }

                // Move `i`-th element one place to the right, thus shifting the hole to the left.
                ptr::copy_nonoverlapping(v.get_unchecked(i), v.get_unchecked_mut(i + 1), 1);
                hole.dest = v.get_unchecked_mut(i);
            }
            // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        }
    }
}


/// Sorts a slice using insertion sort, which is `O(n^2)` worst-case.
fn insertion_sort<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    for i in 1..v.len() {
        shift_tail(&mut v[..i + 1], is_less);
    }
}

unsafe fn get_and_increment<T>(ptr: &mut *mut T) -> *mut T {
    let old = *ptr;
    *ptr = ptr.offset(1);
    old
}

fn sample<T>(v: &mut [T], num_samples: usize)
{
    // benchmark - maybe replace with XorShift?
    // TODO: notice the way break_patterns is implemented
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(0..v.len());

    for dest in 0..num_samples {
        let src = uniform.sample(&mut rng);
        v.swap(src, dest);
    }
}

fn build_classifier<T, F>(v: &mut [T], is_less: &mut F)
where F: FnMut(&T, &T) -> bool,
{
    let len = v.len();
    // TODO: This is a silly hack for prototyping.
    let num_buckets = 256_usize.min(len);
    
    let oversampling_factor: f64 = 1.0_f64.min(0.2 * (len >> 1) as f64);
    let step = 1_usize.max(oversampling_factor as usize);
    let num_samples = step * num_buckets - 1;
    sample(v, num_samples);
    
    // TODO: recurse(&mut v[0..num_samples], is_less);
    insertion_sort(&mut v[..num_samples], is_less);

    // Choose splitters (i.e. equally-spaced samples) to determine buckets
    let mut splitters = Vec::with_capacity(len / 2).as_mut_ptr();

    for splitter_index in ((step - 1)..num_samples).step_by(step) {
        unsafe {
            ptr::copy_nonoverlapping(v.as_mut_ptr(), splitters, 1);
            splitters = get_and_increment(&mut splitters);
        }
    }
    

}
*/

mod quicksort {
    //! Parallel quicksort.
    //!
    //! This implementation is copied verbatim from `std::slice::sort_unstable` and then parallelized.
    //! The only difference from the original is that calls to `recurse` are executed in parallel using
    //! `rayon_core::join`.

    use std::cmp;
    use std::mem;
    use std::ptr;

    /// When dropped, takes the value out of `Option` and writes it into `dest`.
    ///
    /// This allows us to safely read the pivot into a stack-allocated variable for efficiency, and
    /// write it back into the slice after partitioning. This way we ensure that the write happens
    /// even if `is_less` panics in the meantime.
    struct WriteOnDrop<T> {
        value: Option<T>,
        dest: *mut T,
    }

    impl<T> Drop for WriteOnDrop<T> {
        fn drop(&mut self) {
            unsafe {
                ptr::write(self.dest, self.value.take().unwrap());
            }
        }
    }

    /// Holds a value, but never drops it.
    struct NoDrop<T> {
        value: Option<T>,
    }

    impl<T> Drop for NoDrop<T> {
        fn drop(&mut self) {
            mem::forget(self.value.take());
        }
    }

    /// When dropped, copies from `src` into `dest`.
    struct CopyOnDrop<T> {
        src: *mut T,
        dest: *mut T,
    }

    impl<T> Drop for CopyOnDrop<T> {
        fn drop(&mut self) {
            unsafe {
                ptr::copy_nonoverlapping(self.src, self.dest, 1);
            }
        }
    }

    /// Shifts the first element to the right until it encounters a greater or equal element.
    fn shift_head<T, F>(v: &mut [T], is_less: &F)
    where
        F: Fn(&T, &T) -> bool,
    {
        let len = v.len();
        unsafe {
            // If the first two elements are out-of-order...
            if len >= 2 && is_less(v.get_unchecked(1), v.get_unchecked(0)) {
                // Read the first element into a stack-allocated variable. If a following comparison
                // operation panics, `hole` will get dropped and automatically write the element back
                // into the slice.
                let mut tmp = NoDrop {
                    value: Some(ptr::read(v.get_unchecked(0))),
                };
                let mut hole = CopyOnDrop {
                    src: tmp.value.as_mut().unwrap(),
                    dest: v.get_unchecked_mut(1),
                };
                ptr::copy_nonoverlapping(v.get_unchecked(1), v.get_unchecked_mut(0), 1);

                for i in 2..len {
                    if !is_less(v.get_unchecked(i), tmp.value.as_ref().unwrap()) {
                        break;
                    }

                    // Move `i`-th element one place to the left, thus shifting the hole to the right.
                    ptr::copy_nonoverlapping(v.get_unchecked(i), v.get_unchecked_mut(i - 1), 1);
                    hole.dest = v.get_unchecked_mut(i);
                }
                // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
            }
        }
    }

    /// Shifts the last element to the left until it encounters a smaller or equal element.
    fn shift_tail<T, F>(v: &mut [T], is_less: &F)
    where
        F: Fn(&T, &T) -> bool,
    {
        let len = v.len();
        unsafe {
            // If the last two elements are out-of-order...
            if len >= 2 && is_less(v.get_unchecked(len - 1), v.get_unchecked(len - 2)) {
                // Read the last element into a stack-allocated variable. If a following comparison
                // operation panics, `hole` will get dropped and automatically write the element back
                // into the slice.
                let mut tmp = NoDrop {
                    value: Some(ptr::read(v.get_unchecked(len - 1))),
                };
                let mut hole = CopyOnDrop {
                    src: tmp.value.as_mut().unwrap(),
                    dest: v.get_unchecked_mut(len - 2),
                };
                ptr::copy_nonoverlapping(v.get_unchecked(len - 2), v.get_unchecked_mut(len - 1), 1);

                for i in (0..len - 2).rev() {
                    if !is_less(&tmp.value.as_ref().unwrap(), v.get_unchecked(i)) {
                        break;
                    }

                    // Move `i`-th element one place to the right, thus shifting the hole to the left.
                    ptr::copy_nonoverlapping(v.get_unchecked(i), v.get_unchecked_mut(i + 1), 1);
                    hole.dest = v.get_unchecked_mut(i);
                }
                // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
            }
        }
    }

    /// Partially sorts a slice by shifting several out-of-order elements around.
    ///
    /// Returns `true` if the slice is sorted at the end. This function is `O(n)` worst-case.
    #[cold]
    fn partial_insertion_sort<T, F>(v: &mut [T], is_less: &F) -> bool
    where
        F: Fn(&T, &T) -> bool,
    {
        // Maximum number of adjacent out-of-order pairs that will get shifted.
        const MAX_STEPS: usize = 5;
        // If the slice is shorter than this, don't shift any elements.
        const SHORTEST_SHIFTING: usize = 50;

        let len = v.len();
        let mut i = 1;

        for _ in 0..MAX_STEPS {
            unsafe {
                // Find the next pair of adjacent out-of-order elements.
                while i < len && !is_less(v.get_unchecked(i), v.get_unchecked(i - 1)) {
                    i += 1;
                }
            }

            // Are we done?
            if i == len {
                return true;
            }

            // Don't shift elements on short arrays, that has a performance cost.
            if len < SHORTEST_SHIFTING {
                return false;
            }

            // Swap the found pair of elements. This puts them in correct order.
            v.swap(i - 1, i);

            // Shift the smaller element to the left.
            shift_tail(&mut v[..i], is_less);
            // Shift the greater element to the right.
            shift_head(&mut v[i..], is_less);
        }

        // Didn't manage to sort the slice in the limited number of steps.
        false
    }

    /// Sorts a slice using insertion sort, which is `O(n^2)` worst-case.
    fn insertion_sort<T, F>(v: &mut [T], is_less: &F)
    where
        F: Fn(&T, &T) -> bool,
    {
        for i in 1..v.len() {
            shift_tail(&mut v[..=i], is_less);
        }
    }

    /// Sorts `v` using heapsort, which guarantees `O(n log n)` worst-case.
    #[cold]
    fn heapsort<T, F>(v: &mut [T], is_less: &F)
    where
        F: Fn(&T, &T) -> bool,
    {
        // This binary heap respects the invariant `parent >= child`.
        let sift_down = |v: &mut [T], mut node| {
            loop {
                // Children of `node`:
                let left = 2 * node + 1;
                let right = 2 * node + 2;

                // Choose the greater child.
                let greater = if right < v.len() && is_less(&v[left], &v[right]) {
                    right
                } else {
                    left
                };

                // Stop if the invariant holds at `node`.
                if greater >= v.len() || !is_less(&v[node], &v[greater]) {
                    break;
                }

                // Swap `node` with the greater child, move one step down, and continue sifting.
                v.swap(node, greater);
                node = greater;
            }
        };

        // Build the heap in linear time.
        for i in (0..v.len() / 2).rev() {
            sift_down(v, i);
        }

        // Pop maximal elements from the heap.
        for i in (1..v.len()).rev() {
            v.swap(0, i);
            sift_down(&mut v[..i], 0);
        }
    }

    /// Partitions `v` into elements smaller than `pivot`, followed by elements greater than or equal
    /// to `pivot`.
    ///
    /// Returns the number of elements smaller than `pivot`.
    ///
    /// Partitioning is performed block-by-block in order to minimize the cost of branching operations.
    /// This idea is presented in the [BlockQuicksort][pdf] paper.
    ///
    /// [pdf]: http://drops.dagstuhl.de/opus/volltexte/2016/6389/pdf/LIPIcs-ESA-2016-38.pdf
    fn partition_in_blocks<T, F>(v: &mut [T], pivot: &T, is_less: &F) -> usize
    where
        F: Fn(&T, &T) -> bool,
    {
        // Number of elements in a typical block.
        const BLOCK: usize = 128;

        // The partitioning algorithm repeats the following steps until completion:
        //
        // 1. Trace a block from the left side to identify elements greater than or equal to the pivot.
        // 2. Trace a block from the right side to identify elements smaller than the pivot.
        // 3. Exchange the identified elements between the left and right side.
        //
        // We keep the following variables for a block of elements:
        //
        // 1. `block` - Number of elements in the block.
        // 2. `start` - Start pointer into the `offsets` array.
        // 3. `end` - End pointer into the `offsets` array.
        // 4. `offsets - Indices of out-of-order elements within the block.

        // The current block on the left side (from `l` to `l.offset(block_l)`).
        let mut l = v.as_mut_ptr();
        let mut block_l = BLOCK;
        let mut start_l = ptr::null_mut();
        let mut end_l = ptr::null_mut();
        let mut offsets_l = [0u8; BLOCK];

        // The current block on the right side (from `r.offset(-block_r)` to `r`).
        let mut r = unsafe { l.add(v.len()) };
        let mut block_r = BLOCK;
        let mut start_r = ptr::null_mut();
        let mut end_r = ptr::null_mut();
        let mut offsets_r = [0u8; BLOCK];

        // Returns the number of elements between pointers `l` (inclusive) and `r` (exclusive).
        fn width<T>(l: *mut T, r: *mut T) -> usize {
            assert!(mem::size_of::<T>() > 0);
            (r as usize - l as usize) / mem::size_of::<T>()
        }

        loop {
            // We are done with partitioning block-by-block when `l` and `r` get very close. Then we do
            // some patch-up work in order to partition the remaining elements in between.
            let is_done = width(l, r) <= 2 * BLOCK;

            if is_done {
                // Number of remaining elements (still not compared to the pivot).
                let mut rem = width(l, r);
                if start_l < end_l || start_r < end_r {
                    rem -= BLOCK;
                }

                // Adjust block sizes so that the left and right block don't overlap, but get perfectly
                // aligned to cover the whole remaining gap.
                if start_l < end_l {
                    block_r = rem;
                } else if start_r < end_r {
                    block_l = rem;
                } else {
                    block_l = rem / 2;
                    block_r = rem - block_l;
                }
                debug_assert!(block_l <= BLOCK && block_r <= BLOCK);
                debug_assert!(width(l, r) == block_l + block_r);
            }

            if start_l == end_l {
                // Trace `block_l` elements from the left side.
                start_l = offsets_l.as_mut_ptr();
                end_l = offsets_l.as_mut_ptr();
                let mut elem = l;

                for i in 0..block_l {
                    unsafe {
                        // Branchless comparison.
                        *end_l = i as u8;
                        end_l = end_l.offset(!is_less(&*elem, pivot) as isize);
                        elem = elem.offset(1);
                    }
                }
            }

            if start_r == end_r {
                // Trace `block_r` elements from the right side.
                start_r = offsets_r.as_mut_ptr();
                end_r = offsets_r.as_mut_ptr();
                let mut elem = r;

                for i in 0..block_r {
                    unsafe {
                        // Branchless comparison.
                        elem = elem.offset(-1);
                        *end_r = i as u8;
                        end_r = end_r.offset(is_less(&*elem, pivot) as isize);
                    }
                }
            }

            // Number of out-of-order elements to swap between the left and right side.
            let count = cmp::min(width(start_l, end_l), width(start_r, end_r));

            if count > 0 {
                macro_rules! left {
                    () => {
                        l.offset(*start_l as isize)
                    };
                }
                macro_rules! right {
                    () => {
                        r.offset(-(*start_r as isize) - 1)
                    };
                }

                // Instead of swapping one pair at the time, it is more efficient to perform a cyclic
                // permutation. This is not strictly equivalent to swapping, but produces a similar
                // result using fewer memory operations.
                unsafe {
                    let tmp = ptr::read(left!());
                    ptr::copy_nonoverlapping(right!(), left!(), 1);

                    for _ in 1..count {
                        start_l = start_l.offset(1);
                        ptr::copy_nonoverlapping(left!(), right!(), 1);
                        start_r = start_r.offset(1);
                        ptr::copy_nonoverlapping(right!(), left!(), 1);
                    }

                    ptr::copy_nonoverlapping(&tmp, right!(), 1);
                    mem::forget(tmp);
                    start_l = start_l.offset(1);
                    start_r = start_r.offset(1);
                }
            }

            if start_l == end_l {
                // All out-of-order elements in the left block were moved. Move to the next block.
                l = unsafe { l.add(block_l) };
            }

            if start_r == end_r {
                // All out-of-order elements in the right block were moved. Move to the previous block.
                r = unsafe { r.sub(block_r) };
            }

            if is_done {
                break;
            }
        }

        // All that remains now is at most one block (either the left or the right) with out-of-order
        // elements that need to be moved. Such remaining elements can be simply shifted to the end
        // within their block.

        if start_l < end_l {
            // The left block remains.
            // Move it's remaining out-of-order elements to the far right.
            debug_assert_eq!(width(l, r), block_l);
            while start_l < end_l {
                unsafe {
                    end_l = end_l.offset(-1);
                    ptr::swap(l.offset(*end_l as isize), r.offset(-1));
                    r = r.offset(-1);
                }
            }
            width(v.as_mut_ptr(), r)
        } else if start_r < end_r {
            // The right block remains.
            // Move it's remaining out-of-order elements to the far left.
            debug_assert_eq!(width(l, r), block_r);
            while start_r < end_r {
                unsafe {
                    end_r = end_r.offset(-1);
                    ptr::swap(l, r.offset(-(*end_r as isize) - 1));
                    l = l.offset(1);
                }
            }
            width(v.as_mut_ptr(), l)
        } else {
            // Nothing else to do, we're done.
            width(v.as_mut_ptr(), l)
        }
    }

    /// Partitions `v` into elements smaller than `v[pivot]`, followed by elements greater than or
    /// equal to `v[pivot]`.
    ///
    /// Returns a tuple of:
    ///
    /// 1. Number of elements smaller than `v[pivot]`.
    /// 2. True if `v` was already partitioned.
    fn partition<T, F>(v: &mut [T], pivot: usize, is_less: &F) -> (usize, bool)
    where
        F: Fn(&T, &T) -> bool,
    {
        let (mid, was_partitioned) = {
            // Place the pivot at the beginning of slice.
            v.swap(0, pivot);
            let (pivot, v) = v.split_at_mut(1);
            let pivot = &mut pivot[0];

            // Read the pivot into a stack-allocated variable for efficiency. If a following comparison
            // operation panics, the pivot will be automatically written back into the slice.
            let write_on_drop = WriteOnDrop {
                value: unsafe { Some(ptr::read(pivot)) },
                dest: pivot,
            };
            let pivot = write_on_drop.value.as_ref().unwrap();

            // Find the first pair of out-of-order elements.
            let mut l = 0;
            let mut r = v.len();
            unsafe {
                // Find the first element greater then or equal to the pivot.
                while l < r && is_less(v.get_unchecked(l), pivot) {
                    l += 1;
                }

                // Find the last element smaller that the pivot.
                while l < r && !is_less(v.get_unchecked(r - 1), pivot) {
                    r -= 1;
                }
            }

            (
                l + partition_in_blocks(&mut v[l..r], pivot, is_less),
                l >= r,
            )

            // `write_on_drop` goes out of scope and writes the pivot (which is a stack-allocated
            // variable) back into the slice where it originally was. This step is critical in ensuring
            // safety!
        };

        // Place the pivot between the two partitions.
        v.swap(0, mid);

        (mid, was_partitioned)
    }

    /// Partitions `v` into elements equal to `v[pivot]` followed by elements greater than `v[pivot]`.
    ///
    /// Returns the number of elements equal to the pivot. It is assumed that `v` does not contain
    /// elements smaller than the pivot.
    fn partition_equal<T, F>(v: &mut [T], pivot: usize, is_less: &F) -> usize
    where
        F: Fn(&T, &T) -> bool,
    {
        // Place the pivot at the beginning of slice.
        v.swap(0, pivot);
        let (pivot, v) = v.split_at_mut(1);
        let pivot = &mut pivot[0];

        // Read the pivot into a stack-allocated variable for efficiency. If a following comparison
        // operation panics, the pivot will be automatically written back into the slice.
        let write_on_drop = WriteOnDrop {
            value: unsafe { Some(ptr::read(pivot)) },
            dest: pivot,
        };
        let pivot = write_on_drop.value.as_ref().unwrap();

        // Now partition the slice.
        let mut l = 0;
        let mut r = v.len();
        loop {
            unsafe {
                // Find the first element greater that the pivot.
                while l < r && !is_less(pivot, v.get_unchecked(l)) {
                    l += 1;
                }

                // Find the last element equal to the pivot.
                while l < r && is_less(pivot, v.get_unchecked(r - 1)) {
                    r -= 1;
                }

                // Are we done?
                if l >= r {
                    break;
                }

                // Swap the found pair of out-of-order elements.
                r -= 1;
                ptr::swap(v.get_unchecked_mut(l), v.get_unchecked_mut(r));
                l += 1;
            }
        }

        // We found `l` elements equal to the pivot. Add 1 to account for the pivot itself.
        l + 1

        // `write_on_drop` goes out of scope and writes the pivot (which is a stack-allocated variable)
        // back into the slice where it originally was. This step is critical in ensuring safety!
    }

    /// Scatters some elements around in an attempt to break patterns that might cause imbalanced
    /// partitions in quicksort.
    #[cold]
    fn break_patterns<T>(v: &mut [T]) {
        let len = v.len();
        if len >= 8 {
            // Pseudorandom number generator from the "Xorshift RNGs" paper by George Marsaglia.
            let mut random = len as u32;
            let mut gen_u32 = || {
                random ^= random << 13;
                random ^= random >> 17;
                random ^= random << 5;
                random
            };
            let mut gen_usize = || {
                if mem::size_of::<usize>() <= 4 {
                    gen_u32() as usize
                } else {
                    ((u64::from(gen_u32()) << 32) | u64::from(gen_u32())) as usize
                }
            };

            // Take random numbers modulo this number.
            // The number fits into `usize` because `len` is not greater than `isize::MAX`.
            let modulus = len.next_power_of_two();

            // Some pivot candidates will be in the nearby of this index. Let's randomize them.
            let pos = len / 4 * 2;

            for i in 0..3 {
                // Generate a random number modulo `len`. However, in order to avoid costly operations
                // we first take it modulo a power of two, and then decrease by `len` until it fits
                // into the range `[0, len - 1]`.
                let mut other = gen_usize() & (modulus - 1);

                // `other` is guaranteed to be less than `2 * len`.
                if other >= len {
                    other -= len;
                }

                v.swap(pos - 1 + i, other);
            }
        }
    }

    /// Chooses a pivot in `v` and returns the index and `true` if the slice is likely already sorted.
    ///
    /// Elements in `v` might be reordered in the process.
    fn choose_pivot<T, F>(v: &mut [T], is_less: &F) -> (usize, bool)
    where
        F: Fn(&T, &T) -> bool,
    {
        // Minimum length to choose the median-of-medians method.
        // Shorter slices use the simple median-of-three method.
        const SHORTEST_MEDIAN_OF_MEDIANS: usize = 50;
        // Maximum number of swaps that can be performed in this function.
        const MAX_SWAPS: usize = 4 * 3;

        let len = v.len();

        // Three indices near which we are going to choose a pivot.
        let mut a = len / 4 * 1;
        let mut b = len / 4 * 2;
        let mut c = len / 4 * 3;

        // Counts the total number of swaps we are about to perform while sorting indices.
        let mut swaps = 0;

        if len >= 8 {
            // Swaps indices so that `v[a] <= v[b]`.
            let mut sort2 = |a: &mut usize, b: &mut usize| unsafe {
                if is_less(v.get_unchecked(*b), v.get_unchecked(*a)) {
                    ptr::swap(a, b);
                    swaps += 1;
                }
            };

            // Swaps indices so that `v[a] <= v[b] <= v[c]`.
            let mut sort3 = |a: &mut usize, b: &mut usize, c: &mut usize| {
                sort2(a, b);
                sort2(b, c);
                sort2(a, b);
            };

            if len >= SHORTEST_MEDIAN_OF_MEDIANS {
                // Finds the median of `v[a - 1], v[a], v[a + 1]` and stores the index into `a`.
                let mut sort_adjacent = |a: &mut usize| {
                    let tmp = *a;
                    sort3(&mut (tmp - 1), a, &mut (tmp + 1));
                };

                // Find medians in the neighborhoods of `a`, `b`, and `c`.
                sort_adjacent(&mut a);
                sort_adjacent(&mut b);
                sort_adjacent(&mut c);
            }

            // Find the median among `a`, `b`, and `c`.
            sort3(&mut a, &mut b, &mut c);
        }

        if swaps < MAX_SWAPS {
            (b, swaps == 0)
        } else {
            // The maximum number of swaps was performed. Chances are the slice is descending or mostly
            // descending, so reversing will probably help sort it faster.
            v.reverse();
            (len - 1 - b, true)
        }
    }

    /// Sorts `v` recursively.
    ///
    /// If the slice had a predecessor in the original array, it is specified as `pred`.
    ///
    /// `limit` is the number of allowed imbalanced partitions before switching to `heapsort`. If zero,
    /// this function will immediately switch to heapsort.
    fn recurse<'a, T, F>(mut v: &'a mut [T], is_less: &F, mut pred: Option<&'a mut T>, mut limit: usize)
    where
        F: Fn(&T, &T) -> bool,
    {
        // Slices of up to this length get sorted using insertion sort.
        const MAX_INSERTION: usize = 20;
        // If both partitions are up to this length, we continue sequentially. This number is as small
        // as possible but so that the overhead of Rayon's task scheduling is still negligible.
        const MAX_SEQUENTIAL: usize = 2000;

        // True if the last partitioning was reasonably balanced.
        let mut was_balanced = true;
        // True if the last partitioning didn't shuffle elements (the slice was already partitioned).
        let mut was_partitioned = true;

        loop {
            let len = v.len();

            // Very short slices get sorted using insertion sort.
            if len <= MAX_INSERTION {
                insertion_sort(v, is_less);
                return;
            }

            // If too many bad pivot choices were made, simply fall back to heapsort in order to
            // guarantee `O(n log n)` worst-case.
            if limit == 0 {
                heapsort(v, is_less);
                return;
            }

            // If the last partitioning was imbalanced, try breaking patterns in the slice by shuffling
            // some elements around. Hopefully we'll choose a better pivot this time.
            if !was_balanced {
                break_patterns(v);
                limit -= 1;
            }

            // Choose a pivot and try guessing whether the slice is already sorted.
            let (pivot, likely_sorted) = choose_pivot(v, is_less);

            // If the last partitioning was decently balanced and didn't shuffle elements, and if pivot
            // selection predicts the slice is likely already sorted...
            if was_balanced && was_partitioned && likely_sorted {
                // Try identifying several out-of-order elements and shifting them to correct
                // positions. If the slice ends up being completely sorted, we're done.
                if partial_insertion_sort(v, is_less) {
                    return;
                }
            }

            // If the chosen pivot is equal to the predecessor, then it's the smallest element in the
            // slice. Partition the slice into elements equal to and elements greater than the pivot.
            // This case is usually hit when the slice contains many duplicate elements.
            if let Some(ref p) = pred {
                if !is_less(p, &v[pivot]) {
                    let mid = partition_equal(v, pivot, is_less);

                    // Continue sorting elements greater than the pivot.
                    v = &mut { v }[mid..];
                    continue;
                }
            }

            // Partition the slice.
            let (mid, was_p) = partition(v, pivot, is_less);
            was_balanced = cmp::min(mid, len - mid) >= len / 8;
            was_partitioned = was_p;

            // Split the slice into `left`, `pivot`, and `right`.
            let (left, right) = { v }.split_at_mut(mid);
            let (pivot, right) = right.split_at_mut(1);
            let pivot = &mut pivot[0];

            if cmp::max(left.len(), right.len()) <= MAX_SEQUENTIAL {
                // Recurse into the shorter side only in order to minimize the total number of recursive
                // calls and consume less stack space. Then just continue with the longer side (this is
                // akin to tail recursion).
                if left.len() < right.len() {
                    recurse(left, is_less, pred, limit);
                    v = right;
                    pred = Some(pivot);
                } else {
                    recurse(right, is_less, Some(pivot), limit);
                    v = left;
                }
            } else {
                // Sort the left and right half in parallel.
                recurse(left, is_less, pred, limit);
                recurse(right, is_less, Some(pivot), limit);
                break;
            }
        }
    }

    /// Sorts `v` using pattern-defeating quicksort in parallel.
    ///
    /// The algorithm is unstable, in-place, and `O(n log n)` worst-case.
    pub fn quicksort<T, F>(v: &mut [T], is_less: F)
    where
        F: Fn(&T, &T) -> bool,
    {
        // Sorting has no meaningful behavior on zero-sized types.
        if mem::size_of::<T>() == 0 {
            return;
        }

        // Limit the number of imbalanced partitions to `floor(log2(len)) + 1`.
        let limit = mem::size_of::<usize>() * 8 - v.len().leading_zeros() as usize;

        recurse(v, &is_less, None, limit);
    }

    /// Sorts `v` recursively.
    ///
    /// If the slice had a predecessor in the original array, it is specified as `pred`.
    ///
    /// `limit` is the number of allowed imbalanced partitions before switching to `heapsort`. If zero,
    /// this function will immediately switch to heapsort.
    fn par_recurse<'a, T, F>(mut v: &'a mut [T], is_less: &F, mut pred: Option<&'a mut T>, mut limit: usize)
    where
        T: Send,
        F: Fn(&T, &T) -> bool + Sync,
    {
        // Slices of up to this length get sorted using insertion sort.
        const MAX_INSERTION: usize = 20;
        // If both partitions are up to this length, we continue sequentially. This number is as small
        // as possible but so that the overhead of Rayon's task scheduling is still negligible.
        const MAX_SEQUENTIAL: usize = 2000;

        // True if the last partitioning was reasonably balanced.
        let mut was_balanced = true;
        // True if the last partitioning didn't shuffle elements (the slice was already partitioned).
        let mut was_partitioned = true;

        loop {
            let len = v.len();

            // Very short slices get sorted using insertion sort.
            if len <= MAX_INSERTION {
                insertion_sort(v, is_less);
                return;
            }

            // If too many bad pivot choices were made, simply fall back to heapsort in order to
            // guarantee `O(n log n)` worst-case.
            if limit == 0 {
                heapsort(v, is_less);
                return;
            }

            // If the last partitioning was imbalanced, try breaking patterns in the slice by shuffling
            // some elements around. Hopefully we'll choose a better pivot this time.
            if !was_balanced {
                break_patterns(v);
                limit -= 1;
            }

            // Choose a pivot and try guessing whether the slice is already sorted.
            let (pivot, likely_sorted) = choose_pivot(v, is_less);

            // If the last partitioning was decently balanced and didn't shuffle elements, and if pivot
            // selection predicts the slice is likely already sorted...
            if was_balanced && was_partitioned && likely_sorted {
                // Try identifying several out-of-order elements and shifting them to correct
                // positions. If the slice ends up being completely sorted, we're done.
                if partial_insertion_sort(v, is_less) {
                    return;
                }
            }

            // If the chosen pivot is equal to the predecessor, then it's the smallest element in the
            // slice. Partition the slice into elements equal to and elements greater than the pivot.
            // This case is usually hit when the slice contains many duplicate elements.
            if let Some(ref p) = pred {
                if !is_less(p, &v[pivot]) {
                    let mid = partition_equal(v, pivot, is_less);

                    // Continue sorting elements greater than the pivot.
                    v = &mut { v }[mid..];
                    continue;
                }
            }

            // Partition the slice.
            let (mid, was_p) = partition(v, pivot, is_less);
            was_balanced = cmp::min(mid, len - mid) >= len / 8;
            was_partitioned = was_p;

            // Split the slice into `left`, `pivot`, and `right`.
            let (left, right) = { v }.split_at_mut(mid);
            let (pivot, right) = right.split_at_mut(1);
            let pivot = &mut pivot[0];

            if cmp::max(left.len(), right.len()) <= MAX_SEQUENTIAL {
                // Recurse into the shorter side only in order to minimize the total number of recursive
                // calls and consume less stack space. Then just continue with the longer side (this is
                // akin to tail recursion).
                if left.len() < right.len() {
                    recurse(left, is_less, pred, limit);
                    v = right;
                    pred = Some(pivot);
                } else {
                    recurse(right, is_less, Some(pivot), limit);
                    v = left;
                }
            } else {
                // Sort the left and right half in parallel.
                rayon_core::join(
                    || recurse(left, is_less, pred, limit),
                    || recurse(right, is_less, Some(pivot), limit),
                );
                break;
            }
        }
    }

    /// Sorts `v` using pattern-defeating quicksort in parallel.
    ///
    /// The algorithm is unstable, in-place, and `O(n log n)` worst-case.
    pub fn par_quicksort<T, F>(v: &mut [T], is_less: F)
    where
        T: Send,
        F: Fn(&T, &T) -> bool + Sync,
    {
        // Sorting has no meaningful behavior on zero-sized types.
        if mem::size_of::<T>() == 0 {
            return;
        }

        // Limit the number of imbalanced partitions to `floor(log2(len)) + 1`.
        let limit = mem::size_of::<usize>() * 8 - v.len().leading_zeros() as usize;

        par_recurse(v, &is_less, None, limit);
    }

    #[cfg(test)]
    mod tests {
        use super::heapsort;
        use rand::distributions::Uniform;
        use rand::{thread_rng, Rng};

        #[test]
        fn test_heapsort() {
            let rng = thread_rng();

            for len in (0..25).chain(500..501) {
                for &modulus in &[5, 10, 100] {
                    let dist = Uniform::new(0, modulus);
                    for _ in 0..100 {
                        let v: Vec<i32> = rng.sample_iter(&dist).take(len).collect();

                        // Test heapsort using `<` operator.
                        let mut tmp = v.clone();
                        heapsort(&mut tmp, &|a, b| a < b);
                        assert!(tmp.windows(2).all(|w| w[0] <= w[1]));

                        // Test heapsort using `>` operator.
                        let mut tmp = v.clone();
                        heapsort(&mut tmp, &|a, b| a > b);
                        assert!(tmp.windows(2).all(|w| w[0] >= w[1]));
                    }
                }
            }

            // Sort using a completely random comparison function.
            // This will reorder the elements *somehow*, but won't panic.
            let mut v: Vec<_> = (0..100).collect();
            heapsort(&mut v, &|_, _| thread_rng().gen());
            heapsort(&mut v, &|a, b| a < b);

            for i in 0..v.len() {
                assert_eq!(v[i], i);
            }
        }
    }

}


// TODO Hmmm - I don't want to have to manually keep these in sync
const LOG_N_BUCKETS: usize = 8;
const MAX_NUM_BUCKETS: usize = 256;
const BLOCK_SIZE: usize = 512;

fn sample<T>(v: &mut [T], num_samples: usize)
{
    // benchmark - maybe replace with XorShift?
    // TODO: notice the way break_patterns is implemented
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(0..v.len());

    for dest in 0..num_samples {
        let src = uniform.sample(&mut rng);
        v.swap(src, dest);
    }
}

fn classify<T, F>(classifier: &[T], value: &T, is_less: &F, log_n_buckets: usize) -> usize
where 
    T: Clone + std::fmt::Debug + Ord,
    F: Fn(&T, &T) -> bool,
{
    let mut i = 0;

    for _ in 0..log_n_buckets {
        i = (2 * i) + 1 + (value > &&classifier[i]) as usize;
        println!("value: {:?} i: {:?}", value, i);
    }

    i - classifier.len()
}

fn build_classifier<T>(data: &[T], out: &mut [T], dest: usize)
where
    T: Clone,
{
    let len = data.len();
    if len > 0 {
        let mid = len / 2;        
        let left = len / 4;
        let right = (len / 2) + (len / 4);

        out[dest] = data[mid].clone();
        build_classifier(&data[..mid], out, dest * 2 + 1);
        build_classifier(&data[(mid + 1)..], out, dest * 2 + 2);
    }
}

fn select_classifier_samples<T, F>(v: &mut [T], is_less: &F, out: &mut Vec<T>)
where 
    T: Clone + std::fmt::Debug + Ord,
    F: Fn(&T, &T) -> bool,
{
    // Choose splitters (i.e. equally-spaced samples) to determine buckets

    for splitter_index in 0..MAX_NUM_BUCKETS {
        let sample_index = splitter_index * (v.len() / MAX_NUM_BUCKETS);
        // TODO critical, maybe - remove duplicate splitters? fill with outside most?
        out.push(v[sample_index].clone());
    }

    // TODO missing equality buckets
}

fn choose_sampling_number(len: usize) -> usize
{
    let oversampling_factor: f64 = 1.0_f64.min(0.2 * (len >> 1) as f64);
    let step = 1_usize.max(oversampling_factor as usize);
    let num_samples = step * MAX_NUM_BUCKETS - 1;
    num_samples
}

fn locally_classify<T, F>(v: &mut [T], is_less: &F, classifier: &[T], num_buckets: usize, log_n_buckets: usize, block_size: usize, buffer: &mut [T], counts: &mut [usize])
where 
    T: Clone + std::fmt::Debug + Ord,
    F: Fn(&T, &T) -> bool,
{
    // TODO remove allocations
    let mut data_write_index = 0;
    let mut buffer_write_indices = vec![0; num_buckets];


    
    for data_read_index in 0..v.len() {
        // TODO remove bounds checking
        let value = v[data_read_index].clone();
        
        let bucket_index = classify(classifier, &value, is_less, log_n_buckets);

        if buffer_write_indices[bucket_index] == block_size {
            for idx in 0..block_size {
                v[data_write_index] = buffer[bucket_index * block_size + idx].clone();
                data_write_index += 1;
            }
            buffer_write_indices[bucket_index] = 0;
        }

        buffer[bucket_index * block_size + buffer_write_indices[bucket_index]] = value;
        buffer_write_indices[bucket_index] += 1;
        counts[bucket_index] += 1;
    }
}

// TODO maybe is_less doesn't need to be a reference.
fn recurse<T, F>(v: &mut [T], is_less: &F)
where 
    T: Clone + std::fmt::Debug + Ord,
    F: Fn(&T, &T) -> bool,
{
    // Choose your basecase - not benchmarked yet!
    const MAX_PDQ: usize = MAX_NUM_BUCKETS & MAX_NUM_BUCKETS;

    let len = v.len();

    // Slices up to this length get sorted using pdqsort.
    if len <= MAX_PDQ {
        quicksort::quicksort(v, is_less);
    } else {
        let num_samples = choose_sampling_number(v.len());
        // samples num_samples elements from v and puts them in the front
        sample(v, num_samples);

        // or parallel
        // sorts the samples - required for building classifier
        recurse(&mut v[..num_samples], is_less);

        // TODO allocation bad
        // must be a power of 2 minus 1
        let mut classifier = Vec::with_capacity(MAX_NUM_BUCKETS - 1);
        
        {
            let mut classifier_input = Vec::with_capacity(MAX_NUM_BUCKETS - 1);
            select_classifier_samples(&mut v[..num_samples], is_less, &mut classifier_input);
            assert!(classifier_input.is_sorted());
            
            // fill with junk
            // safe, but stupid
            for i in 0..classifier.len() {
                classifier[i] = classifier_input[0].clone();
            }
            build_classifier(&classifier_input, &mut classifier, 0);            
        }
        // Local Classification

        let mut block_buffer = Vec::new();
        // safe, but stupid
        for i in 0..(MAX_NUM_BUCKETS * BLOCK_SIZE) {
            block_buffer.push(classifier[0].clone());
        }
        // write buffers
        // count size of buckets
        let mut counts = vec![0; MAX_NUM_BUCKETS];

        // what a mouthful! 
        // TODO rethink the parameters of this function
        locally_classify(v, &is_less, &classifier, MAX_NUM_BUCKETS, LOG_N_BUCKETS, BLOCK_SIZE, &mut block_buffer, &mut counts);

        // At this stage, the data is in a precarious state
        // Some data is in the original slice. Some parts of the slice are effectively unallocated
        // And some data exists only in the local buffers.
        // Making this safe will be challenging!

        // Block Permutation

            // move full blocks
            // overflow block?
        // Cleanup
    }

}

fn par_recurse<T, F>(v: &mut [T], is_less: F)
where 
    T: Send, 
    F: Fn(&T, &T) -> bool + Sync,
{
    // Choose a basecase - not benchmarked yet!
    const MAX_PDQ: usize = 256 * 256;

    let len = v.len();

    // Slices up to this length get sorted using insertion sort.
    if len <= MAX_PDQ {
        quicksort::par_quicksort(v, is_less);
    }

    //let classifier = build_classifier(v, is_less);

    // Local Classification

    // Block Permutation

    // Cleanup
}

pub fn ips4o<T, F>(v: &mut [T], is_less: F)
where 
    T: Clone + std::fmt::Debug + Ord,
    F: Fn(&T, &T) -> bool,
{
    // Sorting has no meaningful behavior on zero-sized types.
    if mem::size_of::<T>() == 0 {
        return;
    }

    recurse(v, &is_less)
}

pub fn par_ips4o<T, F>(v: &mut [T], is_less: F)
where 
    T: Send,
    F: Fn(&T, &T) -> bool + Sync,
{
    // Sorting has no meaningful behavior on zero-sized types.
    if mem::size_of::<T>() == 0 {
        return;
    }

    par_recurse(v, is_less)
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_classifier() {
        let data = vec!(-10, -6, -4, -2, 0, 2, 4);
        let mut classifier = vec![0; 7];
        build_classifier(&data, &mut classifier, 0);
        let expected = vec!(-2, -6, 2, -10, -4, 0, 4);
        assert_eq!(expected, classifier);
    }

    #[test]
    fn test_classify() {        
        let classifier = vec!(-2, -6, 2, -10, -4, 0, 4);
        
        assert_eq!(0, classify(&classifier, &-1000, &i32::lt, 3));
        assert_eq!(7, classify(&classifier, &1000, &i32::lt, 3));

        assert_eq!(1, classify(&classifier, &-8, &i32::lt, 3));
        assert_eq!(5, classify(&classifier, &2, &i32::lt, 3));
    }

    #[test]
    fn test_locally_classify() {        
        let classifier = vec!(-2, -6, 2, -10, -4, 0, 4);
        let mut data = vec!(-20, 20, -19, -18, 19, 18, 1);

        let expected = vec!(-20, -19, 20, 19, 19, 18, 1);
        
        let mut buffer = vec![0; 2 * 8];
        let mut counts = vec![0; 8];

        locally_classify(&mut data, &i32::lt, &classifier, 8, 3, 2, &mut buffer, &mut counts);
        assert_eq!(expected, data);
    }
}