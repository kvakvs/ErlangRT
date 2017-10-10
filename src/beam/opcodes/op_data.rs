//! Module implements opcodes related to reading, writing, and moving data.
use beam::gen_op;
use emulator::code::CodePtr;
use beam::opcodes::assert_arity;
use defs::{Word, DispatchResult};
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;
use term::lterm::LTerm;


#[inline]
pub fn opcode_move(ctx: &mut Context, hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_MOVE, 2);
  let src = ctx.fetch_term();
  let dst = ctx.fetch_term();
  ctx.store(src, dst, hp);
  DispatchResult::Normal
}
