//! Module implements binary/bit syntax matching and data creation & extraction
//! opcodes for binaries.
use crate::{
  beam::disp_result::DispatchResult,
  defs::BitSize,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::{
    boxed::{
      self,
      binary::{
        match_state::BinaryMatchState, slice::BinarySlice, trait_interface::TBinary,
      },
    },
    lterm::*,
  },
};
use crate::defs::ByteSize;

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


/// Having started binary matching, retrieve a binary piece.
/// Structure: bs_get_binary(Fail, MatchState, Live, Size, Unit, Flags, Dst)
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
      "bgb2: live={} size={} unit={} flags={}",
      live, size, unit, flags
    );

    // Allocate a sub-binary and possibly GC if does not fit?
    let bit_size = BitSize::with_unit(size, unit);
    let src_bin = (*match_state).get_src_binary();

    if bit_size.bit_count > 0 {
      let bit_offset = (*match_state).get_offset();
      let sub_bin =
        BinarySlice::create_into(src_bin, bit_offset, bit_size, &mut proc.heap)?;
      (*match_state).increase_offset(bit_size);

      println!("bgb2: created {}", (*sub_bin).make_term());
      // Return the sub-binary created
      runtime_ctx.store_value((*sub_bin).make_term(), dst, &mut proc.heap)?;
    } else {
      // ignore error here, can't fail
      println!("bgb2: created empty <<>>");
      runtime_ctx
        .store_value(LTerm::empty_binary(), dst, &mut proc.heap)
        .unwrap();
    }

    Ok(DispatchResult::Normal)
  }
}

/// Having started binary matching, check that the match state has so many `Bits`
/// remaining otherwise will jump to the `Fail` label.
/// Structure: bs_test_tail2(Fail, MatchState, Bits)
define_opcode!(
  _vm, rt_ctx, proc, name: OpcodeBsTestTail2, arity: 3,
  run: { Self::bs_test_tail2(rt_ctx, proc, fail, match_state, bits) },
  args: cp_or_nil(fail), binary_match_state(match_state), load_usize(bits),
);


impl OpcodeBsTestTail2 {
  #[inline]
  fn bs_test_tail2(
    runtime_ctx: &mut Context,
    _proc: &mut Process,
    fail: LTerm,
    match_state: *mut BinaryMatchState,
    bits: usize,
  ) -> RtResult<DispatchResult> {
    let remaining = unsafe { (*match_state).get_bits_remaining().bit_count };
    if remaining != bits {
      runtime_ctx.jump(fail);
    }
    return Ok(DispatchResult::Normal);
  }
}

/// This instruction is rewritten on Erlang/OTP to `move S2, Dst`
/// Structure: bs_add(Fail, S1_ignored, S2, Unit, Dst)
define_opcode!(
  _vm, rt_ctx, proc, name: OpcodeBsAdd, arity: 5,
  run: { Self::bs_add(rt_ctx, proc, fail, s2, unit, dst) },
  args: cp_or_nil(fail), unused(s1), load(s2), usize(unit), term(dst),
);

impl OpcodeBsAdd {
  #[inline]
  fn bs_add(
    runtime_ctx: &mut Context,
    proc: &mut Process,
    _fail: LTerm,
    s2: LTerm,
    unit: usize,
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    // TODO: Rewrite this command into something shorter like Erlang/OTP does
    assert_eq!(unit, 1, "in bs_add unit arg is always expected to be 1");
    runtime_ctx.store_value(s2, dst, &mut proc.heap);
    return Ok(DispatchResult::Normal);
  }
}

/// Spec: bs_init2 Fail Sz Words Regs Flags Dst | binary_too_big(Sz) => system_limit Fail
define_opcode!(
  _vm, rt_ctx, proc, name: OpcodeBsInit2, arity: 6,
  run: { Self::bs_init2(rt_ctx, proc, fail, sz) },
  args: cp_or_nil(fail), load_usize(sz), unused(words), unused(regs),
        unused(flags), unused(dst),
);

impl OpcodeBsInit2 {
  #[inline]
  fn bs_init2(
    runtime_ctx: &mut Context,
    _proc: &mut Process,
    fail: LTerm,
    sz: usize,
  ) -> RtResult<DispatchResult> {
    if fail != LTerm::nil() && boxed::Binary::is_size_too_big(ByteSize::new(sz)) {
      runtime_ctx.jump(fail);
    }
    return Ok(DispatchResult::Normal);
  }
}
