use crate::{
  beam::disp_result::DispatchResult,
  defs::ByteSize,
  emulator::{process::Process, runtime_ctx::Context},
  fail::RtResult,
  term::{boxed, lterm::Term},
};

/// Spec:
/// bs_init2 Fail Sz Words Regs Flags Dst | binary_too_big(Sz) => system_limit Fail
/// bs_init2 Fail Sz Words Regs Flags Dst=y =>    bs_init2 Fail Sz Words Regs Flags x | move x Dst
/// bs_init2 Fail Sz=u Words=u==0 Regs Flags Dst => i_bs_init Sz Regs Dst
/// bs_init2 Fail Sz=u Words Regs Flags Dst =>    i_bs_init_heap Sz Words Regs Dst
/// bs_init2 Fail Sz Words=u==0 Regs Flags Dst =>   i_bs_init_fail Sz Fail Regs Dst
/// bs_init2 Fail Sz Words Regs Flags Dst =>   i_bs_init_fail_heap Sz Words Fail Regs Dst
// Example  bs_init2 [], X1, 0, 2, 0, X1
define_opcode!(
  _vm, rt_ctx, proc, name: OpcodeBsInit2, arity: 6,
  run: { Self::bs_init2(rt_ctx, proc, fail, sz, words, regs, flags, dst) },
  args: cp_or_nil(fail), load_usize(sz), usize(words), usize(regs),
        usize(flags), term(dst),
);

impl OpcodeBsInit2 {
  #[inline]
  fn bs_init2(
    runtime_ctx: &mut Context,
    _proc: &mut Process,
    fail: Term,
    sz: usize,
    words: usize,
    regs: usize,
    flags: usize,
    dst: Term,
  ) -> RtResult<DispatchResult> {
    if fail != Term::nil() && boxed::Binary::is_size_too_big(ByteSize::new(sz)) {
      runtime_ctx.jump(fail);
    }
    return Ok(DispatchResult::Normal);
  }
}
