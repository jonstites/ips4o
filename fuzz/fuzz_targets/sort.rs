//! The in-place and parallel sorts must agree with std on any input, under
//! every configuration and seed.

#![no_main]

mod common;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct Input {
    config: u8,
    parallel: bool,
    seed: u64,
    keys: common::Keys,
}

fuzz_target!(|input: Input| {
    let mut v = input.keys.to_u64();
    let mut expected = v.clone();
    expected.sort_unstable();
    common::pool().install(|| {
        ips4o::fuzzing::sort(input.config, input.parallel, &mut v, &u64::cmp, input.seed)
    });
    assert_eq!(v, expected);
});
