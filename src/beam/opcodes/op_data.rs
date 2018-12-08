//! Module implements opcodes related to reading, writing, and moving data.

use crate::{
  beam::{disp_result::DispatchResult, gen_op, opcodes::assert_arity},
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
};


/// Load a value from `src` and store it into `dst`. Source can be any literal
/// term, a register or a stack cell. Destination can be any register or a
/// stack cell.
#[inline]
pub fn opcode_move(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: move(src:src, dst:dst)
  // TODO: Optimize this by having specialized move instructions with packed arg
  assert_arity(gen_op::OPCODE_MOVE, 2);

  let src = ctx.fetch_term();
  let dst = ctx.fetch_term();
  ctx.store(src, dst, &mut curr_p.heap);

  Ok(DispatchResult::Normal)
}
