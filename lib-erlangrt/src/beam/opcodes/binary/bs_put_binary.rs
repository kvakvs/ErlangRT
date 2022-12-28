use crate::{
  beam::disp_result::DispatchResult,
  defs::BitSize,
  emulator::{
    gen_atoms, heap::THeapOwner, process::Process, runtime_ctx::*, vm::VM,
  },
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      self,
      binary::bits_paste::{self, SizeOrAll},
    },
    Term,
  },
};
use crate::beam::opcodes::binary::ArchUsize;

// Store `src` into the binary open for writing, the binary and the write
// position are stored in the process runtime context.
// Arg: sz can have a special value 'all'.
// Spec: bs_put_binary Fail=j Sz=s Unit=u Flags=u Src=s
define_opcode!(
  vm, rt_ctx, proc, name: OpcodeBsPutBinary, arity: 5,
  run: {
    Self::bs_put_binary(vm, rt_ctx, proc, fail, sz, unit, flags as ArchUsize, src)
  },
  args: cp_or_nil(fail), load(sz), usize(unit), usize(flags), load(src),
);

impl OpcodeBsPutBinary {
  /// Given size arg which can be either small unsigned or atom `all`, create
  /// a `SizeOrAll` value for put_binary.
  #[inline]
  fn get_size_or_all(size: Term) -> SizeOrAll {
    if size == gen_atoms::ALL {
      return SizeOrAll::All;
    }
    SizeOrAll::Bits(BitSize::with_bits(size.get_small_unsigned()))
  }

  /// Put Binary opcode with the size
  #[inline]
  fn bs_put_binary(
    _vm: &mut VM,
    ctx: &mut RuntimeContext,
    _proc: &mut Process,
    fail: Term,
    in_size_term: Term,
    _unit: usize,
    flags: ArchUsize,
    src: Term,
  ) -> RtResult<DispatchResult> {
    debug_assert!(
      ctx.current_bin.valid(),
      "bs_put_binary with no ctx.current_bin"
    );

    let _dst_binary = ctx.current_bin.dst.unwrap();
    let size_or_all = Self::get_size_or_all(in_size_term);

    unsafe {
      match bits_paste::put_binary(
        ctx.current_bin.dst.unwrap(),
        size_or_all,
        boxed::Binary::get_trait_mut_from_term(src),
        ctx.current_bin.offset,
        crate::beam::opcodes::BsFlags::from_bits_truncate(flags),
      ) {
        Ok(copied_size) => {
          ctx.current_bin.offset = ctx.current_bin.offset + copied_size;
          Ok(DispatchResult::Normal)
        }
        Err(RtErr::BinaryDestinationTooSmall) => {
          ctx.jump(fail);
          Ok(DispatchResult::Normal)
        }
        Err(err) => {
          // Rewrap the error into result type for opcode
          Err(err)
        }
      }
    }
  }
}
