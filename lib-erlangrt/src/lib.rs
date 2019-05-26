//! `ErlangRT` is an alternative Erlang BEAM Runtime written in Rust.
//! This is a library, to run it, link against it and call start_emulator().
//! This is done in `erl` and in `ct_run` projects.
#![crate_type = "lib"]
#![crate_name = "erlangrt"]
#![feature(raw)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

mod beam;
mod big;
pub mod command_line_args;
mod defs;
mod emulator;
mod fail;
pub mod lib_main;
mod native_fun;
mod rt_util;
mod term;
