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

The C++ sorters cover `u64`, `f64`, `pair` and `record100`. Strings are
Rust-only.

## Running

The C++ comparison needs the submodule, a C++17 compiler, and TBB's headers
(the C++ IPS⁴o includes them even for its sequential sort):

```sh
git submodule update --init
sudo apt install libtbb-dev
```

Build with `-C target-cpu=native`; the build script then compiles the C++ with
`-march=native` too, so both sides are optimized alike:

```sh
RUSTFLAGS="-C target-cpu=native" cargo run -p benchmarks --release --features cpp -- \
    --types u64 --out benchmarks/results/u64.csv
uv run benchmarks/plot.py benchmarks/results/u64.csv
```

`--help` lists every option. Useful ones: `--types`, `--dists`, `--sorters`,
`--min-log`/`--max-log`/`--log-step` (sizes as powers of two),
`--max-bytes` (skips sizes that don't fit), `--min-time` (per measurement).

Results go to `benchmarks/results/`, which is git-ignored.

## Getting trustworthy numbers

- **Go past the cache.** Sizes up to the L3 cache measure an in-cache sort,
  not the memory-bound regime IPS⁴o is designed for. With a 96 MiB L3,
  `u64` needs at least `--max-log 26` (512 MiB); `--max-log 28` is better.
- **Pin and quiet the machine.** Run on one core (`taskset -c 2 ...`), close
  other programs, and keep the CPU governor fixed if you can
  (`cpupower frequency-set -g performance`).
- **Same input for every sorter.** The runner already does this: each size
  and distribution is generated once from `--seed`.

## Metric

Each row reports the median and minimum time over repetitions, and the median
divided by `n log2 n` (in nanoseconds). The IPS⁴o paper uses the same
normalization, so an ideal `O(n log n)` sort is a flat line.

`ips4o` randomizes its sampling per call, so its run-to-run spread is a bit
larger than the deterministic sorters'.
