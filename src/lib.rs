#![cfg_attr(feature = "unstable", feature(test))]


mod sort;

pub fn sort<T>(v: &mut [T])
where T: Ord,
{
    sort::sequential_sort(v,  |a, b| a.lt(b));    
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::panic;
    
    use rand::distributions::Standard;
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng};
    

    #[test]
    fn test_sort() {
        let mut rng = thread_rng();
    
        for len in (2..25).chain(500..510) {
            for &modulus in &[5, 10, 100, 1000] {
                for _ in 0..10 {
                    let orig: Vec<_> =
                        rng.sample_iter::<i32, _>(&Standard).map(|x| x % modulus).take(len).collect();
    
                    // Sort in default order.
                    let mut v = orig.clone();
                    v.sort();
                    assert!(v.windows(2).all(|w| w[0] <= w[1]));
    
                    // Sort in ascending order.
                    let mut v = orig.clone();
                    v.sort_by(|a, b| a.cmp(b));
                    assert!(v.windows(2).all(|w| w[0] <= w[1]));
    
                    // Sort in descending order.
                    let mut v = orig.clone();
                    v.sort_by(|a, b| b.cmp(a));
                    assert!(v.windows(2).all(|w| w[0] >= w[1]));
    
                    // Sort in lexicographic order.
                    let mut v1 = orig.clone();
                    let mut v2 = orig.clone();
                    v1.sort_by_key(|x| x.to_string());
                    v2.sort_by_cached_key(|x| x.to_string());
                    assert!(v1.windows(2).all(|w| w[0].to_string() <= w[1].to_string()));
                    assert!(v1 == v2);
    
                    // Sort with many pre-sorted runs.
                    let mut v = orig.clone();
                    v.sort();
                    v.reverse();
                    for _ in 0..5 {
                        let a = rng.gen::<usize>() % len;
                        let b = rng.gen::<usize>() % len;
                        if a < b {
                            v[a..b].reverse();
                        } else {
                            v.swap(a, b);
                        }
                    }
                    v.sort();
                    assert!(v.windows(2).all(|w| w[0] <= w[1]));
                }
            }
        }
    
        // Sort using a completely random comparison function.
        // This will reorder the elements *somehow*, but won't panic.
        let mut v = [0; 500];
        for i in 0..v.len() {
            v[i] = i as i32;
        }
        v.sort_by(|_, _| *[Less, Equal, Greater].choose(&mut rng).unwrap());
        v.sort();
        for i in 0..v.len() {
            assert_eq!(v[i], i as i32);
        }
    
        // Should not panic.
        [0i32; 0].sort();
        [(); 10].sort();
        [(); 100].sort();
    
        let mut v = [0xDEADBEEFu64];
        v.sort();
        assert!(v == [0xDEADBEEF]);
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    use std::mem;
    
    use rand::distributions::{Alphanumeric, Standard};
    use rand::{Rng, SeedableRng};
    use rand_xorshift::XorShiftRng;

    extern crate test;
    use test::{black_box, Bencher};

    fn gen_ascending(len: usize) -> Vec<u64> {
        (0..len as u64).collect()
    }
    
    fn gen_descending(len: usize) -> Vec<u64> {
        (0..len as u64).rev().collect()
    }
    
    const SEED: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    
    fn gen_random(len: usize) -> Vec<u64> {
        let mut rng = XorShiftRng::from_seed(SEED);
        (&mut rng).sample_iter(&Standard).take(len).collect()
    }
    
    fn gen_mostly_ascending(len: usize) -> Vec<u64> {
        let mut rng = XorShiftRng::from_seed(SEED);
        let mut v = gen_ascending(len);
        for _ in (0usize..).take_while(|x| x * x <= len) {
            let x = rng.gen::<usize>() % len;
            let y = rng.gen::<usize>() % len;
            v.swap(x, y);
        }
        v
    }
    
    fn gen_mostly_descending(len: usize) -> Vec<u64> {
        let mut rng = XorShiftRng::from_seed(SEED);
        let mut v = gen_descending(len);
        for _ in (0usize..).take_while(|x| x * x <= len) {
            let x = rng.gen::<usize>() % len;
            let y = rng.gen::<usize>() % len;
            v.swap(x, y);
        }
        v
    }
    
    fn gen_strings(len: usize) -> Vec<String> {
        let mut rng = XorShiftRng::from_seed(SEED);
        let mut v = vec![];
        for _ in 0..len {
            let n = rng.gen::<usize>() % 20 + 1;
            v.push((&mut rng).sample_iter(&Alphanumeric).take(n).collect());
        }
        v
    }
    
    fn gen_big_random(len: usize) -> Vec<[u64; 16]> {
        let mut rng = XorShiftRng::from_seed(SEED);
        (&mut rng).sample_iter(&Standard).map(|x| [x; 16]).take(len).collect()
    }
    
    macro_rules! sort {
        ($f:ident, $name:ident, $gen:expr, $len:expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let v = $gen($len);
                b.iter(|| v.clone().$f());
                b.bytes = $len * mem::size_of_val(&$gen(1)[0]) as u64;
            }
        };
    }
    
    macro_rules! sort_strings {
        ($f:ident, $name:ident, $gen:expr, $len:expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let v = $gen($len);
                let v = v.iter().map(|s| &**s).collect::<Vec<&str>>();
                b.iter(|| v.clone().$f());
                b.bytes = $len * mem::size_of::<&str>() as u64;
            }
        };
    }
    
    macro_rules! sort_expensive {
        ($f:ident, $name:ident, $gen:expr, $len:expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let v = $gen($len);
                b.iter(|| {
                    let mut v = v.clone();
                    let mut count = 0;
                    v.$f(|a: &u64, b: &u64| {
                        count += 1;
                        if count % 1_000_000_000 == 0 {
                            panic!("should not happen");
                        }
                        (*a as f64).cos().partial_cmp(&(*b as f64).cos()).unwrap()
                    });
                    black_box(count);
                });
                b.bytes = $len * mem::size_of_val(&$gen(1)[0]) as u64;
            }
        };
    }
    
    macro_rules! sort_lexicographic {
        ($f:ident, $name:ident, $gen:expr, $len:expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let v = $gen($len);
                b.iter(|| v.clone().$f(|x| x.to_string()));
                b.bytes = $len * mem::size_of_val(&$gen(1)[0]) as u64;
            }
        };
    }
    
    sort!(sort_unstable, sort_unstable_small_ascending, gen_ascending, 10);
    sort!(sort_unstable, sort_unstable_small_descending, gen_descending, 10);
    sort!(sort_unstable, sort_unstable_small_random, gen_random, 10);
    sort!(sort_unstable, sort_unstable_small_big, gen_big_random, 10);
    sort!(sort_unstable, sort_unstable_medium_random, gen_random, 100);
    sort!(sort_unstable, sort_unstable_large_ascending, gen_ascending, 10000);
    sort!(sort_unstable, sort_unstable_large_descending, gen_descending, 10000);
    sort!(sort_unstable, sort_unstable_large_mostly_ascending, gen_mostly_ascending, 10000);
    sort!(sort_unstable, sort_unstable_large_mostly_descending, gen_mostly_descending, 10000);
    sort!(sort_unstable, sort_unstable_large_random, gen_random, 10000);
    sort!(sort_unstable, sort_unstable_large_big, gen_big_random, 10000);
    sort!(sort_unstable, sort_unstable_huge_random, gen_random, 10_000_000);
    sort_strings!(sort_unstable, sort_unstable_large_strings, gen_strings, 10000);
    sort_expensive!(sort_unstable_by, sort_unstable_large_expensive, gen_random, 10000);
    
    sort_lexicographic!(sort_by_key, sort_by_key_lexicographic, gen_random, 10000);
    sort_lexicographic!(sort_unstable_by_key, sort_unstable_by_key_lexicographic, gen_random, 10000);
    sort_lexicographic!(sort_by_cached_key, sort_by_cached_key_lexicographic, gen_random, 10000);
}