//! Benchmark runner: sweeps sorters × element types × distributions × sizes
//! and writes one CSV row per combination.
//!
//! Every sorter in a row group sorts the same input. Each measurement copies
//! the input into a reused buffer (not timed), sorts it (timed), and checks
//! the result once. Rows report the median and minimum over repetitions, and
//! the median in nanoseconds per `n log2 n`, the IPS4o paper's metric.
//!
//! See `benchmarks/README.md` for usage.

mod inputs;

/// `generic` or `native` (built with `-C target-cpu=native`); set by build.rs.
const BUILD: &str = env!("BENCH_BUILD");

use std::io::{self, Write};
use std::process::ExitCode;
use std::time::{Duration, Instant};

use inputs::{Distribution, Element, Pair, Record100, Rng};

const USAGE: &str = "\
usage: benchmarks [options]

  --preset NAME      quick: 5 distributions, sizes 2^12..2^24 step 4, sorters
                     ips4o,std_unstable,cpp_ips4o, min-time 0.5 (about 5 min for
                     all types); full: the defaults below. Later options override.
  --types LIST       element types: u64,f64,pair,record100,string (default: u64)
  --dists LIST       distributions (default: all): uniform, exponential,
                     almost_sorted, root_dup, two_dup, eight_dup, sorted,
                     reverse_sorted, all_equal
  --sorters LIST     only these sorters (default: all): ips4o, std_unstable,
                     std_stable, cpp_ips4o, cpp_std_sort
  --min-log N        smallest size 2^N (default: 10)
  --max-log N        largest size 2^N (default: 26)
  --log-step N       step between sizes, in powers of two (default: 2)
  --max-bytes N      skip inputs larger than N bytes (default: 4294967296)
  --min-time SECS    repeat each measurement for at least this long (default: 1)
  --min-reps N       at least this many repetitions (default: 3)
  --max-reps N       at most this many repetitions (default: 50)
  --seed N           input seed (default: 1)
  --out PATH         write CSV here instead of stdout
";

struct Options {
    types: Vec<String>,
    dists: Vec<Distribution>,
    sorters: Option<Vec<String>>,
    min_log: u32,
    max_log: u32,
    log_step: u32,
    max_bytes: usize,
    min_time: Duration,
    min_reps: usize,
    max_reps: usize,
    seed: u64,
    out: Option<String>,
}

impl Options {
    fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut opts = Options {
            types: vec!["u64".into()],
            dists: Distribution::ALL.to_vec(),
            sorters: None,
            min_log: 10,
            max_log: 26,
            log_step: 2,
            max_bytes: 1 << 32,
            min_time: Duration::from_secs(1),
            min_reps: 3,
            max_reps: 50,
            seed: 1,
            out: None,
        };
        let list = |s: String| s.split(',').map(str::to_owned).collect::<Vec<_>>();
        while let Some(flag) = args.next() {
            if flag == "--help" || flag == "-h" {
                return Err(String::new());
            }
            let value = args.next().ok_or(format!("{flag} needs a value"))?;
            let number = |v: &str| {
                v.parse::<u64>()
                    .map_err(|_| format!("{flag}: not a number: {v}"))
            };
            match flag.as_str() {
                "--preset" => match value.as_str() {
                    "quick" => {
                        opts.dists = [
                            Distribution::Uniform,
                            Distribution::Exponential,
                            Distribution::RootDup,
                            Distribution::TwoDup,
                            Distribution::AlmostSorted,
                        ]
                        .to_vec();
                        opts.sorters = Some(list("ips4o,std_unstable,cpp_ips4o".into()));
                        (opts.min_log, opts.max_log, opts.log_step) = (12, 24, 4);
                        opts.min_time = Duration::from_secs_f64(0.5);
                    }
                    "full" => {}
                    _ => return Err(format!("unknown preset: {value}")),
                },
                "--types" => opts.types = list(value),
                "--dists" => {
                    opts.dists = list(value)
                        .iter()
                        .map(|d| Distribution::parse(d).ok_or(format!("unknown distribution: {d}")))
                        .collect::<Result<_, _>>()?
                }
                "--sorters" => opts.sorters = Some(list(value)),
                "--min-log" => opts.min_log = number(&value)? as u32,
                "--max-log" => opts.max_log = number(&value)? as u32,
                "--log-step" => opts.log_step = number(&value)?.max(1) as u32,
                "--max-bytes" => opts.max_bytes = number(&value)? as usize,
                "--min-time" => {
                    let secs: f64 = value
                        .parse()
                        .map_err(|_| format!("--min-time: not a number: {value}"))?;
                    opts.min_time = Duration::from_secs_f64(secs);
                }
                "--min-reps" => opts.min_reps = number(&value)?.max(1) as usize,
                "--max-reps" => opts.max_reps = number(&value)?.max(1) as usize,
                "--seed" => opts.seed = number(&value)?,
                "--out" => opts.out = Some(value),
                _ => return Err(format!("unknown option: {flag}")),
            }
        }
        Ok(opts)
    }
}

