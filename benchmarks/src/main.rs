use rand::distributions::{Standard};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::time::{Duration, Instant};

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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct NormalString(String);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct BigString(String);

impl NormalString {

    fn from_i64(i: i64) -> Self
    {
        Self(format!("{:064}", i))
    }
}

impl BigString {

    fn from_i64(i: i64) -> Self
    {
        Self(format!("{:01000}", i))
    }
}

trait GenData {

    fn uniform(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized;

    fn near_sort(rng: &mut XorShiftRng, len: usize, fraction: f64) -> Vec<Self>
    where Self: Sized;

    fn near_sort50(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized 
    {
        Self::near_sort(rng, len, 0.50)
    }

    fn near_sort90(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized 
    {
        Self::near_sort(rng, len, 0.90)
    }

    fn near_sort99(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized 
    {
        Self::near_sort(rng, len, 0.99)
    }

    fn asc(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized;

    fn desc(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized;

    fn dupsq(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized;

    fn dup8(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized;

    fn mod8(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized;

    fn ones(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized;

    fn organ(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized;

    fn merge(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized;

    fn label() -> String;
}

impl GenData for i32 {

    fn uniform(rng: &mut XorShiftRng, len: usize) -> Vec<Self> {
        rng.sample_iter(&Standard).take(len).collect()
    }

    fn near_sort(rng: &mut XorShiftRng, len: usize, sorted_fraction: f64) -> Vec<Self>
    {
        let mut v: Vec<Self> = (0..len).map(|i| i as Self).collect();

        for i in 0..len {
            if !rng.gen_bool(sorted_fraction) {
                v[i] = rng.sample(&Standard);
            }
        }
        v
    }

    fn asc(_rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        let mut i = Self::min_value();
        let mut v = Vec::with_capacity(len);

        for _i in 0..len {
            v.push(i);
            i = i.saturating_add(1);
        }
        v
    }

    fn desc(_rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        let mut i = Self::max_value();
        let mut v = Vec::with_capacity(len);

        for _i in 0..len {
            v.push(i);
            i = i.saturating_sub(1);
        }
        v
    }

    fn dupsq(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        let sq = (len as f64).sqrt() as Self;
        rng.sample_iter(&Standard).map(|i: Self| i % sq).take(len).collect()
    }

    fn dup8(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {        
        let half_n = (len >> 1).min(Self::max_value as usize) as Self;
        let mod_n = len.min(Self::max_value as usize) as Self;
        rng.sample_iter(&Standard).map(|i: Self| (i.wrapping_pow(8) + half_n) % mod_n).take(len).collect()
    }

    fn mod8(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        rng.sample_iter(&Standard).map(|i: Self| i % 8).take(len).collect()
    }

    fn ones(_rng: &mut XorShiftRng, len: usize) -> Vec<Self> {
        vec![1; len]
    }

    fn organ(_rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        let mut i = Self::min_value();
        let mut v = Vec::with_capacity(len);

        let first_half = (len / 2) + 1;
        for _i in 0..first_half {
            v.push(i);
            i = i.saturating_add(1);
        }

        while v.len() < len {
            v.push(i);
            i = i.saturating_sub(1);
        }
        v
    }

    fn merge(_rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        let mut i = Self::min_value();
        let mut v = Vec::with_capacity(len);

        let first_half = (len / 2) + 1;
        for _i in 0..first_half {
            v.push(i);
            i = i.saturating_add(1);
        }
        let mut v2 = v.clone();
        v.append(&mut v2);
        v.truncate(len);
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

    fn near_sort(rng: &mut XorShiftRng, len: usize, sorted_fraction: f64) -> Vec<Self>
    {
        let mut v: Vec<Self> = (0..len).map(|i| i as Self).collect();

        for i in 0..len {
            if !rng.gen_bool(sorted_fraction) {
                v[i] = rng.sample(&Standard);
            }
        }
        v
    }

    fn asc(_rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        let mut i = Self::min_value();
        let mut v = Vec::with_capacity(len);

        for _i in 0..len {
            v.push(i);
            i = i.saturating_add(1);
        }
        v
    }

    fn desc(_rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        let mut i = Self::max_value();
        let mut v = Vec::with_capacity(len);

        for _i in 0..len {
            v.push(i);
            i = i.saturating_sub(1);
        }
        v
    }

    fn dupsq(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    where Self: Sized
    {
        let sq = (len as f64).sqrt() as Self;
        rng.sample_iter(&Standard).map(|i: Self| i % sq).take(len).collect()
    }

    fn dup8(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {        
        let half_n = (len >> 1).min(Self::max_value as usize) as Self;
        let mod_n = len.min(Self::max_value as usize) as Self;
        rng.sample_iter(&Standard).map(|i: Self| (i.wrapping_pow(8) + half_n) % mod_n).take(len).collect()
    }

    fn mod8(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        rng.sample_iter(&Standard).map(|i: Self| i % 8).take(len).collect()
    }

    fn ones(_rng: &mut XorShiftRng, len: usize) -> Vec<Self> {
        vec![1; len]
    }

    fn organ(_rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        let mut i = Self::min_value();
        let mut v = Vec::with_capacity(len);

        let first_half = (len / 2) + 1;
        for _i in 0..first_half {
            v.push(i);
            i = i.saturating_add(1);
        }

        while v.len() < len {
            v.push(i);
            i = i.saturating_sub(1);
        }
        v
    }

    fn merge(_rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        let mut i = Self::min_value();
        let mut v = Vec::with_capacity(len);

        let first_half = (len / 2) + 1;
        for _i in 0..first_half {
            v.push(i);
            i = i.saturating_add(1);
        }
        let mut v2 = v.clone();
        v.append(&mut v2);
        v.truncate(len);
        v

    }

    fn label() -> String {
        return "i64".to_string()
    }
}

impl GenData for NormalString {

    fn uniform(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::uniform(rng, len).into_iter().map(|i| NormalString::from_i64(i)).collect()
    }

    fn near_sort(rng: &mut XorShiftRng, len: usize, fraction: f64) -> Vec<Self>
    {
        i64::near_sort(rng, len, fraction).into_iter().map(|i| NormalString::from_i64(i)).collect()
    }

    fn asc(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::asc(rng, len).into_iter().map(|i| NormalString::from_i64(i)).collect()
    }

    fn desc(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::desc(rng, len).into_iter().map(|i| NormalString::from_i64(i)).collect()
    }

    fn dupsq(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::dupsq(rng, len).into_iter().map(|i| NormalString::from_i64(i)).collect()
    }

    fn dup8(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::dup8(rng, len).into_iter().map(|i| NormalString::from_i64(i)).collect()
    }
    fn mod8(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::mod8(rng, len).into_iter().map(|i| NormalString::from_i64(i)).collect()
    }
    fn ones(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::ones(rng, len).into_iter().map(|i| NormalString::from_i64(i)).collect()
    }
    fn organ(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::organ(rng, len).into_iter().map(|i| NormalString::from_i64(i)).collect()
    }
    fn merge(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::merge(rng, len).into_iter().map(|i| NormalString::from_i64(i)).collect()
    }
    fn label() -> String
    {
        "String".to_string()
    }
}

impl GenData for BigString {

    fn uniform(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::uniform(rng, len).into_iter().map(|i| BigString::from_i64(i)).collect()
    }

    fn near_sort(rng: &mut XorShiftRng, len: usize, fraction: f64) -> Vec<Self>
    {
        i64::near_sort(rng, len, fraction).into_iter().map(|i| BigString::from_i64(i)).collect()
    }

    fn asc(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::asc(rng, len).into_iter().map(|i| BigString::from_i64(i)).collect()
    }

    fn desc(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::desc(rng, len).into_iter().map(|i| BigString::from_i64(i)).collect()
    }

    fn dupsq(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::dupsq(rng, len).into_iter().map(|i| BigString::from_i64(i)).collect()
    }

    fn dup8(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::dup8(rng, len).into_iter().map(|i| BigString::from_i64(i)).collect()
    }
    fn mod8(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::mod8(rng, len).into_iter().map(|i| BigString::from_i64(i)).collect()
    }
    fn ones(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::ones(rng, len).into_iter().map(|i| BigString::from_i64(i)).collect()
    }
    fn organ(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::organ(rng, len).into_iter().map(|i| BigString::from_i64(i)).collect()
    }
    fn merge(rng: &mut XorShiftRng, len: usize) -> Vec<Self>
    {
        i64::merge(rng, len).into_iter().map(|i| BigString::from_i64(i)).collect()
    }
    fn label() -> String
    {
        "BigString".to_string()
    }
}

fn run<T, SortAlgo, Dist>(sorter: SortAlgo, sorter_label: &str, dist: Dist, dist_label: &str, parallelism: bool, max_log_n: usize) 
where T: GenData + Send + Ord, SortAlgo: Fn(&mut [T]), Dist: Fn(&mut XorShiftRng, usize) -> Vec<T>
{
    const ITERATIONS: usize = 10;
    const SEED: u64 = 2020;

    for log_n in 0..=max_log_n {
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

fn benchmark<T, SortAlgo>(sorter: SortAlgo, sorter_label: &str, parallelism: bool, max_log_n: usize) 
where T: GenData + Send + Ord, SortAlgo: Fn(&mut [T]) + Clone
{
    run(sorter.clone(), sorter_label, T::uniform, "uniform", parallelism, max_log_n);
    run(sorter.clone(), sorter_label, T::near_sort50, "sort50", parallelism, max_log_n);
    run(sorter.clone(), sorter_label, T::near_sort90, "sort90", parallelism, max_log_n);
    run(sorter.clone(), sorter_label, T::near_sort99, "sort99", parallelism, max_log_n);
    run(sorter.clone(), sorter_label, T::asc, "asc", parallelism, max_log_n);
    run(sorter.clone(), sorter_label, T::desc, "desc", parallelism, max_log_n);
    run(sorter.clone(), sorter_label, T::dupsq, "dupsq", parallelism, max_log_n);
    run(sorter.clone(), sorter_label, T::dup8, "dup8", parallelism, max_log_n);
    run(sorter.clone(), sorter_label, T::mod8, "mod8", parallelism, max_log_n);
    run(sorter.clone(), sorter_label, T::ones, "ones", parallelism, max_log_n);
    run(sorter.clone(), sorter_label, T::organ, "organ", parallelism, max_log_n);
    run(sorter.clone(), sorter_label, T::merge, "merge", parallelism, max_log_n);
}

fn main() {
    const MAX_LOG_N_I32: usize = 20;
    const MAX_LOG_N_I64: usize = 20;
    const MAX_LOG_N_NORMALSTRING: usize = 16;
    const MAX_LOG_N_BIGSTRING: usize = 12;
    {
        let parallelism = true;
        let sorter_label = "rayon::par_sort";
        benchmark::<i32, _>(&rayon::slice::ParallelSliceMut::par_sort, sorter_label, parallelism, MAX_LOG_N_I32);
        benchmark::<i64, _>(&rayon::slice::ParallelSliceMut::par_sort, sorter_label, parallelism, MAX_LOG_N_I64);
        benchmark::<NormalString, _>(&rayon::slice::ParallelSliceMut::par_sort, sorter_label, parallelism, MAX_LOG_N_NORMALSTRING);
        benchmark::<BigString, _>(&rayon::slice::ParallelSliceMut::par_sort, sorter_label, parallelism, MAX_LOG_N_BIGSTRING);

    }
    {
        let parallelism = true;
        let sorter_label = "rayon::par_sort_unstable";
        benchmark::<i32, _>(&rayon::slice::ParallelSliceMut::par_sort_unstable, sorter_label, parallelism, MAX_LOG_N_I32);
        benchmark::<i64, _>(&rayon::slice::ParallelSliceMut::par_sort_unstable, sorter_label, parallelism, MAX_LOG_N_I64);
        benchmark::<NormalString, _>(&rayon::slice::ParallelSliceMut::par_sort_unstable, sorter_label, parallelism, MAX_LOG_N_NORMALSTRING);
        benchmark::<BigString, _>(&rayon::slice::ParallelSliceMut::par_sort_unstable, sorter_label, parallelism, MAX_LOG_N_BIGSTRING);
    }
    {
        let parallelism = true;
        let sorter_label = "ips4o::par_sort_unstable";
        benchmark::<i32, _>(&ips4o::par_sort_unstable, sorter_label, parallelism, MAX_LOG_N_I32);
        benchmark::<i64, _>(&ips4o::par_sort_unstable, sorter_label, parallelism, MAX_LOG_N_I64);
        benchmark::<NormalString, _>(&ips4o::par_sort_unstable, sorter_label, parallelism, MAX_LOG_N_NORMALSTRING);
        benchmark::<BigString, _>(&ips4o::par_sort_unstable, sorter_label, parallelism, MAX_LOG_N_BIGSTRING);
    }
    {
        let parallelism = false;
        let sorter_label = "std::sort";
        benchmark::<i32, _>(&(<[_]>::sort), sorter_label, parallelism, MAX_LOG_N_I32);
        benchmark::<i64, _>(&(<[_]>::sort), sorter_label, parallelism, MAX_LOG_N_I64);
        benchmark::<NormalString, _>(&(<[_]>::sort), sorter_label, parallelism, MAX_LOG_N_NORMALSTRING);
        benchmark::<BigString, _>(&(<[_]>::sort), sorter_label, parallelism, MAX_LOG_N_BIGSTRING);
    }
    {
        let parallelism = false;
        let sorter_label = "std::sort_unstable";
        benchmark::<i32, _>(&(<[_]>::sort_unstable), sorter_label, parallelism, MAX_LOG_N_I32);
        benchmark::<i64, _>(&(<[_]>::sort_unstable), sorter_label, parallelism, MAX_LOG_N_I64);
        benchmark::<NormalString, _>(&(<[_]>::sort_unstable), sorter_label, parallelism, MAX_LOG_N_NORMALSTRING);
        benchmark::<BigString, _>(&(<[_]>::sort_unstable), sorter_label, parallelism, MAX_LOG_N_BIGSTRING);
    }
    {
        let parallelism = false;
        let sorter_label = "ips4o::sort_unstable";
        benchmark::<i32, _>(&ips4o::sort_unstable, sorter_label, parallelism, MAX_LOG_N_I32);
        benchmark::<i64, _>(&ips4o::sort_unstable, sorter_label, parallelism, MAX_LOG_N_I64);
        benchmark::<NormalString, _>(&ips4o::sort_unstable, sorter_label, parallelism, MAX_LOG_N_NORMALSTRING);
        benchmark::<BigString, _>(&ips4o::sort_unstable, sorter_label, parallelism, MAX_LOG_N_BIGSTRING);
    }
}
