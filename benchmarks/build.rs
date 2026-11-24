fn main() {
    #[cfg(feature = "cpp")]
    build_cpp();
}

/// Compiles the C++ shim. Uses `-march=native` exactly when the Rust code is
/// built with `-C target-cpu=native`, so both sides get the same treatment.
#[cfg(feature = "cpp")]
fn build_cpp() {
    use std::path::Path;

    let include = Path::new("third_party/ips4o/include");
    if !include.join("ips4o.hpp").exists() {
        panic!("C++ IPS4o not found; run `git submodule update --init`");
    }

    let rustflags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default();
    let native = rustflags
        .split('\x1f')
        .any(|flag| flag.contains("target-cpu=native"));

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .file("cpp/shim.cpp")
        .include(include)
        .define("NDEBUG", None)
        .opt_level(3)
        .warnings(false);
    if native {
        build.flag("-march=native");
    }
    build.compile("ips4o_shim");

    println!("cargo:rerun-if-changed=cpp/shim.cpp");
    println!("cargo:rerun-if-changed=third_party/ips4o/include");
}
