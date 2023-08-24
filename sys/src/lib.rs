//! # mircomp-sys
//!
//! Auto-generated FFI bindings to [Vladimir Makarov's MIR](https://github.com/vnmakarov/mir) toolchain.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)] // `u128`

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
