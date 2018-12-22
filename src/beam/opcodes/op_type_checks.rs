use crate::beam::disp_result::DispatchResult;
use crate::beam::gen_op;
use crate::beam::opcodes::assert_arity;
use crate::emulator::process::Process;
use crate::emulator::runtime_ctx::Context;
use crate::emulator::vm::VM;
use crate::fail::RtResult;

/// Checks that argument is an atom, otherwise jumps to label.
#[inline]
pub fn opcode_is_atom(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: is_atom(on_false:label, val:src)
  assert_arity(gen_op::OPCODE_IS_ATOM, 2);
  let hp = &curr_p.heap;
  let fail_label = ctx.fetch_term();
  let val = ctx.fetch_and_load(hp);

  if !val.is_atom() {
    ctx.jump(fail_label)
  }

  Ok(DispatchResult::Normal)
}

/// Checks that argument is a function or closure otherwise jumps to label.
/// Structure: is_function(on_false:label, arg:src)
#[inline]
pub fn opcode_is_function(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  assert_arity(gen_op::OPCODE_IS_FUNCTION, 2);
  let hp = &curr_p.heap;
  let fail_label = ctx.fetch_term();
  let val = ctx.fetch_and_load(hp);

  if !val.is_fun() {
    ctx.jump(fail_label)
  }

  Ok(DispatchResult::Normal)
}

/// Checks that argument is a function or closure otherwise jumps to label.
/// Structure: is_function(on_false:label, arg:src)
#[inline]
pub fn opcode_is_function2(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  assert_arity(gen_op::OPCODE_IS_FUNCTION2, 3);
  let hp = &curr_p.heap;
  let fail_label = ctx.fetch_term();
  let val = ctx.fetch_and_load(hp);
  let arity = ctx.fetch_and_load(hp).get_small_unsigned();

  if !val.is_fun_of_arity(arity) {
    ctx.jump(fail_label)
  }

  Ok(DispatchResult::Normal)
}
