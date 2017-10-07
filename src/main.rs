//!
//! ErlangRT is an alternative Erlang BEAM Runtime written in Rust
//!
// Comment this to use Rust's jemalloc library which is fat but fast
//#![feature(alloc_system)]
//extern crate alloc_system;
extern crate compress;
extern crate bytes;
extern crate num;
extern crate bit_field;

mod beam;
mod emulator;
mod fail;
mod term;
mod defs;
mod util;

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

  //let test_a = "test".to_string();
  //let t = world.new_atom(&test_a);
  //println!("t.val={}", t.get_raw())

  let mfa = MFArgs::new(beam.atom("test"),
                        beam.atom("start"),
                        Vec::new()
  );
  let r = beam.create_process(LTerm::nil(), &mfa);
  let _root_p = match r {
    Ok(p0) => p0,
    Err(e) => panic!("{:?}", e)
  };

  println!("Process created. Entering main loop...");
  while beam.tick() {
  }
}
