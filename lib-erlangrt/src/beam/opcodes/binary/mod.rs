//! Module implements binary/bit syntax matching and data creation & extraction
//! opcodes for binaries.
pub mod bs_init;

pub use bs_init::*;

pub mod bs_start_match;
pub use bs_start_match::*;

pub mod bs_get_binary;
pub use bs_get_binary::*;

use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context},
  fail::RtResult,
  term::{boxed::binary::match_state::BinaryMatchState, lterm::*},
};

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
  run: {
    rt_ctx.store_value(s2, dst, &mut proc.heap)?;
    Ok(DispatchResult::Normal)
  },
  args: cp_or_nil(fail), IGNORE(s1), load(s2), IGNORE(unit), term(dst),
);
