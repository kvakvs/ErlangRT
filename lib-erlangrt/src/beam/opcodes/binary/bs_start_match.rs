use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context},
  fail::RtResult,
  term::{
    boxed::{
      self,
      binary::{match_state::BinaryMatchState, trait_interface::TBinary},
    },
    lterm::LTerm,
  },
};

/// Begin binary matching (version 2 used from OTP R11 to OTP 21 inclusive,
/// version 3 is OTP 22). Starts a binary match sequence.
///
/// Deprecated: bs_start_match2(fail, context:x|y, live:uint, {src,slots}, ctxr)
/// Structure: bs_start_match3(Fail Bin Live Dst)

// Erlang/OTP uses following rewrite rules
// bs_start_match3 Fail Bin Live Ctx | bs_get_position Ctx Pos=x Ignored
//    => i_bs_start_match3_gp Bin Live Fail Ctx Pos
// bs_start_match3 Fail=f ica Live Dst => jump Fail
// bs_start_match3 Fail Bin Live Dst => i_bs_start_match3 Bin Live Fail Dst
//
define_opcode!(
  _vm, rt_ctx, proc, name: OpcodeBsStartMatch3, arity: 4,
  run: { Self::bs_start_match_3(rt_ctx, proc, fail, match_context, live, dst) },
  args: cp_or_nil(fail), load(match_context), usize(live), term(dst),
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
    let trait_ptr = unsafe { (*header).get_trait_ptr() };
    let box_type = unsafe { (*trait_ptr).get_type() };

    // Switch based on the box type of the context...
    match box_type {
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
    // let _total_bin_size = unsafe { (*bin_ptr).get_byte_size() };
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
