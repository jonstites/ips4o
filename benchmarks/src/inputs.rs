//! Element types and input distributions.
//!
//! The distributions follow the IPS4o paper (Axtmann et al., "Engineering
//! In-place (Shared-memory) Sorting Algorithms", 2020).

use std::cmp::Ordering;
use std::fmt::Write;

/// SplitMix64.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A value in `0..n`.
    pub fn below(&mut self, n: u64) -> u64 {
        ((self.next_u64() as u128 * n as u128) >> 64) as u64
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Distribution {
    /// Uniformly random 64-bit keys.
    Uniform,
    /// Pick `i` uniformly from `0..=log2(n)`, then a key uniformly from
    /// `[2^i, 2^(i+1))`: many small keys, few large ones.
    Exponential,
    /// Sorted, then `sqrt(n)` random pairs swapped.
    AlmostSorted,
    /// `i mod floor(sqrt(n))`: about `sqrt(n)` distinct keys.
    RootDup,
    /// `(i^2 + n/2) mod n`: few distinct keys, heavily duplicated.
    TwoDup,
    /// `(i^8 + n/2) mod n`: fewer still.
    EightDup,
    Sorted,
    ReverseSorted,
    AllEqual,
}

impl Distribution {
    pub const ALL: [Distribution; 9] = [
        Distribution::Uniform,
        Distribution::Exponential,
        Distribution::AlmostSorted,
        Distribution::RootDup,
        Distribution::TwoDup,
        Distribution::EightDup,
        Distribution::Sorted,
        Distribution::ReverseSorted,
        Distribution::AllEqual,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Distribution::Uniform => "uniform",
            Distribution::Exponential => "exponential",
            Distribution::AlmostSorted => "almost_sorted",
            Distribution::RootDup => "root_dup",
            Distribution::TwoDup => "two_dup",
            Distribution::EightDup => "eight_dup",
            Distribution::Sorted => "sorted",
            Distribution::ReverseSorted => "reverse_sorted",
            Distribution::AllEqual => "all_equal",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|d| d.name() == name)
    }

    /// `n` keys from this distribution.
    pub fn keys(self, n: usize, rng: &mut Rng) -> Vec<u64> {
        let n64 = n as u64;
        let pow_mod = |i: u64, e: u32| -> u64 {
            let mut acc = 1u128;
            for _ in 0..e {
                acc = acc * i as u128 % n64.max(1) as u128;
            }
            acc as u64
        };
        match self {
            Distribution::Uniform => (0..n).map(|_| rng.next_u64()).collect(),
            Distribution::Exponential => {
                let levels = n64.max(2).ilog2() as u64 + 1;
                (0..n)
                    .map(|_| {
                        let i = rng.below(levels);
                        (1 << i) + rng.below(1 << i)
                    })
                    .collect()
            }
            Distribution::AlmostSorted => {
                let mut v: Vec<u64> = (0..n64).collect();
                let swaps = (n as f64).sqrt() as usize;
                for _ in 0..swaps {
                    let a = rng.below(n64) as usize;
                    let b = rng.below(n64) as usize;
                    v.swap(a, b);
                }
                v
            }
            Distribution::RootDup => {
                let root = ((n as f64).sqrt() as u64).max(1);
                (0..n64).map(|i| i % root).collect()
            }
            Distribution::TwoDup => (0..n64).map(|i| (pow_mod(i, 2) + n64 / 2) % n64).collect(),
            Distribution::EightDup => (0..n64).map(|i| (pow_mod(i, 8) + n64 / 2) % n64).collect(),
            Distribution::Sorted => (0..n64).collect(),
            Distribution::ReverseSorted => (0..n64).rev().collect(),
            Distribution::AllEqual => vec![42; n],
        }
    }
}

/// A sort under test.
pub struct Sorter<T> {
    pub name: &'static str,
    pub sort: fn(&mut [T]),
}

/// An element type the runner can benchmark.
pub trait Element: Clone + 'static {
    const NAME: &'static str;
    /// Builds an element whose order follows `key`.
    fn from_key(key: u64, rng: &mut Rng) -> Self;
    fn compare(a: &Self, b: &Self) -> Ordering;
    fn sorters() -> Vec<Sorter<Self>>;
}

/// Two 64-bit integers, ordered lexicographically.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pair {
    pub a: u64,
    pub b: u64,
}

/// The paper's 100-byte record: a 10-byte key compared bytewise, plus
/// payload.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Record100 {
    pub key: [u8; 10],
    pub payload: [u8; 90],
}

