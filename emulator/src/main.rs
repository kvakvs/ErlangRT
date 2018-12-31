//! `ErlangRT` is an alternative Erlang BEAM Runtime written in Rust

#[macro_use]
extern crate lazy_static;

// extern crate core;

mod beam;
mod bif;
mod defs;
mod emulator;
mod fail;
mod main_main;
mod rt_util;
mod term;

use crate::main_main::entrypoint;

fn main() {
  entrypoint();
}
