use crate::{
  beam::disp_result::DispatchResult,
  defs::BitSize,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::value::Term,
};

// Store `src` into the binary open for writing, the binary and the write
// position are stored in the process runtime context
// Spec: bs_put_binary Fail=j Sz=s Unit=u Flags=u Src=s
define_opcode!(
  vm, rt_ctx, proc, name: OpcodeBsPutBinary, arity: 5,
  run: { Self::bs_put_binary(vm, rt_ctx, proc, fail, sz, unit, flags, src) },
  args: cp_or_nil(fail), load_usize(sz), usize(unit), usize(flags), load(src),
);

impl OpcodeBsPutBinary {
  #[inline]
  fn bs_put_binary(
    _vm: &mut VM,
    ctx: &mut Context,
    _proc: &mut Process,
    _fail: Term,
    arg_sz: usize,
    _unit: usize,
    flags: usize,
    src: Term,
  ) -> RtResult<DispatchResult> {
    debug_assert!(
      ctx.current_bin.valid(),
      "Attempt to bs_put_binary with no ctx.current_bin"
    );
    let dst_binary = ctx.current_bin.dst.unwrap();
    let sz = BitSize::with_bits(arg_sz);
    unsafe {
      (*dst_binary).put_binary(
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