impl Ord for Record100 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for Record100 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The Rust sorters, for a type with a total order given by `$cmp`.
macro_rules! rust_sorters {
    ($t:ty, $cmp:expr) => {{
        vec![
            Sorter::<$t> {
                name: "ips4o",
                sort: |v| ips4o::sort_unstable_by(v, $cmp),
            },
            Sorter::<$t> {
                name: "std_unstable",
                sort: |v| v.sort_unstable_by($cmp),
            },
            Sorter::<$t> {
                name: "std_stable",
                sort: |v| v.sort_by($cmp),
            },
        ]
    }};
}

/// The C++ sorters for a type with a shim (see cpp/shim.cpp).
#[cfg(feature = "cpp")]
macro_rules! cpp_sorters {
    ($t:ty, $ips4o:ident, $std_sort:ident) => {{
        unsafe extern "C" {
            fn $ips4o(data: *mut $t, len: usize);
            fn $std_sort(data: *mut $t, len: usize);
        }
        vec![
            Sorter::<$t> {
                name: "cpp_ips4o",
                // SAFETY: the shim sorts exactly `len` elements in place, with
                // the same element layout and order as the Rust type.
                sort: |v| unsafe { $ips4o(v.as_mut_ptr(), v.len()) },
            },
            Sorter::<$t> {
                name: "cpp_std_sort",
                // SAFETY: as above.
                sort: |v| unsafe { $std_sort(v.as_mut_ptr(), v.len()) },
            },
        ]
    }};
}

macro_rules! all_sorters {
    ($t:ty, $cmp:expr, $ips4o:ident, $std_sort:ident) => {{
        #[allow(unused_mut)]
        let mut sorters = rust_sorters!($t, $cmp);
        #[cfg(feature = "cpp")]
        sorters.extend(cpp_sorters!($t, $ips4o, $std_sort));
        sorters
    }};
}

impl Element for u64 {
    const NAME: &'static str = "u64";
    fn from_key(key: u64, _: &mut Rng) -> Self {
        key
    }
    fn compare(a: &Self, b: &Self) -> Ordering {
        a.cmp(b)
    }
    fn sorters() -> Vec<Sorter<Self>> {
        all_sorters!(u64, u64::cmp, cpp_ips4o_u64, cpp_std_sort_u64)
    }
}

/// Rust sorts `f64` with `total_cmp`; C++ uses `<`. The inputs contain no
/// NaNs, so the orders agree.
impl Element for f64 {
    const NAME: &'static str = "f64";
    fn from_key(key: u64, _: &mut Rng) -> Self {
        key as f64
    }
    fn compare(a: &Self, b: &Self) -> Ordering {
        a.total_cmp(b)
    }
    fn sorters() -> Vec<Sorter<Self>> {
        all_sorters!(f64, f64::total_cmp, cpp_ips4o_f64, cpp_std_sort_f64)
    }
}

impl Element for Pair {
    const NAME: &'static str = "pair";
    fn from_key(key: u64, rng: &mut Rng) -> Self {
        Pair {
            a: key,
            b: rng.next_u64(),
        }
    }
    fn compare(a: &Self, b: &Self) -> Ordering {
        a.cmp(b)
    }
    fn sorters() -> Vec<Sorter<Self>> {
        all_sorters!(Pair, Pair::cmp, cpp_ips4o_pair, cpp_std_sort_pair)
    }
}

impl Element for Record100 {
    const NAME: &'static str = "record100";
    fn from_key(key: u64, rng: &mut Rng) -> Self {
        let mut record = Record100 {
            key: [0; 10],
            payload: [0; 90],
        };
        record.key[..8].copy_from_slice(&key.to_be_bytes());
        record.key[8..].copy_from_slice(&(rng.next_u64() as u16).to_be_bytes());
        record
    }
    fn compare(a: &Self, b: &Self) -> Ordering {
        a.cmp(b)
    }
    fn sorters() -> Vec<Sorter<Self>> {
        all_sorters!(
            Record100,
            Record100::cmp,
            cpp_ips4o_record100,
            cpp_std_sort_record100
        )
    }
}

/// Fixed-width hex, so string order matches key order. Rust sorters only.
impl Element for String {
    const NAME: &'static str = "string";
    fn from_key(key: u64, _: &mut Rng) -> Self {
        let mut s = String::with_capacity(16);
        write!(s, "{key:016x}").unwrap();
        s
    }
    fn compare(a: &Self, b: &Self) -> Ordering {
        a.cmp(b)
    }
    fn sorters() -> Vec<Sorter<Self>> {
        rust_sorters!(String, String::cmp)
    }
}
