//! Module implements binary/bit syntax matching and data creation & extraction
//! opcodes for binaries.
use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::{
    boxed::{
      self,
      binary::{
        b_match::BinaryMatchState, bitsize::BitSize, slice::BinarySlice,
        trait_interface::TBinary,
      },
    },
    lterm::*,
  },
};

#[allow(dead_code)]
fn module() -> &'static str {
  "opcodes::bits&bins: "
}

/// Begin binary matching (version 2 used from OTP R11 to OTP 21 inclusive,
/// version 3 is OTP 22). Starts a binary match sequence.
///
/// Deprecated: bs_start_match2(fail, context:x|y, live:uint, {src,slots}, ctxr)
/// Structure: bs_start_match3(Fail Bin Live Dst)
define_opcode!(
  _vm, rt_ctx, proc, name: OpcodeBsStartMatch3, arity: 4,
  run: { Self::bs_start_match_3(rt_ctx, proc, fail, match_context, live, dst) },
  args: cp_not_nil(fail), load(match_context), usize(live), term(dst),
);

impl OpcodeBsStartMatch3 {
  #[inline]
  fn bs_start_match_3(
    runtime_ctx: &mut Context,
    proc: &mut Process,
    fail: LTerm,
    match_context: LTerm,
    live: usize,
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    println!(
      "bs_start_match3 {}, context={}, live={}, dst={}",
      fail, match_context, live, dst
    );

    // Must be either a binary or a binary_match_context
    if !match_context.is_boxed() {
      runtime_ctx.jump(fail);
      return Ok(DispatchResult::Normal);
    }

    let header = match_context.get_box_ptr_mut::<boxed::BoxHeader>();
    let h_tag = unsafe { (*header).get_tag() };

    // Switch based on the box type of the context...
    match h_tag {
      boxed::BOXTYPETAG_BINARY_MATCH_STATE => unsafe {
        return Self::continue_with_matchstate(
          runtime_ctx,
          proc,
          header as *mut BinaryMatchState,
          dst,
        );
      },

      boxed::BOXTYPETAG_BINARY => {
        let bin_ptr = unsafe { boxed::Binary::get_trait(header as *const boxed::Binary) };
        return Self::start_with_new_binary(runtime_ctx, proc, fail, bin_ptr, dst);
      }

      _ => {
        // Context must either be a binary or matchstate
        runtime_ctx.jump(fail);
        return Ok(DispatchResult::Normal);
      }
    }
    // Ok(DispatchResult::Normal)
  }

  /// When `bs_start_match*` is called with a binary, we allocate a new binary
  /// match context right here, and store it in the output register.
  fn start_with_new_binary(
    runtime_ctx: &mut Context,
    proc: &mut Process,
    _fail: LTerm,
    bin_ptr: *const TBinary,
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    let _total_bin_size = unsafe { (*bin_ptr).get_size() };
    // OTP has a guard for total_bin_size to fit in 2^(64-3)

    // Here we have a new start, matchstate does not exist and the context
    // is a binary. Have to construct a new match context.
    let new_match_state =
      unsafe { BinaryMatchState::create_into(bin_ptr, &mut proc.heap)? };

    // The binary, we're working on, is stored temporarily in x[live]
    // runtime_ctx.set_x(live, context);
    // TODO: GC if no space on heap, verify that GC gave us enough space
    // context = runtime_ctx.get_x(live);

    //    if new_match_state.is_null() {
    //      runtime_ctx.jump(fail);
    //      return Ok(DispatchResult::Normal);
    //    }
    runtime_ctx.store_value(LTerm::make_boxed(new_match_state), dst, &mut proc.heap)?;
    Ok(DispatchResult::Normal)
  }

  /// When `bs_start_match*` is called with a matchstate, which already exists
  /// on heap, we continue using that state.
  unsafe fn continue_with_matchstate(
    runtime_ctx: &mut Context,
    proc: &mut Process,
    match_state: *mut BinaryMatchState,
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    // Here we continue, matchstate has already been created, in context
    // Reset the position to the beginning
    (*match_state).reset();
    runtime_ctx.store_value(LTerm::make_boxed(match_state), dst, &mut proc.heap)?;
    Ok(DispatchResult::Normal)
  }
}


/// Having started binary matching, retrieve a binary piece.
///
/// Structure: bs_get_binary(Fail, MatchState, Live, Size, Unit, Flags, Dst)
define_opcode!(
  _vm, rt_ctx, proc, name: OpcodeBsGetBinary2, arity: 7,
  run: {unsafe {
    Self::bs_get_binary2_7(rt_ctx, proc, fail, match_state, live, size, unit, flags, dst)
  }},
  args: cp_not_nil(fail), binary_match_state(match_state),
        usize(live), load_usize(size), usize(unit), term(flags), term(dst),
);

impl OpcodeBsGetBinary2 {
  #[inline]
  unsafe fn bs_get_binary2_7(
    runtime_ctx: &mut Context,
    proc: &mut Process,
    _fail: LTerm,
    match_state: *mut BinaryMatchState,
    live: usize,
    size: usize,
    unit: usize,
    flags: LTerm,
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    println!(
      "bs_get_binary2 impl: live={} size={} unit={} flags={}",
      live, size, unit, flags
    );

    // Allocate a sub-binary and possibly GC if does not fit?
    let bit_size = BitSize::with_unit(size, unit);
    let src_bin = (*match_state).get_src_binary();
    let sub_bin = BinarySlice::create_into(src_bin, bit_size, &mut proc.heap)?;

    // Return the sub-binary created
    runtime_ctx.store_value((*sub_bin).make_term(), dst, &mut proc.heap)?;

    Ok(DispatchResult::Normal)
  }
}
