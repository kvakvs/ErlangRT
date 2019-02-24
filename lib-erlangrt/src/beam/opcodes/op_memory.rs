use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::{Error, RtResult},
  term::lterm::LTerm,
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
) -> RtResult<()> {
  ctx.live = live;

  let hp = &mut curr_p.heap;

  if !hp.heap_has_available(heap_need) {
    return Err(Error::HeapIsFull);
  }

  if hp.stack_have(stack_need + 1) {
    // Stack has enough words, we can allocate unchecked
    if stack_need > 0 {
      hp.stack_alloc_unchecked(stack_need, zero);
    }
    hp.stack_push_lterm_unchecked(ctx.cp.to_cp_term());
  } else {
    // Stack has not enough, invoke GC and possibly fail
    return Err(Error::HeapIsFull);
  }
  Ok(())
}

/// Allocate `need` words on stack, in case of GC use `live` amount of registers.
/// Structure: allocate_zero(need:int, live:int)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeAllocateZero, arity: 2,
  run: {
    shared_allocate(ctx, curr_p, stack_need, 0, live, true)?;
    Ok(DispatchResult::Normal)
  },
  args: usize(stack_need), usize(live),
);

/// Allocate `need` words on stack, in case of GC use `live` amount of registers.
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeAllocate, arity: 2,
  run: {
    shared_allocate(ctx, curr_p, stack_need, 0, live, false)?;
    Ok(DispatchResult::Normal)
  },
  args: usize(stack_need), usize(live),
);

/// Allocate `stack_need` words on stack, check that there's available
/// `heap_need` words on heap, in case of GC use `live` amount of registers.
/// Structure: allocate_heap_zero(stack_need:int, heap_need: int, live:int)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeAllocateHeapZero, arity: 3,
  run: {
    shared_allocate(ctx, curr_p, stack_need, heap_need, live, true)?;
    Ok(DispatchResult::Normal)
  },
  args: usize(stack_need), usize(heap_need), usize(live),
);

/// Allocate `need` words on heap, in case of GC use `live` amount of registers.
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeAllocateHeap, arity: 3,
  run: {
    shared_allocate(ctx, curr_p, stack_need, heap_need, live, false)?;
    Ok(DispatchResult::Normal)
  },
  args: usize(stack_need), usize(heap_need), usize(live),
);

/// Pop `cp` from the top of the stack and then deallocate additional `n_free`
/// words from the stack.
/// Structure: deallocate(n:int)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeDeallocate, arity: 1,
  run: {
    ctx.set_cp(curr_p.heap.stack_deallocate(free));
    Ok(DispatchResult::Normal)
  },
  args: usize(free),
);

/// Check that there are `heap_need` words available on heap, otherwise run the
/// GC using `live` amount of registers as a part of root set.
/// Structure: test_heap(heap_need:int, live:int)
define_opcode!(_vm, _ctx, curr_p,
  name: OpcodeTestHeap, arity: 2,
  run: { Self::test_heap(curr_p, heap_need) },
  args: usize(heap_need), unused(live),
);
// unsigned small 'live' will be used for gc

impl OpcodeTestHeap {
  #[inline]
  pub fn test_heap(
    curr_p: &mut Process,
    heap_need: usize,
  ) -> RtResult<DispatchResult> {
    if !curr_p.heap.heap_has_available(heap_need) {
      // Heap has not enough, invoke GC and possibly fail
      return Err(Error::HeapIsFull);
    }
    Ok(DispatchResult::Normal)
  }
}

/// Reduce the stack usage by N words, keeping CP on top of the stack.
/// Remaining value is used for?
/// Structure: trim(N:smallint, Remaining:smallint)
define_opcode!(_vm, _ctx, curr_p,
  name: OpcodeTrim, arity: 2,
  run: { Self::trim(curr_p, n_trim) },
  args: usize(n_trim), unused(remaining),
);

impl OpcodeTrim {
  #[inline]
  pub fn trim(
    curr_p: &mut Process,
    n_trim: usize
  ) -> RtResult<DispatchResult> {
    let hp = &mut curr_p.heap;
    let tmp_cp = hp.stack_deallocate(n_trim);
    // assume that after trimming the cp will fit back on stack just fine
    hp.stack_push_lterm_unchecked(tmp_cp);
    Ok(DispatchResult::Normal)
  }
}

/// Set Y-register on stack to NIL, offset is encoded inside the term.
/// Structure: init(yreg:special_yregister)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeInit, arity: 1,
  run: {
    curr_p.heap.set_y(y.get_special_value(), LTerm::nil())?;
    Ok(DispatchResult::Normal)
  },
  args: yreg(y),
);
