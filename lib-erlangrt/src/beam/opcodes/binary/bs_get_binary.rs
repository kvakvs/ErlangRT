use crate::{
  beam::disp_result::DispatchResult,
  defs::BitSize,
  emulator::{heap::THeapOwner, process::Process, runtime_ctx::*},
  fail::RtResult,
  term::{
    boxed::binary::{match_state::BinaryMatchState, BinarySlice},
    Term,
  },
};

// Having started binary matching, retrieve a binary piece.
// Structure: bs_get_binary(Fail, MatchState, Live, Size, Unit, Flags, Dst)
define_opcode!(
  _vm, rt_ctx, proc, name: OpcodeBsGetBinary2, arity: 7,
  run: {unsafe {
    Self::bs_get_binary2_7(rt_ctx, proc, fail, match_state, live, size, unit, flags, dst)
  }},
  args: cp_or_nil(fail), binary_match_state(match_state),
        usize(live), load_usize(size), usize(unit), term(flags), term(dst),
);

impl OpcodeBsGetBinary2 {
  #[inline]
  #[allow(clippy::too_many_arguments)]
  unsafe fn bs_get_binary2_7(
    runtime_ctx: &mut RuntimeContext,
    proc: &mut Process,
    _fail: Term,
    match_state: *mut BinaryMatchState,
    live: usize,
    size: usize,
    unit: usize,
    _flags: Term,
    dst: Term,
  ) -> RtResult<DispatchResult> {
    // Allocate a sub-binary and possibly GC if does not fit?
    let bit_size = BitSize::with_unit(size, unit);
    let src_bin = (*match_state).get_src_binary();

    if !bit_size.is_empty() {
      runtime_ctx.live = live;
      proc.ensure_heap(BinarySlice::storage_size())?;

      // Create slice
      let bit_offset = (*match_state).get_offset();
      let slice =
        BinarySlice::create_into(src_bin, bit_offset, bit_size, proc.get_heap_mut())?;
      (*match_state).increase_offset(bit_size);

      // Return the slice (sub-binary) created
      runtime_ctx.store_value((*slice).make_term(), dst, proc.get_heap_mut())?;
    } else {
      // ignore error here, can't fail
      runtime_ctx
        .store_value(Term::empty_binary(), dst, proc.get_heap_mut())
        .unwrap();
    }

    Ok(DispatchResult::Normal)
  }
}
