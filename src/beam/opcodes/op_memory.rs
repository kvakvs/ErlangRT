use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
};

/// Allocate `need` words on heap, in case of GC use `live` amount of registers.
/// Structure: allocate_zero(need:int, live:int)
pub struct OpcodeAllocateZero {}

impl OpcodeAllocateZero {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let stack_need = ctx.fetch_term().get_small_unsigned();
    let _live = ctx.fetch_term();

    let hp = &mut curr_p.heap;
    if hp.stack_have(stack_need + 1) {
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
    Ok(DispatchResult::Normal)
  }
}

/// Allocate `need` words on heap, in case of GC use `live` amount of registers.
pub struct OpcodeAllocate {}

impl OpcodeAllocate {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    OpcodeAllocateZero::run(vm, ctx, curr_p)
  }
}

/// Pop `cp` from the top of the stack and then deallocate additional `n_free`
/// words from the stack.
/// Structure: deallocate(n:int)
pub struct OpcodeDeallocate {}

impl OpcodeDeallocate {
  pub const ARITY: usize = 1;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let n_free = ctx.fetch_term().get_small_unsigned();
    ctx.set_cp(curr_p.heap.stack_deallocate(n_free));

    Ok(DispatchResult::Normal)
  }
}

/// Check that there are `heap_need` words available on heap, otherwise run the
/// GC using `live` amount of registers as a part of root set.
/// Structure: test_heap(heap_need:int, live:int)
pub struct OpcodeTestHeap {}

impl OpcodeTestHeap {
  pub const ARITY: usize = 2;
  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let heap_need = ctx.fetch_term().get_small_unsigned();
    let _live = ctx.fetch_term().get_small_unsigned();

    if !curr_p.heap.have(heap_need) {
      // Heap has not enough, invoke GC and possibly fail
      panic!("TODO GC here or fail");
    }

    Ok(DispatchResult::Normal)
  }
}

/// Reduce the stack usage by N words, keeping CP on top of the stack.
/// Remaining value is used for?
/// Structure: trim(N:smallint, Remaining:smallint)
pub struct OpcodeTrim {}

impl OpcodeTrim {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let trim = ctx.fetch_term().get_small_unsigned();
    let _remaining = ctx.fetch_term();

    let hp = &mut curr_p.heap;
    let tmp_cp = hp.stack_deallocate(trim);
    // assume that after trimming the cp will fit back on stack just fine
    hp.stack_push_lterm_unchecked(tmp_cp);

    Ok(DispatchResult::Normal)
  }
}
