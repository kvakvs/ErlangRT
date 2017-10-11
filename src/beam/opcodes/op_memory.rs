use beam::gen_op;
use beam::opcodes::assert_arity;
use defs::{DispatchResult};
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;
//use term::lterm::LTerm;


#[inline]
pub fn opcode_allocate_zero(ctx: &mut Context, heap: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_ALLOCATE_ZERO, 2);
  let stack_need_t = ctx.fetch_term();
  let stack_need = stack_need_t.small_get_u();

  if !heap.stack_have(stack_need + 1) {
    // Stack has not enough, invoke GC and possibly fail
    let live = ctx.fetch_term();
    panic!("TODO GC here or fail");
  } else {
    // Stack has enough words, we can allocate unchecked
    heap.stack_alloc_unchecked(stack_need);
    heap.stack_push_unchecked(ctx.cp.to_cp());
  }
  DispatchResult::Normal
}


#[inline]
pub fn opcode_allocate(ctx: &mut Context, heap: &mut Heap) -> DispatchResult {
  opcode_allocate_zero(ctx, heap)
}
