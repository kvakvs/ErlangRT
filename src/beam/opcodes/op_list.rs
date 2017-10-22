//! Module implements opcodes related to lists manipulation.

//use emulator::gen_atoms;
use beam::gen_op;
use beam::opcodes::assert_arity;
use defs::DispatchResult;
use emulator::code::CodePtr;
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;


//fn module() -> &'static str { "opcodes::op_list: " }


#[inline]
pub fn opcode_is_nonempty_list(ctx: &mut Context,
                               _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_IS_NONEMPTY_LIST, 2);
  let fail = ctx.fetch_term(); // jump if not a list

  println!("ctx.ip: {}", ctx.ip);
  assert!(fail.is_cp() || fail.is_nil());

  let list = ctx.fetch_term();

  if list.is_nil() && !list.is_cons() {
    if !fail.is_nil() {
      // jump to fail label
      ctx.ip = CodePtr::from_cp(fail)
    }
  }

  DispatchResult::Normal
}


#[inline]
pub fn opcode_is_nil(ctx: &mut Context,
                     _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_IS_NIL, 2);
  let fail = ctx.fetch_term(); // jump if not a list
  assert!(fail.is_cp() || fail.is_nil());

  let list = ctx.fetch_term();

  if !list.is_nil() {
    if !fail.is_nil() {
      // jump to fail label
      ctx.ip = CodePtr::from_cp(fail)
    }
  }

  DispatchResult::Normal
}