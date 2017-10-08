use beam::gen_op;
use emulator::runtime_ctx::Context;
use defs::{Word, DispatchResult};

#[inline]
fn assert_arity(op: gen_op::OPCODE, val: Word) {
  assert_eq!(gen_op::ARITY_MAP[op as usize] as Word, val,
             "{:?} arity is expected to be {}", op, val);
}


#[inline]
pub fn opcode_call(ctx: &mut Context) -> DispatchResult {
  assert_arity(gen_op::OPCODE::Call, 2);

  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_only(ctx: &mut Context) -> DispatchResult {
  assert_arity(gen_op::OPCODE::CallOnly, 2);

  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_last(ctx: &mut Context) -> DispatchResult {
  assert_arity(gen_op::OPCODE::CallLast, 3);

  DispatchResult::Normal
}
