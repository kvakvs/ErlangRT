use beam::gen_op;
use defs::{Word, DispatchResult};
use emulator::runtime_ctx::Context;
use term::lterm::LTerm;

#[inline]
fn assert_arity(op: gen_op::OPCODE, val: Word) {
  assert_eq!(gen_op::ARITY_MAP[op as usize] as Word, val,
             "{:?} arity is expected to be {}", op, val);
}


#[inline]
pub fn opcode_call(ctx: &mut Context) -> DispatchResult {
  assert_arity(gen_op::OPCODE::Call, 2);
  ctx.ip_add(1);
  let location = LTerm::from_raw(ctx.fetch());
  assert!(location.is_small());
  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_only(_ctx: &mut Context) -> DispatchResult {
  assert_arity(gen_op::OPCODE::CallOnly, 2);

  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_last(_ctx: &mut Context) -> DispatchResult {
  assert_arity(gen_op::OPCODE::CallLast, 3);

  DispatchResult::Normal
}
