use beam::gen_op;
use beam::opcodes::assert_arity;
use defs::{Word, DispatchResult};
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;
use term::lterm::LTerm;


#[inline]
pub fn opcode_allocate_zero(ctx: &mut Context, heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_ALLOCATE_ZERO, 2);
  let stack_need = ctx.fetch_term();
  let live = ctx.fetch_term();
  heap.stack_alloc(stack_need.small_get_u());
  heap.stack_push(ctx.cp.to_cp());

  DispatchResult::Normal
}


#[inline]
pub fn opcode_allocate(ctx: &mut Context, heap: &mut Heap) -> DispatchResult {
  opcode_allocate_zero(ctx, heap)
}
