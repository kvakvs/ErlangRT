mod mfargs;
mod term;
mod vm;
mod types;
mod process;

use term::Term;
use vm::VM;
use mfargs::MFArgs;

fn main() {
  println!("Erlang/OTP Replacement (compat OTP 20)");

  let mut beam = VM::new();

  //let test_a = "test".to_string();
  //let t = world.new_atom(&test_a);
  //println!("t.val={}", t.get_raw())

  let mfa = MFArgs::new(
    beam.find_or_create_atom("init"),
    beam.find_or_create_atom("start"),
    Vec::new()
  );
  let root_p = beam.create_process(Term::nil(), &mfa);
}
