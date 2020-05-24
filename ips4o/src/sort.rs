use std::cmp;
use std::mem;
use std::ptr;

extern crate rand;
use rand::distributions::{Distribution, Uniform};

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

fn recurse<T, F>(v: &mut [T], is_less: &mut F)
where F: FnMut(&T, &T) -> bool,
{
    // pdqsort has 20, ips4o is 16. benchmark.
    const MAX_INSERTION: usize = 16;

    let len = v.len();

    // Slices up to this length get sorted using insertion sort.
    if len <= MAX_INSERTION {
        insertion_sort(v, is_less);
    }

    let classifier = build_classifier(v, is_less);

    // Local Classification

    // Block Permutation

    // Cleanup
}

pub fn sequential_sort<T, F>(v: &mut [T], mut is_less: F)
where F: FnMut(&T, &T) -> bool,
{
    // Sorting has no meaningful behavior on zero-sized types.
    if mem::size_of::<T>() == 0 {
        return;
    }

    recurse(v, &mut is_less)
}