liboptarith
===========

Optimized arithmetic operations for 32, 64, and 128bit integers.

Includes optimized implementations of many different extended GCD algorithms.

Rust workspace
==============

This repository now includes a Cargo workspace with two crates:

- `optarith-sys`: raw FFI + C build/linking
- `optarith`: safe Rust wrappers on top of `optarith-sys`

```
cargo test
```

The safe wrapper crate (`optarith`) exposes broad safe wrappers for
32/64/128-bit arithmetic, GCD/XGCD, and GMP-backed helpers.
For full 1:1 low-level access, use `optarith::raw` (re-export of
`optarith-sys`).
The `gmp` feature is enabled by default.

Build-time codegen
==================

`optarith-sys` runs `code_gen/gen_sqrtmodp.cc` from `build.rs` and uses
`OPTARITH_MAX_PRIME` (default `104729`) to control the generated table size.
The default is set in `.cargo/config.toml` and can be changed there.

`optarith-sys` extras
========================

- Rust port of `tests/test_gcd.c`: `crates/optarith-sys/tests/test_gcd.rs`
- Rust port of `tests/test_u128.cc`: `crates/optarith-sys/tests/test_u128.rs`
- Rust ports of timing tools:
  - `cargo run -p optarith-sys --bin timegcd -- <seed> <pairs>`
  - `cargo run -p optarith-sys --bin timepartial -- <seed> <triples>`
