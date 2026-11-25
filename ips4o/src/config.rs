//! Compile-time tuning parameters.
//!
//! Every knob lives on the [`Config`] trait so tests can instantiate the
//! algorithm with tiny values (2 buckets, base case of 1 element, ...). Small
//! inputs then exercise every code path, which keeps Miri runs fast.

/// Tuning parameters for the sort.
pub(crate) trait Config {
    /// log2 of the maximum number of buckets per partitioning step.
    const LOG_MAX_BUCKETS: u32;
    /// The base case: slices of at most `base_case_size::<T, C>()` elements,
    /// that is `BASE_CASE_BYTES / size_of::<T>()` clamped to
    /// `MIN_BASE_CASE..=MAX_BASE_CASE`, are handed to
    /// `slice::sort_unstable_by` instead of being partitioned. Buckets aim
    /// for half that size.
    ///
    /// Sized in bytes because the best cutoff depends on element size: std's
    /// sort moves elements around a lot, which is cheap for small elements
    /// and costly for large ones, while partitioning moves whole blocks.
    const BASE_CASE_BYTES: usize;
    const MIN_BASE_CASE: usize;
    const MAX_BASE_CASE: usize;
    /// Oversampling, in percent per bit of input size: each splitter is
    /// chosen from `max(1, OVERSAMPLING_PERCENT * log2(n) / 100)` samples.
    const OVERSAMPLING_PERCENT: usize;
    /// Equality buckets are used once the sorted splitters contain at least
    /// this many adjacent equal pairs (capped at half the splitter count, so
    /// small trees can still detect heavy duplication).
    const EQUAL_BUCKETS_THRESHOLD: usize;
    /// Size of a block in bytes. Elements move between buffers and the input
    /// in blocks of `max(1, BLOCK_BYTES / size_of::<T>())` elements.
    const BLOCK_BYTES: usize;
    /// The parallel sort partitions in parallel only if each thread gets at
    /// least this many elements; smaller inputs (and buckets) are sorted
    /// sequentially. (Default 2^12: on 16 threads, 2^14 and 2^16 left
    /// mid-sized inputs sequential, and 2^10 made nested parallel steps on
    /// small buckets cost more than they saved.)
    const PAR_MIN_STRIPE: usize;
}

/// The defaults. Bucket counts, oversampling and block size follow the C++
/// implementation. The base case is much larger than its 32 elements,
/// because our base case is std's sort rather than an insertion sort:
/// 4 KiB of elements, 32 to 512 of them (512 for `u64`, about 170 for
/// `String`, 40 for 100-byte records). Measured on one machine (Zen 5).
pub(crate) struct DefaultConfig;

impl Config for DefaultConfig {
    const LOG_MAX_BUCKETS: u32 = 8;
    const BASE_CASE_BYTES: usize = 4096;
    const MIN_BASE_CASE: usize = 32;
    const MAX_BASE_CASE: usize = 512;
    const OVERSAMPLING_PERCENT: usize = 20;
    const EQUAL_BUCKETS_THRESHOLD: usize = 5;
    const BLOCK_BYTES: usize = 2048;
    const PAR_MIN_STRIPE: usize = 1 << 12;
}

/// Rejects nonsensical configurations at compile time.
pub(crate) const fn validate<C: Config>() {
    // At most 2^16 - 1 buckets (with equality buckets), so ids fit in a u16.
    assert!(C::LOG_MAX_BUCKETS >= 1 && C::LOG_MAX_BUCKETS <= 15);
    assert!(C::MIN_BASE_CASE >= 1);
    assert!(C::MAX_BASE_CASE >= C::MIN_BASE_CASE);
    assert!(C::EQUAL_BUCKETS_THRESHOLD >= 1);
    assert!(C::PAR_MIN_STRIPE >= 1);
}

/// Slices of at most this many elements go to the base case sort.
pub(crate) fn base_case_size<T, C: Config>() -> usize {
    (C::BASE_CASE_BYTES / size_of::<T>().max(1)).clamp(C::MIN_BASE_CASE, C::MAX_BASE_CASE)
}

/// The bucket size that partitioning aims for: half the base case.
pub(crate) fn bucket_target_size<T, C: Config>() -> usize {
    (base_case_size::<T, C>() / 2).max(1)
}

/// Number of elements per block.
pub(crate) fn block_size<T, C: Config>() -> usize {
    (C::BLOCK_BYTES / size_of::<T>().max(1)).max(1)
}

