use rand::seq::SliceRandom;
use rand::distributions::{Alphanumeric, Standard};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use rayon::prelude::*;
use std::time::{Duration, Instant};
use std::ops::Add;

use ips4o;

fn mean(data: &[Duration]) -> Option<f32>
{
    let sum = data.iter().sum::<Duration>().as_nanos() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

trait GenData {

    fn uniform(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
        where Self: Sized;

    fn label() -> String;
}

impl GenData for i32 {

    fn uniform(rng: &mut XorShiftRng, len: usize) -> Vec<Self> {
        rng.sample_iter(&Standard).take(len).collect()
    }

    fn label() -> String {
        return "i32".to_string()
    }
}

impl GenData for i64 {

    fn uniform(rng: &mut XorShiftRng, len: usize) -> Vec<Self> {
        rng.sample_iter(&Standard).take(len).collect()
    }

    fn label() -> String {
        return "i64".to_string()
    }
}

fn run<T, SortAlgo, Dist>(mut sorter: SortAlgo, sorter_label: &str, dist: Dist, dist_label: &str, parallelism: bool) 
where T: GenData + Send + Ord, SortAlgo: Fn(&mut [T]), Dist: Fn(&mut XorShiftRng, usize) -> Vec<T>
{
    const MAX_LOG_N: usize = 26;
    const ITERATIONS: usize = 10;
    const SEED: u64 = 2020;

    for log_n in 0..=MAX_LOG_N {
        let mut times = Vec::new();
        let mut rng = XorShiftRng::seed_from_u64(SEED);
        let len = 1 << log_n;
        for _i in 0..ITERATIONS {
            let mut v = dist(&mut rng, len);
            let now = Instant::now();
            sorter(&mut v[..]);
            let elapsed = now.elapsed();
            times.push(elapsed);
        }

        println!(
            "RESULT algo={} dist={} size={} iters={} time={} parallelism={} type={}", 
            sorter_label, dist_label, len, ITERATIONS, 
            mean(&times[..]).unwrap(),
            parallelism,
            T::label()
        )
    }
}

fn benchmark<T, SortAlgo>(mut sorter: SortAlgo, sorter_label: &str, parallelism: bool) 
where T: GenData + Send + Ord, SortAlgo: Fn(&mut [T])
{
    run(sorter, sorter_label, T::uniform, "uniform", parallelism);
}

fn main() {
    {
        let parallelism = true;
        let sorter_label = "rayon::par_sort";
        benchmark::<i32, _>(&rayon::slice::ParallelSliceMut::par_sort, sorter_label, parallelism);
        benchmark::<i64, _>(&rayon::slice::ParallelSliceMut::par_sort, sorter_label, parallelism);
    }
    {
        let parallelism = true;
        let sorter_label = "rayon::par_sort_unstable";
        benchmark::<i32, _>(&rayon::slice::ParallelSliceMut::par_sort_unstable, sorter_label, parallelism);
        benchmark::<i64, _>(&rayon::slice::ParallelSliceMut::par_sort_unstable, sorter_label, parallelism);
    }
    {
        let parallelism = true;
        let sorter_label = "ips4o::par_sort_unstable";
        benchmark::<i32, _>(&ips4o::par_sort_unstable, sorter_label, parallelism);
        benchmark::<i64, _>(&ips4o::par_sort_unstable, sorter_label, parallelism);
    }
}
