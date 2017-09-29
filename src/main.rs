extern crate bytes;

mod beam;
mod code_srv;
mod mfargs;
mod process;
mod rterror;
mod term;
mod types;
mod util;
mod vm;

use mfargs::MFArgs;
use term::Term;
use vm::VM;

fn main() {
  println!("Erlang/OTP Replacement (compat OTP 20)");

  let mut beam = VM::new();

  //let test_a = "test".to_string();
  //let t = world.new_atom(&test_a);
  //println!("t.val={}", t.get_raw())

  let mfa = MFArgs::new(
    beam.find_or_create_atom("lists"),
    beam.find_or_create_atom("start"),
    Vec::new()
  );
  let root_p = beam.create_process(Term::nil(), &mfa);
  while beam.tick() {
  }
}
