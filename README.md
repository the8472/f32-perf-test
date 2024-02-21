# f32-perf-test

This crate was created to benchmark the difference between multiple implementations of f32::midpoint.
in this PR: https://github.com/rust-lang/rust/pull/121062

To run the benchmark, you must have Rust installed, and `cargo` installed (Rust's package manager).

Simply run:
```sh
cargo bench
```

You may want to tweak the RUSTFLAGS to include `-C target-cpu=native` to check if that improves performance!
