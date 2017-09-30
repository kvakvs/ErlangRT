//!
//! ErlangRT is an alternative Erlang BEAM Runtime written in Rust
//!

extern crate bytes;

mod beam;
mod code_srv;
mod function;
mod mfa;
mod module;
mod process;
mod rterror;
mod term;
mod types;
mod util;
mod vm;

use mfa::MFArgs;
use term::Term;
use vm::VM;

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
  let root_p = beam.create_process(Term::nil(), &mfa);
  while beam.tick() {
  }
}
