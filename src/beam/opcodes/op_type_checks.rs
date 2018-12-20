use crate::emulator::vm::VM;
use crate::emulator::runtime_ctx::Context;
use crate::emulator::process::Process;
use crate::fail::RtResult;
use crate::beam::disp_result::DispatchResult;
use crate::beam::gen_op;
use crate::beam::opcodes::assert_arity;
use crate::emulator::code::pointer::CodePtr;

/// Checks that argument is an atom, otherwise jumps to label.
#[inline]
pub fn opcode_is_atom(
  vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: is_atom(on_false:label, val:src)
  assert_arity(gen_op::OPCODE_IS_ATOM, 2);
  let hp = &curr_p.heap;
  let fail_label = ctx.fetch_term();
  let val = ctx.fetch_and_load(hp);

  if !val.is_atom() {
    ctx.ip = CodePtr::from_cp(fail_label)
  }

  Ok(DispatchResult::Normal)
}
