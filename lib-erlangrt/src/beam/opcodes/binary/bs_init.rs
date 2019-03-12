use crate::{
  beam::disp_result::DispatchResult,
  defs::{BitSize, ByteSize, WordSize},
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::{self, RtResult},
  term::{boxed, lterm::Term},
};

// Create a binary on proc heap or binary heap with GC if required.
// Sz - size of the binary, Words - some extra size to reserve on heap
//
// Spec:
// bs_init2 Fail Sz Words Regs Flags Dst | binary_too_big(Sz) => system_limit Fail
// bs_init2 Fail Sz Words Regs Flags Dst=y =>    bs_init2 Fail Sz Words Regs Flags x | move x Dst
// bs_init2 Fail Sz=u Words=u==0 Regs Flags Dst => i_bs_init Sz Regs Dst
// bs_init2 Fail Sz=u Words Regs Flags Dst =>    i_bs_init_heap Sz Words Regs Dst
// bs_init2 Fail Sz Words=u==0 Regs Flags Dst =>   i_bs_init_fail Sz Fail Regs Dst
// bs_init2 Fail Sz Words Regs Flags Dst =>   i_bs_init_fail_heap Sz Words Fail Regs Dst
// Example  bs_init2 [], X1, 0, 2, 0, X1
define_opcode!(
  vm, rt_ctx, proc, name: OpcodeBsInit2, arity: 6,
  run: { Self::bs_init2(vm, rt_ctx, proc, fail, sz, words, regs, flags, dst) },
  args: cp_or_nil(fail), load_usize(sz), usize(words), usize(regs),
        usize(flags), term(dst),
);

impl OpcodeBsInit2 {
  #[inline]
  fn bs_init2(
    vm: &mut VM,
    runtime_ctx: &mut Context,
    proc: &mut Process,
    fail: Term,
    sz: usize,
    words: usize,
    _regs: usize,
    _flags: usize,
    dst: Term,
  ) -> RtResult<DispatchResult> {
    if fail != Term::nil() && boxed::Binary::is_size_too_big(ByteSize::new(sz)) {
      return fail::create::system_limit();
    }
    if sz == 0 {
      // TODO: Check GC for extra words on heap
      runtime_ctx.store_value(Term::empty_binary(), dst, &mut proc.heap)?;
      return Ok(DispatchResult::Normal);
    }

    // Show intent to allocate memory; TODO: add GC related args, like live/regs
    boxed::Binary::ensure_memory_for_binary(
      vm,
      &mut proc.heap,
      BitSize::with_bytes(sz),
      WordSize::new(words),
    )?;

    let binary_size = BitSize::with_bytes(sz);
    let bin = unsafe { boxed::Binary::create_into(binary_size, &mut proc.heap)? };

    let bin_term = unsafe { (*bin).make_term() };
    runtime_ctx.current_bin.reset(bin_term);
    runtime_ctx.store_value(bin_term, dst, &mut proc.heap)?;
    Ok(DispatchResult::Normal)
  }
}