fn main() -> ExitCode {
    let opts = match Options::parse(std::env::args().skip(1)) {
        Ok(opts) => opts,
        Err(message) => {
            if !message.is_empty() {
                eprintln!("error: {message}\n");
            }
            eprint!("{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    let mut out: Box<dyn Write> = match &opts.out {
        Some(path) => match std::fs::File::create(path) {
            Ok(file) => Box::new(io::BufWriter::new(file)),
            Err(e) => {
                eprintln!("error: cannot create {path}: {e}");
                return ExitCode::FAILURE;
            }
        },
        None => Box::new(io::stdout()),
    };
    writeln!(
        out,
        "build,type,distribution,n,sorter,reps,median_ns,min_ns,ns_per_n_log_n"
    )
    .unwrap();
    eprintln!("build: {BUILD}");

    for name in &opts.types {
        let result = match name.as_str() {
            "u64" => run::<u64>(&opts, &mut out),
            "f64" => run::<f64>(&opts, &mut out),
            "pair" => run::<Pair>(&opts, &mut out),
            "record100" => run::<Record100>(&opts, &mut out),
            "string" => run::<String>(&opts, &mut out),
            _ => Err(format!("unknown type: {name}")),
        };
        if let Err(message) = result {
            eprintln!("error: {message}");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

/// Approximate memory per element, counting a `String`'s heap buffer.
fn bytes_per_element<T: Element>() -> usize {
    size_of::<T>() + if T::NAME == "string" { 16 } else { 0 }
}

fn run<T: Element>(opts: &Options, out: &mut dyn Write) -> Result<(), String> {
    let sorters: Vec<_> = T::sorters()
        .into_iter()
        .filter(|s| {
            opts.sorters
                .as_ref()
                .is_none_or(|names| names.iter().any(|n| n == s.name))
        })
        .collect();
    if sorters.is_empty() {
        eprintln!("note: no selected sorters support {}; skipping", T::NAME);
        return Ok(());
    }

    for &dist in &opts.dists {
        for log in (opts.min_log..=opts.max_log).step_by(opts.log_step as usize) {
            let n = 1usize << log;
            // Input plus the working copy.
            if 2 * n * bytes_per_element::<T>() > opts.max_bytes {
                eprintln!("note: skipping {} n=2^{log}: over --max-bytes", T::NAME);
                continue;
            }
            let mut rng = Rng::new(opts.seed ^ (log as u64) << 32);
            let input: Vec<T> = dist
                .keys(n, &mut rng)
                .into_iter()
                .map(|k| T::from_key(k, &mut rng))
                .collect();
            let mut work = input.clone();

            for (sorter, times) in sorters
                .iter()
                .zip(measure(opts, &sorters, &input, &mut work)?)
            {
                let median = times[times.len() / 2];
                let min = times[0];
                let per = median as f64 / (n as f64 * (n as f64).log2());
                writeln!(
                    out,
                    "{BUILD},{},{},{n},{},{},{median},{min},{per:.4}",
                    T::NAME,
                    dist.name(),
                    sorter.name,
                    times.len()
                )
                .unwrap();
                out.flush().unwrap();
                eprintln!(
                    "{:<9} {:<14} n=2^{log:<2} {:<13} {per:>7.3} ns/(n log n)  ({} reps)",
                    T::NAME,
                    dist.name(),
                    sorter.name,
                    times.len()
                );
            }
        }
    }
    Ok(())
}

/// Times every sorter on `input`, returning each one's timings (in
/// nanoseconds, sorted).
///
/// Repetitions alternate between sorters (one rep of each, then the next
/// round), so a burst of background activity lands on all of them alike and
/// the comparison between them stays fair. Each sorter stops once it has
/// `min_reps` reps and either `min_time` in total or `max_reps` reps.
fn measure<T: Element>(
    opts: &Options,
    sorters: &[inputs::Sorter<T>],
    input: &[T],
    work: &mut [T],
) -> Result<Vec<Vec<u128>>, String> {
    let mut times = vec![Vec::new(); sorters.len()];
    let mut totals = vec![Duration::ZERO; sorters.len()];
    let wants_more = |times: &Vec<u128>, total: Duration| {
        times.len() < opts.min_reps || (total < opts.min_time && times.len() < opts.max_reps)
    };
    while (0..sorters.len()).any(|i| wants_more(&times[i], totals[i])) {
        for (i, sorter) in sorters.iter().enumerate() {
            if !wants_more(&times[i], totals[i]) {
                continue;
            }
            work.clone_from_slice(input);
            let start = Instant::now();
            (sorter.sort)(std::hint::black_box(&mut *work));
            let elapsed = start.elapsed();
            if times[i].is_empty() && !work.is_sorted_by(|a, b| T::compare(a, b).is_le()) {
                return Err(format!(
                    "{} produced unsorted output for {}",
                    sorter.name,
                    T::NAME
                ));
            }
            totals[i] += elapsed;
            times[i].push(elapsed.as_nanos());
        }
    }
    for t in &mut times {
        t.sort_unstable();
    }
    Ok(times)
}
