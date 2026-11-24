#!/usr/bin/env bash
# Builds the runner twice, for generic x86-64 and with -C target-cpu=native
# (the C++ follows suit), and runs both with the same arguments, pinned to
# one core.
#
# usage: benchmarks/run.sh NAME [runner options...]
#   writes benchmarks/results/NAME-generic.csv and NAME-native.csv
#
# environment:
#   BENCH_CPU=N        core to pin to (default: 2)
#   BENCH_BUILDS=...   which builds to run (default: "generic native")
set -euo pipefail

if [[ $# -lt 1 ]]; then
    sed -n '2,13p' "$0" | sed 's/^# \{0,1\}//'
    exit 1
fi
name=$1
shift

root=$(cd "$(dirname "$0")/.." && pwd)
results=$root/benchmarks/results
mkdir -p "$results"

for build in ${BENCH_BUILDS:-generic native}; do
    case $build in
        generic) flags="" ;;
        native) flags="-C target-cpu=native" ;;
        *) echo "unknown build: $build" >&2; exit 1 ;;
    esac
    # Separate target directories, so one build never overwrites the other.
    target=$root/target/bench-$build
    RUSTFLAGS="$flags" CARGO_TARGET_DIR=$target \
        cargo build --quiet --release --manifest-path "$root/Cargo.toml" -p benchmarks --features cpp
    out=$results/$name-$build.csv
    echo "== $build -> $out" >&2
    taskset -c "${BENCH_CPU:-2}" "$target/release/benchmarks" --out "$out" "$@" 2> >(tee "$results/$name-$build.log" >&2)
done
