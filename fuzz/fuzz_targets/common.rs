//! Shared by the fuzz targets.

use std::sync::OnceLock;

/// A 4-thread pool, so the parallel sort really runs several stripes at once.
pub fn pool() -> &'static rayon::ThreadPool {
    static POOL: OnceLock<rayon::ThreadPool> = OnceLock::new();
    POOL.get_or_init(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap()
    })
}

/// Keys from a few ranges, so the fuzzer can produce heavy duplication as
/// easily as distinct keys.
#[derive(arbitrary::Arbitrary, Debug)]
pub enum Keys {
    Bits(Vec<bool>),
    Bytes(Vec<u8>),
    Shorts(Vec<u16>),
    Wide(Vec<u64>),
}

impl Keys {
    pub fn to_u64(&self) -> Vec<u64> {
        match self {
            Keys::Bits(v) => v.iter().map(|&k| k as u64).collect(),
            Keys::Bytes(v) => v.iter().map(|&k| k as u64).collect(),
            Keys::Shorts(v) => v.iter().map(|&k| k as u64).collect(),
            Keys::Wide(v) => v.clone(),
        }
    }
}
