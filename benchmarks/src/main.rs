use rand::seq::SliceRandom;
use rand::distributions::{Alphanumeric, Standard};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use rayon::prelude::*;
use std::time::{Duration, Instant};
use std::ops::Add;

use ips4o;


fn gen_random<T>(rng: &mut T, len: usize) -> Vec<u32>
where T: Rng 
{
    rng.sample_iter(&Standard).take(len).collect()
}

fn benchmark<T>(rng: &mut T, log_n: usize, iterations: usize)
where T: Rng 
{
    let n = 1 << log_n;

    let mut par_sort_times = Vec::new();
    let mut par_sort_unstable_times = Vec::new();

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

    }
    write_benchmark("par_sort", "random", n, iterations, par_sort_times);
    write_benchmark("par_sort_unstable", "random", n, iterations, par_sort_unstable_times);

}

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

fn write_benchmark(algo: &str, dist: &str, size: usize, iterations: usize, times: Vec<Duration>) {
    println!("RESULT algo={} dist={} size={} iters={} time={} stddev={} ", algo, dist, size, iterations, mean(&times[..]).unwrap(), std_deviation(&times[..]).unwrap());
}

fn main() {    
    let mut rng = XorShiftRng::seed_from_u64(2020);
    const ITERATIONS: usize = 10;
    const MAX_LOG_SIZE: usize = 26;

    for log_n in 0..=MAX_LOG_SIZE {
        benchmark(&mut rng, log_n, ITERATIONS);
    }

}
