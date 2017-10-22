use beam::gen_op;
use beam::opcodes::assert_arity;
use defs::{DispatchResult};
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;
//use term::lterm::LTerm;


#[inline]
pub fn opcode_allocate_zero(ctx: &mut Context, hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_ALLOCATE_ZERO, 2);
  let stack_need_t = ctx.fetch_term();
  let stack_need = stack_need_t.small_get_u();

  if !hp.have(stack_need + 1) {
    // Stack has not enough, invoke GC and possibly fail
    let _live = ctx.fetch_term();
    panic!("TODO GC here or fail");
  } else {
    // Stack has enough words, we can allocate unchecked
    hp.stack_alloc_unchecked(stack_need);
    hp.stack_push_unchecked(ctx.cp.to_cp());
  }
  DispatchResult::Normal
}


#[inline]
pub fn opcode_allocate(ctx: &mut Context, hp: &mut Heap) -> DispatchResult {
  opcode_allocate_zero(ctx, hp)
}


/// Check that there are `heap_need` words available on heap, otherwise run the
/// GC using `live` amount of registers as a part of root set.
#[inline]
pub fn opcode_test_heap(ctx: &mut Context, hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_TEST_HEAP, 2);
  let heap_need = ctx.fetch_term().small_get_u();
  let live = ctx.fetch_term().small_get_u();

  if !hp.have(heap_need) {
    // Heap has not enough, invoke GC and possibly fail
    panic!("TODO GC here or fail");
  }

  DispatchResult::Normal
}