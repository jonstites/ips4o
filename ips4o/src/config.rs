//! Compile-time tuning parameters.
//!
//! Every knob lives on the [`Config`] trait so tests can instantiate the
//! algorithm with tiny values (2 buckets, base case of 1 element, ...). Small
//! inputs then exercise every code path, which keeps Miri runs fast.

/// Tuning parameters for the sort.
pub(crate) trait Config {
    /// log2 of the maximum number of buckets per partitioning step.
    const LOG_MAX_BUCKETS: u32;
    /// Slices with at most this many elements are handed to
    /// `slice::sort_unstable_by` instead of being partitioned.
    const BASE_CASE_SIZE: usize;
    /// Desired bucket size. The number of buckets is chosen so that buckets
    /// shrink to about this size within one or two partitioning levels.
    /// Must be at most `BASE_CASE_SIZE`.
    const BUCKET_TARGET_SIZE: usize;
    /// Oversampling, in percent per bit of input size: each splitter is
    /// chosen from `max(1, OVERSAMPLING_PERCENT * log2(n) / 100)` samples.
    const OVERSAMPLING_PERCENT: usize;
    /// Equality buckets are used once the sorted splitters contain at least
    /// this many adjacent equal pairs (capped at half the splitter count, so
    /// small trees can still detect heavy duplication).
    const EQUAL_BUCKETS_THRESHOLD: usize;
}

/// The defaults. These are the C++ implementation's values (`kBaseCaseSize`
/// is our `BUCKET_TARGET_SIZE`, and C++ recurses down to `2 * kBaseCaseSize`).
/// They are not tuned yet; with std's sort as the base case, a larger
/// `BASE_CASE_SIZE` may be better.
pub(crate) struct DefaultConfig;

impl Config for DefaultConfig {
    const LOG_MAX_BUCKETS: u32 = 8;
    const BASE_CASE_SIZE: usize = 32;
    const BUCKET_TARGET_SIZE: usize = 16;
    const OVERSAMPLING_PERCENT: usize = 20;
    const EQUAL_BUCKETS_THRESHOLD: usize = 5;
}

/// Rejects nonsensical configurations at compile time.
pub(crate) const fn validate<C: Config>() {
    assert!(C::LOG_MAX_BUCKETS >= 1 && C::LOG_MAX_BUCKETS <= 16);
    assert!(C::BUCKET_TARGET_SIZE >= 1);
    assert!(C::BASE_CASE_SIZE >= C::BUCKET_TARGET_SIZE);
    assert!(C::EQUAL_BUCKETS_THRESHOLD >= 1);
}

/// Chooses `L` (the step partitions into `2^L` buckets) for `n` elements.
///
/// Like the C++ implementation, this aims for buckets that reach
/// `BUCKET_TARGET_SIZE` after one level (small `n`) or two levels (medium
/// `n`), and otherwise uses the maximum.
///
/// Requires `n > C::BASE_CASE_SIZE`.
pub(crate) fn log_buckets<C: Config>(n: usize) -> u32 {
    const { validate::<C>() };
    debug_assert!(n > C::BASE_CASE_SIZE);
    let max = C::LOG_MAX_BUCKETS;
    let lg = (n / C::BUCKET_TARGET_SIZE).ilog2();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_log_buckets() {
        type C = DefaultConfig;
        // One level: buckets end up near the base case.
        assert_eq!(log_buckets::<C>(33), 1);
        assert_eq!(log_buckets::<C>(16 * 16), 4);
        assert_eq!(log_buckets::<C>(16 * 256), 8);
        // Two levels.
        assert_eq!(log_buckets::<C>(16 * 512), 5);
        assert_eq!(log_buckets::<C>(16 * 256 * 256), 8);
        // Saturated.
        assert_eq!(log_buckets::<C>(usize::MAX), 8);
    }

    #[test]
    fn sample_fits() {
        type C = DefaultConfig;
        for n in [33, 100, 1000, 1 << 20, usize::MAX / 2] {
            let m = (1usize << log_buckets::<C>(n)) - 1;
            let step = sample_step::<C>(n, m);
            assert!(step >= 1 && step * m <= n, "n={n} m={m} step={step}");
        }
    }
}
