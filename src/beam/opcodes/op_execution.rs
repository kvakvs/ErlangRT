//! Module implements opcodes related to execution control: Calls, jumps,
//! returns etc.
use beam::gen_op;
use emulator::code::CodePtr;
use beam::opcodes::assert_arity;
use defs::{DispatchResult};
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;
//use term::lterm::LTerm;


fn module() -> &'static str { "opcodes::op_execution: " }


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


//#[inline]
//pub fn opcode_call_last(_ctx: &mut Context, _heap: &mut Heap) -> DispatchResult {
//  assert_arity(gen_op::OPCODE_CALL_LAST, 3);
//  panic!("notimpl call_last");
//  DispatchResult::Normal
//}


/// Performs a tail recursive call to a Destination mfarity (a tuple
/// `{Mod, Fun, Arity}`) which can point to an exported function or a BIF.
/// Does not update the CP register with a return address, making return skip
/// over the current code location.
#[inline]
pub fn opcode_call_ext_only(ctx: &mut Context,
                            _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_CALL_EXT_ONLY, 2);
  let _arity = ctx.fetch();
  let _export = ctx.fetch();
  // TODO: Get M:F/Arity

  // TODO: is bif?
  if true { //is_bif(export) {
    // TODO: call bif
  } else {
    // TODO: Call export properly
  }
  panic!("{}Incomplete call_ext_only implementation", module());

  //DispatchResult::Normal
}


#[inline]
pub fn opcode_return(ctx: &mut Context, hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_RETURN, 0);
  if ctx.cp.is_null() {
    if hp.stack_depth() == 0 {
      // Process end of life: return on empty stack
      panic!("{}Process exit: normal", module())
    } else {
      panic!("{}Return instruction with 0 in ctx.cp", module())
    }
  }

  ctx.ip = ctx.cp;
  ctx.cp = CodePtr::null();

  DispatchResult::Normal
}
