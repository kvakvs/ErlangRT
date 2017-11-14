use beam::gen_op;
use beam::opcodes::assert_arity;
use rt_defs::DispatchResult;
use emulator::code::CodePtr;
use emulator::process::Process;
use emulator::runtime_ctx::Context;
use term::lterm::*;


/// Allocate `need` words on heap, in case of GC use `live` amount of registers.
#[inline]
pub fn opcode_allocate_zero(ctx: &mut Context,
                            curr_p: &mut Process) -> DispatchResult {
  // Structure: allocate_zero(need:int, live:int)
  assert_arity(gen_op::OPCODE_ALLOCATE_ZERO, 2);

  let stack_need = ctx.fetch_term().small_get_u();
  let _live = ctx.fetch_term();

  let hp = &mut curr_p.heap;
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


/// Allocate `need` words on heap, in case of GC use `live` amount of registers.
#[inline]
pub fn opcode_allocate(ctx: &mut Context,
                       curr_p: &mut Process) -> DispatchResult {
  opcode_allocate_zero(ctx, curr_p)
}


/// Pop `cp` from the top of the stack and then deallocate additional `n_free`
/// words from the stack.
#[inline]
pub fn opcode_deallocate(ctx: &mut Context,
                         curr_p: &mut Process) -> DispatchResult {
  // Structure: deallocate(n:int)
  assert_arity(gen_op::OPCODE_DEALLOCATE, 1);

  let n_free = ctx.fetch_term().small_get_u();

  let new_cp = curr_p.heap.stack_deallocate(n_free);
  ctx.cp = CodePtr::from_cp(new_cp);

  DispatchResult::Normal
}


/// Check that there are `heap_need` words available on heap, otherwise run the
/// GC using `live` amount of registers as a part of root set.
#[inline]
pub fn opcode_test_heap(ctx: &mut Context,
                        curr_p: &mut Process) -> DispatchResult {
  // Structure: test_heap(heap_need:int, live:int)
  assert_arity(gen_op::OPCODE_TEST_HEAP, 2);

  let heap_need = ctx.fetch_term().small_get_u();
  let _live = ctx.fetch_term().small_get_u();

  if !curr_p.heap.have(heap_need) {
    // Heap has not enough, invoke GC and possibly fail
    panic!("TODO GC here or fail");
  }

  DispatchResult::Normal
}