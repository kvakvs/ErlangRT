//!
//! ErlangRT is an alternative Erlang BEAM Runtime written in Rust
//!
// Comment this to use Rust's jemalloc library which is fat but fast
//#![feature(alloc_system)]
//extern crate alloc_system;
extern crate bytes;
extern crate num;

mod beam;
mod emulator;
mod rterror;
mod term;
mod defs;
mod util;

use emulator::mfa::MFArgs;
use emulator::vm::VM;
use term::low_level::LTerm;

/// Entry point for the command-line interface
fn main() {
  println!("Erlang Runtime (compat OTP 20)");

  let mut beam = VM::new();

  //let test_a = "test".to_string();
  //let t = world.new_atom(&test_a);
  //println!("t.val={}", t.get_raw())

  let mfa = MFArgs::new(beam.atom("lists"),
                        beam.atom("start"),
                        Vec::new()
  );
  let root_p = match beam.create_process(LTerm::nil(), &mfa) {
    Ok(p0) => p0,
    Err(e) => panic!("{:?}", e)
  };

  while beam.tick() {
  }
}
