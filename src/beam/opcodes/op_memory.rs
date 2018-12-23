use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
};

/// Shared code for stack checks and allocations with an optional heap check.
#[inline]
fn shared_allocate(
  ctx: &mut Context,
  curr_p: &mut Process,
  stack_need: usize,
  heap_need: usize,
  live: usize,
  zero: bool,
) {
  ctx.live = live;

  let hp = &mut curr_p.heap;

  if !hp.have(heap_need) {
    panic!("Heap doesn't have {} words", heap_need);
  }

  if hp.stack_have(stack_need + 1) {
    // Stack has enough words, we can allocate unchecked
    if stack_need > 0 {
      hp.stack_alloc_unchecked(stack_need, zero);
    }
    hp.stack_push_unchecked(ctx.cp.to_cp());
  } else {
    // Stack has not enough, invoke GC and possibly fail
    panic!("TODO GC here or fail");
  }
}

/// Allocate `need` words on stack, in case of GC use `live` amount of registers.
/// Structure: allocate_zero(need:int, live:int)
pub struct OpcodeAllocateZero {}

impl OpcodeAllocateZero {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context) -> (usize, usize) {
    let need = ctx.fetch_term().get_small_unsigned();
    let live = ctx.fetch_term().get_small_unsigned();
    (need, live)
  }

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (stack_need, live) = Self::fetch_args(ctx);
    shared_allocate(ctx, curr_p, stack_need, 0, live, true);
    Ok(DispatchResult::Normal)
  }
}

/// Allocate `need` words on stack, in case of GC use `live` amount of registers.
pub struct OpcodeAllocate {}

impl OpcodeAllocate {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context) -> (usize, usize) {
    let need = ctx.fetch_term().get_small_unsigned();
    let live = ctx.fetch_term().get_small_unsigned();
    (need, live)
  }

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (stack_need, live) = Self::fetch_args(ctx);
    shared_allocate(ctx, curr_p, stack_need, 0, live, false);
    Ok(DispatchResult::Normal)
  }
}


/// Allocate `stack_need` words on stack, check that there's available
/// `heap_need` words on heap, in case of GC use `live` amount of registers.
/// Structure: allocate_heap_zero(stack_need:int, heap_need: int, live:int)
pub struct OpcodeAllocateHeapZero {}

impl OpcodeAllocateHeapZero {
  pub const ARITY: usize = 3;

  #[inline]
  fn fetch_args(ctx: &mut Context) -> (usize, usize, usize) {
    let stack_need = ctx.fetch_term().get_small_unsigned();
    let heap_need = ctx.fetch_term().get_small_unsigned();
    let live = ctx.fetch_term().get_small_unsigned();
    (stack_need, heap_need, live)
  }

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (stack_need, heap_need, live) = Self::fetch_args(ctx);
    shared_allocate(ctx, curr_p, stack_need, heap_need, live, true);
    Ok(DispatchResult::Normal)
  }
}

/// Allocate `need` words on heap, in case of GC use `live` amount of registers.
pub struct OpcodeAllocateHeap {}

impl OpcodeAllocateHeap {
  pub const ARITY: usize = 3;

  #[inline]
  fn fetch_args(ctx: &mut Context) -> (usize, usize, usize) {
    let stack_need = ctx.fetch_term().get_small_unsigned();
    let heap_need = ctx.fetch_term().get_small_unsigned();
    let live = ctx.fetch_term().get_small_unsigned();
    (stack_need, heap_need, live)
  }

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (stack_need, heap_need, live) = Self::fetch_args(ctx);
    shared_allocate(ctx, curr_p, stack_need, heap_need, live, false);
    Ok(DispatchResult::Normal)
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
    _vm: &mut VM,
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
    _vm: &mut VM,
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
    _vm: &mut VM,
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
