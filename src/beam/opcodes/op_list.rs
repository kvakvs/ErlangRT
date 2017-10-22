//! Module implements opcodes related to lists manipulation.

use emulator::gen_atoms;
use beam::gen_op;
use beam::opcodes::assert_arity;
use defs::DispatchResult;
use emulator::code::CodePtr;
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;


fn module() -> &'static str { "opcodes::op_list: " }


#[inline]
pub fn opcode_is_nonempty_list(ctx: &mut Context,
                               _heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_IS_NONEMPTY_LIST, 2);
  let fail = ctx.fetch_term(); // jump if not a list
  let list = ctx.fetch_term();

  if list.is_nil() {
    ctx.regs[0] = gen_atoms::FALSE
  } else if list.is_cons() {
    // Cons has at least 1 element, so is non-empty
    ctx.regs[0] = gen_atoms::TRUE
  } else {
    if fail.is_nil() {
      // return false silently
      ctx.regs[0] = gen_atoms::FALSE
    } else {
      // jump to fail label
      ctx.ip = CodePtr::from_cp(fail)
    }
  }

  DispatchResult::Normal
}
