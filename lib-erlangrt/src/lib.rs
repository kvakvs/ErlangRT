//! `ErlangRT` is an alternative Erlang BEAM Runtime written in Rust.
//! This is a library, to run it, link against it and call start_emulator().
//! This is done in `erl` and in `ct_run` projects.
#![crate_type = "lib"]
#![crate_name = "erlangrt"]
#![feature(raw)]
// #![feature(ptr_internals)] // for std/core::ptr::Unique
#![feature(maybe_uninit)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate bitflags;
// extern crate paste;

mod beam;
mod big;
mod defs;
mod emulator;
mod fail;
mod native_fun;
mod rt_util;
mod term;
pub mod command_line_args;
pub mod lib_main;
