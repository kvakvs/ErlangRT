use crate::{
  beam::disp_result::DispatchResult,
  defs::BitSize,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::lterm::Term,
};

/// Store `src` into ??? context with unit size
/// Erlang/OTP rewrites to:
/// bs_put_integer Fail=j Sz=sq Unit=u Flags=u Src=s => gen_put_integer(Fail, Sz, Unit, Flags, Src)
define_opcode!(
  vm, rt_ctx, proc, name: OpcodeBsPutInteger, arity: 5,
  run: { Self::bs_put_integer(vm, rt_ctx, proc, fail, sz, unit, flags, src) },
  args: cp_or_nil(fail), load_usize(sz), usize(unit), usize(flags), load(src),
);

impl OpcodeBsPutInteger {
  #[inline]
  fn bs_put_integer(
    _vm: &mut VM,
    ctx: &mut Context,
    _proc: &mut Process,
    _fail: Term,
    arg_sz: usize,
    _unit: usize,
    flags: usize,
    src: Term,
  ) -> RtResult<DispatchResult> {
    assert!(
      ctx.current_bin.valid(),
      "Attempt to bs_put_integer with no ctx.current_bin"
    );
    let dst_binary = ctx.current_bin.dst.unwrap();
    let sz = BitSize::with_bits(arg_sz);
    unsafe {
      (*dst_binary).put_integer(
        src,
        sz,
        ctx.current_bin.offset,
        crate::beam::opcodes::BsFlags::from_bits_truncate(flags),
      )?;
    }
    ctx.current_bin.offset = ctx.current_bin.offset + sz;
    Ok(DispatchResult::Normal)
  }
}