/// Chooses `L` (the step partitions into `2^L` buckets) for `n` elements.
///
/// Like the C++ implementation, this aims for buckets that reach the
/// bucket target size after one level (small `n`) or two levels (medium
/// `n`), and otherwise uses the maximum.
///
/// Requires `n > base_case_size::<T, C>()`.
pub(crate) fn log_buckets<T, C: Config>(n: usize) -> u32 {
    const { validate::<C>() };
    debug_assert!(n > base_case_size::<T, C>());
    let max = C::LOG_MAX_BUCKETS;
    let lg = (n / bucket_target_size::<T, C>()).ilog2();
    let l = if lg <= max {
        lg
    } else if lg <= 2 * max {
        lg.div_ceil(2)
    } else {
        max
    };
    l.clamp(1, max)
}

/// Number of samples per splitter when picking `m` splitters from `n` elements.
/// Guarantees `step * m <= n`.
pub(crate) fn sample_step<C: Config>(n: usize, m: usize) -> usize {
    let step = (C::OVERSAMPLING_PERCENT * n.ilog2() as usize / 100).max(1);
    step.min(n / m)
}

/// Tiny configurations for tests and fuzzing.
#[cfg(any(test, feature = "fuzzing"))]
pub(crate) mod small {
    use super::Config;

    /// Tiny configurations. Small inputs drive these through many levels of
    /// partitioning, which keeps the tests (and Miri) fast. Block sizes vary:
    /// `u64` gets 2, 12 and 5 elements per block; `Tracked` (32 bytes) gets 1, 3
    /// and 1.
    pub struct TwoBuckets;
    impl Config for TwoBuckets {
        const LOG_MAX_BUCKETS: u32 = 1;
        const BASE_CASE_BYTES: usize = 0;
        const MIN_BASE_CASE: usize = 1;
        const MAX_BASE_CASE: usize = 1;
        const OVERSAMPLING_PERCENT: usize = 20;
        const EQUAL_BUCKETS_THRESHOLD: usize = 1;
        const BLOCK_BYTES: usize = 16;
        const PAR_MIN_STRIPE: usize = 2;
    }

    pub struct FourBuckets;
    impl Config for FourBuckets {
        const LOG_MAX_BUCKETS: u32 = 2;
        const BASE_CASE_BYTES: usize = 0;
        const MIN_BASE_CASE: usize = 2;
        const MAX_BASE_CASE: usize = 2;
        const OVERSAMPLING_PERCENT: usize = 50;
        const EQUAL_BUCKETS_THRESHOLD: usize = 1;
        const BLOCK_BYTES: usize = 96;
        const PAR_MIN_STRIPE: usize = 3;
    }

    pub struct EightBuckets;
    impl Config for EightBuckets {
        const LOG_MAX_BUCKETS: u32 = 3;
        const BASE_CASE_BYTES: usize = 0;
        const MIN_BASE_CASE: usize = 4;
        const MAX_BASE_CASE: usize = 4;
        const OVERSAMPLING_PERCENT: usize = 100;
        const EQUAL_BUCKETS_THRESHOLD: usize = 2;
        const BLOCK_BYTES: usize = 40;
        const PAR_MIN_STRIPE: usize = 8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case_follows_element_size() {
        type C = DefaultConfig;
        assert_eq!(base_case_size::<u8, C>(), 512);
        assert_eq!(base_case_size::<u64, C>(), 512);
        assert_eq!(base_case_size::<String, C>(), 4096 / 24);
        assert_eq!(base_case_size::<[u8; 100], C>(), 40);
        assert_eq!(base_case_size::<[u8; 1000], C>(), 32);
        assert_eq!(bucket_target_size::<u64, C>(), 256);
    }

    #[test]
    fn default_log_buckets() {
        type C = DefaultConfig;
        // u64: base case 512, bucket target 256.
        // One level: buckets end up near the target.
        assert_eq!(log_buckets::<u64, C>(513), 1);
        assert_eq!(log_buckets::<u64, C>(256 * 16), 4);
        assert_eq!(log_buckets::<u64, C>(256 * 256), 8);
        // Two levels.
        assert_eq!(log_buckets::<u64, C>(256 * 512), 5);
        assert_eq!(log_buckets::<u64, C>(256 * 256 * 256), 8);
        // Saturated.
        assert_eq!(log_buckets::<u64, C>(usize::MAX), 8);
    }

    #[test]
    fn sample_fits() {
        type C = DefaultConfig;
        for n in [513, 1000, 1 << 20, usize::MAX / 2] {
            let m = (1usize << log_buckets::<u64, C>(n)) - 1;
            let step = sample_step::<C>(n, m);
            assert!(step >= 1 && step * m <= n, "n={n} m={m} step={step}");
        }
    }
}
