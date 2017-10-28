//use std::cmp::Ordering;

use beam::gen_op;
use beam::opcodes::assert_arity;
use defs::{DispatchResult};
use emulator::code::CodePtr;
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;
use term::compare;


/// Checks exact equality between arg1 and arg2, on false jump to arg0
#[inline]
pub fn opcode_is_eq_exact(ctx: &mut Context,
                          hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_IS_EQ_EXACT, 3);

  let on_false = ctx.fetch_term();
  let a = ctx.fetch_and_load(hp);
  let b = ctx.fetch_and_load(hp);

  if compare::eq_terms(a, b, true) == false {
    ctx.ip = CodePtr::from_cp(on_false)
  }

  DispatchResult::Normal
}
