mod term;
mod vm;
mod types;

use vm::VM;

fn main() {
  println!("Erlang/OTP Runtime Replacement");

  let mut world = VM::new();

  let test_a = "test".to_string();
  let t = world.new_atom(&test_a);
  println!("t.val={}", t.get_raw())
}
