//! Module implements opcodes related to execution control: Calls, jumps,
//! returns etc.
use beam::gen_op;
use emulator::code::CodePtr;
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
  debug_assert!(location.is_box(),
                "Call location must be a box (have {})", location);

  ctx.cp = ctx.ip;
  ctx.ip = CodePtr::from_ptr(location.box_ptr());

  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_only(ctx: &mut Context, _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_CALL_ONLY, 2);
  let _arity = ctx.fetch(); // skip arity
  let location = ctx.fetch_term();
  debug_assert!(location.is_box(),
                "Call location must be a box (have {})", location);

  ctx.ip = CodePtr::from_ptr(location.box_ptr());

  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_last(_ctx: &mut Context, _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_CALL_LAST, 3);
  panic!("notimpl call_last");
  DispatchResult::Normal
}
