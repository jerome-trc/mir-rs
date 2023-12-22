# mir-rs

## About

The `mirjit-sys` crate is a set of auto-generated FFI bindings for [Vladimir Makarov's MIR](https://github.com/vnmakarov/mir) toolchain.

The `mirjit` crate is a safe Rust wrapper for `mirjit-sys` to allow more ergonomic use by other Rust projects.

Please note that neither this project nor Makarov's are related to Rust's [Mid-level IR](https://rustc-dev-guide.rust-lang.org/mir/index.html).

## Feature Flags

`mirjit-sys` comes with the `parallel` feature flag, which passes along the `MIR_PARALLEL_GEN` preprocessor setting, enabling parallelized generation of machine code.

## Licensing

MIR is provided under the [MIT License](https://github.com/vnmakarov/mir/blob/master/LICENSE).

The [same license](./LICENSE) applies to this project and all code herein.
