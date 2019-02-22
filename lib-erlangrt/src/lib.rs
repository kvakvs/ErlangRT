//! `ErlangRT` is an alternative Erlang BEAM Runtime written in Rust.
//! This is a library, to run it, link against it and call start_emulator().
//! This is done in `erl` and in `ct_run` projects.
#![crate_type = "lib"]
#![crate_name = "erlangrt"]

#[macro_use]
extern crate lazy_static;

extern crate paste;

mod beam;
mod bif;
pub mod command_line_args;
mod defs;
mod emulator;
mod fail;
pub mod lib_main;
mod rt_util;
mod term;
