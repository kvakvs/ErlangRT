use beam::gen_op;
use beam::opcodes::assert_arity;
use defs::{Word, DispatchResult};
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;
use term::lterm::LTerm;


#[inline]
pub fn opcode_call(ctx: &mut Context, _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_CALL, 2);
  let _arity = ctx.fetch(); // skip arity
  let location = ctx.fetch_term();
  assert!(location.is_small());

  ctx.cp = ctx.ip;
  ctx.ip = ctx.ip.offset(location.small_get_s());

  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_only(ctx: &mut Context, _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_CALL_ONLY, 2);
  let _arity = ctx.fetch(); // skip arity
  let location = ctx.fetch_term();
//  println!("call_only a={} loc={}", _arity, location);
  assert!(location.is_small());

  ctx.ip = ctx.ip.offset(location.small_get_s());

  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_last(_ctx: &mut Context, _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_CALL_LAST, 3);
  panic!("notimpl call_last");
  DispatchResult::Normal
}
