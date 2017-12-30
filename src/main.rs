//! `ErlangRT` is an alternative Erlang BEAM Runtime written in Rust
//!

#![feature(const_fn)]
//#![feature(alloc)] // for rawvec
#![feature(const_size_of)]

// Use from command line instead: `cargo build --features "clippy"` or `make clippy`
//#![cfg_attr(feature="clippy", feature(plugin))]
//#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate bit_field;
extern crate bytes;
extern crate compress;
extern crate num;

#[macro_use]
extern crate lazy_static;

//#[macro_use]
//extern crate log;

extern crate rt_defs;
extern crate rt_util;

mod beam;
mod bif;
mod emulator;
mod fail;
mod term;
mod main_main;


use main_main::{entrypoint};

fn main() { entrypoint(); }
