use rand::seq::SliceRandom;
use rand::distributions::{Alphanumeric, Standard};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use rayon::prelude::*;
use std::time::{Duration, Instant};
use std::ops::Add;

use ips4o;

/*
fn gen_random<T>(rng: &mut T, len: usize) -> Vec<u32>
where T: Rng 
{
    rng.sample_iter(&Standard).take(len).collect()
}

fn benchmark<T>(rng: &mut T, f: &fn(&mut T, usize) -> Vec<u32>, log_n: usize, iterations: usize)
where T: Rng 
{
    let n = 1 << log_n;

    let mut par_sort_times = Vec::new();
    let mut par_sort_unstable_times = Vec::new();
    let mut ips4o_times = Vec::new();

    for _i in 0..iterations {
        let v = gen_random(rng, n);

        {
            let mut v_copy = v.clone();
            let now = Instant::now();
            v_copy.par_sort();
            let elapsed = now.elapsed();
            par_sort_times.push(elapsed);
        }

        {
            let mut v_copy = v.clone();
            let now = Instant::now();
            v_copy.par_sort_unstable();
            let elapsed = now.elapsed();
            par_sort_unstable_times.push(elapsed);
        }

        {
            let mut v_copy = v.clone();
            let now = Instant::now();
            ips4o::sort(&mut v_copy);
            let elapsed = now.elapsed();
            ips4o_times.push(elapsed);
        }

    }
    write_benchmark("par_sort", "random", n, iterations, par_sort_times);
    write_benchmark("par_sort_unstable", "random", n, iterations, par_sort_unstable_times);
    write_benchmark("ips4o", "random", n, iterations, ips4o_times);
}*/

fn mean(data: &[Duration]) -> Option<f32>
{
    let sum = data.iter().sum::<Duration>().as_nanos() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &[Duration]) -> Option<f32> 
{
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - value.as_nanos() as f32;

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}

trait GenData {

    fn uniform(rng: &mut XorShiftRng, size: usize) -> Vec<Self>
        where Self: Sized;

    fn label() -> String;
}

impl GenData for i32 {

    fn uniform(rng: &mut XorShiftRng, size: usize) -> Vec<Self> {
        vec!()
    }

    fn label() -> String {
        return "i32".to_string()
    }
}

impl GenData for i64 {

    fn uniform(rng: &mut XorShiftRng, size: usize) -> Vec<Self> {
        vec!()
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
            "RESULT algo={} dist={} size={} iters={} time={} stddev={} parallelism={}", 
            sorter_label, dist_label, len, ITERATIONS, 
            mean(&times[..]).unwrap(), 
            std_deviation(&times[..]).unwrap(),
            parallelism
        )
    }
}

fn benchmark<T, SortAlgo>(mut sorter: SortAlgo, sorter_label: &str, parallelism: bool) 
where T: GenData + Send + Ord, SortAlgo: Fn(&mut [T])
{
    /*let mut times = Vec::new();

    for _i in 0..iterations {
        let v = gen_random(rng, n);

        {
            let mut v_copy = v.clone();
            let now = Instant::now();
            v_copy.par_sort();
            let elapsed = now.elapsed();
            par_sort_times.push(elapsed);
        }


    println!(
        "RESULT algo={} dist={} size={} iters={} time={} stddev={} parallelism={}", 
        algo, dist, size, iterations, 
        mean(&times[..]).unwrap(), 
        std_deviation(&times[..]).unwrap(),
        parallelism
    );*/

    run(sorter, sorter_label, T::uniform, "uniform", parallelism);

}

fn benchmarks()
{
    /*const MAX_LOGN: usize = 26;
    const ITERATIONS: usize = 10;
    const SEED: u64 = 2020;

    for logn in 0..MAX_LOGN {
        {
            let mut rng = XorShiftRng::seed_from_u64(SEED);
            for _i in 0..ITERATIONS {
                let mut v = T::uniform();
                let f = rayon::slice::ParallelSliceMut::par_sort;
                f(&mut v[..]);
            }
        }


    }
    
    const MAX_LOG_N: usize = 26;
    const ITERATIONS: usize = 10;
    const SEED: u64 = 2020;

    for log_n in 0..MAX_LOG_N {


        let size = 1 << log_n;
    
    */

    let algo = "rayon::par_sort";
    let parallelism = true;
    benchmark::<i32, _>(&rayon::slice::ParallelSliceMut::par_sort, algo, parallelism);
    benchmark::<i64, _>(&rayon::slice::ParallelSliceMut::par_sort, algo, parallelism);
    
    
}

fn main() {    
    /*let mut rng = XorShiftRng::seed_from_u64(2020);
    const ITERATIONS: usize = 10;
    const MAX_LOG_SIZE: usize = 26;

    let distribution_gens: Vec<&dyn Fn(&mut XorShiftRng, usize) -> Vec<u32>> = vec![&gen_random];

    for log_n in 0..=MAX_LOG_SIZE {
        for f in &distribution_gens {
            benchmark(&mut rng, f, log_n, ITERATIONS);
        }
    }*/    
    benchmarks();

}
