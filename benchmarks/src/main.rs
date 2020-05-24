use rand::seq::SliceRandom;
use rand::distributions::{Alphanumeric, Standard};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use rayon::prelude::*;
use std::time::{Duration, Instant};
use std::ops::Add;

use partial_function;

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

    fn near_sort(rng: &mut XorShiftRng, len: usize, fraction: f32) -> Vec<Self>
    where Self: Sized;

    fn near_sort50(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized 
    {
        Self::near_sort(rng, len, 0.50)
    }

    fn label() -> String;
}

impl GenData for i32 {

    fn uniform(rng: &mut XorShiftRng, len: usize) -> Vec<Self> {
        rng.sample_iter(&Standard).take(len).collect()
    }

    fn near_sort(rng: &mut XorShiftRng, len: usize, fraction: f32) -> Vec<Self>
    where Self: Sized
    {
        let mut v: Vec<Self> = (0..len).map(|i| i as Self).collect();

        for i in (0..len).step_by((len as f32 / fraction) as usize) {
            v[i] = rng.sample(&Standard);
        }
        v
    }

    fn label() -> String {
        return "i32".to_string()
    }
}

impl GenData for i64 {

    fn uniform(rng: &mut XorShiftRng, len: usize) -> Vec<Self> {
        rng.sample_iter(&Standard).take(len).collect()
    }

    fn near_sort(rng: &mut XorShiftRng, len: usize, fraction: f32) -> Vec<Self>
    where Self: Sized
    {
        let mut v: Vec<Self> = (0..len).map(|i| i as Self).collect();

        for i in (0..len).step_by((len as f32 / fraction) as usize) {
            v[i] = rng.sample(&Standard);
        }
        v
    }

    fn label() -> String {
        return "i64".to_string()
    }
}

fn run<T, SortAlgo, Dist>(sorter: SortAlgo, sorter_label: &str, dist: Dist, dist_label: &str, parallelism: bool) 
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

fn benchmark<T, SortAlgo>(sorter: SortAlgo, sorter_label: &str, parallelism: bool) 
where T: GenData + Send + Ord, SortAlgo: Fn(&mut [T]) + Clone
{
    run(sorter.clone(), sorter_label, T::uniform, "uniform", parallelism);
    run(sorter.clone(), sorter_label, T::near_sort50, "sort50", parallelism);
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
    {
        let parallelism = false;
        let sorter_label = "std::sort";
        benchmark::<i32, _>(&(<[i32]>::sort), sorter_label, parallelism);
        benchmark::<i64, _>(&(<[i64]>::sort), sorter_label, parallelism);
    }
    {
        let parallelism = false;
        let sorter_label = "std::sort_unstable";
        benchmark::<i32, _>(&(<[i32]>::sort_unstable), sorter_label, parallelism);
        benchmark::<i64, _>(&(<[i64]>::sort_unstable), sorter_label, parallelism);
    }
    {
        let parallelism = false;
        let sorter_label = "ips4o::sort_unstable";
        benchmark::<i32, _>(&ips4o::sort_unstable, sorter_label, parallelism);
        benchmark::<i64, _>(&ips4o::sort_unstable, sorter_label, parallelism);
    }
}
