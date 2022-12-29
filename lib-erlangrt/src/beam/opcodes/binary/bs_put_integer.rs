use crate::{
  beam::disp_result::DispatchResult,
  defs::BitSize,
  emulator::{heap::THeapOwner, process::Process, runtime_ctx::*, vm::VM},
  fail::RtResult,
  term::Term,
};
use crate::beam::opcodes::binary::ArchUsize;

// Store `src` into the binary open for writing, the binary and the write
// position are stored in the process runtime context
// Erlang/OTP rewrites to:
// bs_put_integer Fail=j Sz=sq Unit=u Flags=u Src=s => gen_put_integer(Fail, Sz, Unit, Flags, Src)
define_opcode!(
  vm, rt_ctx, proc, name: OpcodeBsPutInteger, arity: 5,
  run: { Self::bs_put_integer(vm, rt_ctx, proc, fail, sz, unit, flags as ArchUsize, src) },
  args: cp_or_nil(fail), load_usize(sz), usize(unit), usize(flags), load(src),
);

impl OpcodeBsPutInteger {
  #[inline]
  #[allow(clippy::too_many_arguments)]
  fn bs_put_integer(
    _vm: &mut VM,
    ctx: &mut RuntimeContext,
    _proc: &mut Process,
    _fail: Term,
    arg_sz: usize,
    _unit: usize,
    flags: ArchUsize,
    src: Term,
  ) -> RtResult<DispatchResult> {
    debug_assert!(
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
