// C ABI wrappers around the C++ IPS4o and std::sort, for the Rust benchmark
// runner. Element layouts must match the #[repr(C)] types in src/inputs.rs.
//
// Compiled without -pthread, so _REENTRANT is undefined and IPS4o's parallel
// code (which needs TBB) is left out.

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <cstring>

#include "ips4o.hpp"

namespace {

struct Pair {
    std::uint64_t a;
    std::uint64_t b;
};
static_assert(sizeof(Pair) == 16, "layout must match Rust");

inline bool operator<(const Pair& x, const Pair& y) {
    return x.a < y.a || (x.a == y.a && x.b < y.b);
}

struct Record100 {
    std::uint8_t key[10];
    std::uint8_t payload[90];
};
static_assert(sizeof(Record100) == 100, "layout must match Rust");

inline bool operator<(const Record100& x, const Record100& y) {
    return std::memcmp(x.key, y.key, sizeof x.key) < 0;
}

}  // namespace

#define SORTERS(T, NAME)                                                  \
    extern "C" void cpp_ips4o_##NAME(T* data, std::size_t len) noexcept { \
        ips4o::sort(data, data + len);                                    \
    }                                                                     \
    extern "C" void cpp_std_sort_##NAME(T* data, std::size_t len) noexcept { \
        std::sort(data, data + len);                                      \
    }

SORTERS(std::uint64_t, u64)
SORTERS(double, f64)
SORTERS(Pair, pair)
SORTERS(Record100, record100)
