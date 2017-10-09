use beam::gen_op;
use beam::opcodes::assert_arity;
use defs::{Word, DispatchResult};
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;
use term::lterm::LTerm;


#[inline]
pub fn opcode_call(ctx: &mut Context, _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_CALL, 2);
  ctx.ip_add(1);

  let location = ctx.fetch_term();
  assert!(location.is_small());

  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_only(_ctx: &mut Context, _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_CALL_ONLY, 2);

  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_last(_ctx: &mut Context, _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_CALL_LAST, 3);

  DispatchResult::Normal
}
