# Fuzzing

Three [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) targets, built with
AddressSanitizer. They reach the sort's internals through the library's hidden
`fuzzing` feature, which runs the sorts under the tiny test configurations
(few buckets, tiny blocks), so short inputs go through every phase.

| target | checks |
|---|---|
| `sort` | the in-place and parallel sorts match std, for any input, configuration and seed |
| `partition` | one partitioning step (in place, or parallel with 1–5 stripes) produces exactly the reference's buckets |
| `chaos` | comparators that panic at any call or answer at random: every element stays in the slice exactly once and is dropped exactly once; honest runs come out sorted |

```sh
cargo install cargo-fuzz          # once
cargo +nightly fuzz run sort      # until stopped; or add -- -max_total_time=600
cargo +nightly fuzz run chaos -- -jobs=8 -workers=8
```

A crash leaves its input in `artifacts/<target>/`; re-run it with
`cargo +nightly fuzz run <target> artifacts/<target>/crash-...`, and shrink it
with `cargo +nightly fuzz tmin <target> <file>`.

The parallel sort runs on a 4-thread pool, so a few runs on the same input
may take different thread schedules; a crash that depends on one may need
several re-runs to reproduce.
