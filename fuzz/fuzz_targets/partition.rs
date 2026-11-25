//! One partitioning step, in place or with 1-5 stripes, must produce exactly
//! the reference implementation's buckets.

#![no_main]

mod common;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct Input {
    config: u8,
    seed: u64,
    /// 0 is the in-place step; 1-5 the parallel step with that many stripes.
    stripes: u8,
    keys: common::Keys,
}

fuzz_target!(|input: Input| {
    let keys = input.keys.to_u64();
    let stripes = (input.stripes % 6) as usize;
    let result = common::pool()
        .install(|| ips4o::fuzzing::check_partition(input.config, &keys, input.seed, stripes));
    if let Err(message) = result {
        panic!("partition differs from the reference: {message}");
    }
});
