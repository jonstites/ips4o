# Benchmarks

A runner that times sorters over element types × input distributions × sizes
and writes CSV, plus a script that plots it.

## Sorters

| name | what |
|---|---|
| `ips4o` | this crate's `sort_unstable_by` |
| `std_unstable` | Rust's `sort_unstable_by` (ipnsort) |
| `std_stable` | Rust's `sort_by` (driftsort): stable, so a different contract |
| `cpp_ips4o` | the original C++ IPS⁴o, sequential (`--features cpp`) |
| `cpp_std_sort` | C++ `std::sort` (`--features cpp`) |
| `rayon_par` | rayon's `par_sort_unstable_by` (parallel pdqsort) |
| `cpp_ips4o_par` | the C++ IPS⁴o, parallel (`--features cpp`); its sorter and thread pool are created once and reused, like rayon's pool |

Parallel sorters use `--threads` threads (default: all); the CSV records it,
and `plot.py` puts parallel results in their own figures.

The C++ sorters cover `u64`, `f64`, `pair` and `record100`. Strings are
Rust-only.

## Running

The C++ comparison needs the submodule, a C++17 compiler, and TBB (the C++
IPS⁴o includes its headers even for the sequential sort, and the parallel sort
links it):

```sh
git submodule update --init
sudo apt install libtbb-dev
```

### Two builds: generic and native

Results depend on how the code is compiled, sometimes a lot: auto-vectorization
for AVX-512 once made a change 75% slower in a native build while it was 15%
faster in a generic one. So we benchmark two builds:

- **generic** (primary): default flags, what users of the crate get. Run the
  full sweep.
- **native**: `-C target-cpu=native`, with the C++ built with `-march=native`
  to match. Run a smaller sweep as a regression check.

`run.sh` builds both into separate target directories and runs them pinned to
one core (`BENCH_CPU`, default 2). Every CSV row records its build.

Use the **quick preset** for routine checks: five distributions, sizes 2¹²
to 2²⁴, and the three sorters that matter for tuning (about 5 minutes for all
types). Keep **full sweeps** (all distributions and sorters, up to 2²⁸) for
milestones, ideally while you are away from the machine; they take about 1.5
hours for all types, and each type can be run separately.

```sh
# routine: generic, all types
BENCH_BUILDS=generic benchmarks/run.sh quick --preset quick --types u64,pair,record100,string
# routine: native regression check
BENCH_BUILDS=native benchmarks/run.sh quick --preset quick --types u64,string
# parallel: don't pin to one core
BENCH_CPU=all BENCH_BUILDS=generic benchmarks/run.sh par --preset parallel --types u64,string
# milestone: one type at a time, both builds
benchmarks/run.sh full-u64 --types u64 --max-log 28
uv run benchmarks/plot.py benchmarks/results/quick-*.csv
```

For a single build without the script, the build script compiles the C++ with
`-march=native` exactly when Rust gets `-C target-cpu=native`:

```sh
cargo run -p benchmarks --release --features cpp -- --types u64 --out benchmarks/results/u64.csv
```

`--help` lists every option. Useful ones: `--types`, `--dists`, `--sorters`,
`--min-log`/`--max-log`/`--log-step` (sizes as powers of two),
`--max-bytes` (skips sizes that don't fit), `--min-time` (per measurement).

Results go to `benchmarks/results/`, which is git-ignored.

## Getting trustworthy numbers

- **Go past the cache.** Sizes up to the L3 cache measure an in-cache sort,
  not the memory-bound regime IPS⁴o is designed for. With a 96 MiB L3,
  `u64` needs at least `--max-log 26` (512 MiB); `--max-log 28` is better.
- **Pin the run.** `run.sh` pins to one core. If you can, also fix the CPU
  governor (`cpupower frequency-set -g performance`).
- **Light desktop use is fine.** Repetitions alternate between sorters, so a
  burst of background work slows all of them alike and the comparison stays
  fair; the median absorbs the rest. Avoid heavy work (compiles, video calls)
  during a run.
- **Same input for every sorter.** The runner already does this: each size
  and distribution is generated once from `--seed`.

## Metric

Each row reports the median and minimum time over repetitions, and the median
divided by `n log2 n` (in nanoseconds). The IPS⁴o paper uses the same
normalization, so an ideal `O(n log n)` sort is a flat line.

`ips4o` randomizes its sampling per call, so its run-to-run spread is a bit
larger than the deterministic sorters'.
