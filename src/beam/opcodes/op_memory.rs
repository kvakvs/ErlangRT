use beam::gen_op;
use beam::opcodes::assert_arity;
use defs::DispatchResult;
use emulator::code::CodePtr;
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;
//use term::lterm::LTerm;


#[inline]
pub fn opcode_allocate_zero(ctx: &mut Context, hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_ALLOCATE_ZERO, 2);
  let stack_need = ctx.fetch_term().small_get_u();
  let _live = ctx.fetch_term();

  if hp.have(stack_need + 1) {
    // Stack has enough words, we can allocate unchecked
    if stack_need > 0 {
      hp.stack_alloc_unchecked(stack_need);
    }
    hp.stack_push_unchecked(ctx.cp.to_cp());
  } else {
    // Stack has not enough, invoke GC and possibly fail
    panic!("TODO GC here or fail");
  }

  // hp.stack_info();
  DispatchResult::Normal
}


#[inline]
pub fn opcode_allocate(ctx: &mut Context, hp: &mut Heap) -> DispatchResult {
  opcode_allocate_zero(ctx, hp)
}


#[inline]
pub fn opcode_deallocate(ctx: &mut Context,
                         hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_DEALLOCATE, 1);
  let n_free = ctx.fetch_term().small_get_u();

  let new_cp = hp.stack_deallocate(n_free);
  ctx.cp = CodePtr::from_cp(new_cp);

  DispatchResult::Normal
}


/// Check that there are `heap_need` words available on heap, otherwise run the
/// GC using `live` amount of registers as a part of root set.
#[inline]
pub fn opcode_test_heap(ctx: &mut Context, hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_TEST_HEAP, 2);
  let heap_need = ctx.fetch_term().small_get_u();
  let _live = ctx.fetch_term().small_get_u();

  if !hp.have(heap_need) {
    // Heap has not enough, invoke GC and possibly fail
    panic!("TODO GC here or fail");
  }

  DispatchResult::Normal
}