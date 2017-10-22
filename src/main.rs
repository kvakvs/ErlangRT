//!
//! `ErlangRT` is an alternative Erlang BEAM Runtime written in Rust
//!

#![feature(const_fn)]
//#![feature(alloc)] // for rawvec

// Use from command line instead: `cargo build --features "clippy"` or `make clippy`
//#![cfg_attr(feature="clippy", feature(plugin))]
//#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate compress;
extern crate bytes;
extern crate num;
extern crate bit_field;

#[macro_use]
extern crate lazy_static;

mod beam;
mod bif;
mod emulator;
mod fail;
mod term;
mod defs;
mod util;

use emulator::atom;
use emulator::scheduler::Prio;
use emulator::mfa::MFArgs;
use emulator::vm::VM;
use term::lterm::LTerm;

/// Entry point for the command-line interface
fn main() {
  if cfg!(feature = "r19") {
    println!("Erlang Runtime (compat OTP 19)");
  }
  if cfg!(feature = "r20") {
    println!("Erlang Runtime (compat OTP 20)");
  }

  let mut beam = VM::new();

  let mfa = MFArgs::new(atom::from_str("test"),
                        atom::from_str("start"),
                        Vec::new());
  let rootp = beam.create_process(LTerm::nil(),
                                  &mfa,
                                  Prio::Normal).unwrap();

  println!("Process created. Entering main loop...");
  while beam.tick() {}
}
