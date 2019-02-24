//! Module implements binary/bit syntax matching and data creation & extraction
//! opcodes for binaries.
use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::{
    boxed::{self, box_header::BoxHeader},
    lterm::*,
  },
};

#[allow(dead_code)]
fn module() -> &'static str {
  "opcodes::bits&bins: "
}

/// Begin binary matching (version 2 used from OTP R11 to OTP 21 inclusive).
/// Structure: bs_start_match2(fail, context:x|y, live:uint, {src,slots}, ctxr)
define_opcode!(
  _vm, ctx, curr_p, name: OpcodeBsStartMatch2, arity: 5,
  run: { Self::bs_start_match_2(ctx, fail, context) },
  args: cp_not_nil(fail), load(context), unused(_usize_live), unused(_term_src),
        unused(_usize_slots), load(_ctxr),
);

impl OpcodeBsStartMatch2 {
  #[inline]
  fn bs_start_match_2(
    runtime_ctx: &mut Context,
    fail: LTerm,
    context: LTerm,
  ) -> RtResult<DispatchResult> {
    // Must be either a binary or a binary_match_context
    if !context.is_boxed() {
      runtime_ctx.jump(fail);
      return Ok(DispatchResult::Normal);
    }

    // Switch based on the box type of the context...
    let header = context.get_box_ptr_mut::<BoxHeader>();
    match unsafe { (*header).get_tag() } {
      boxed::BOXTYPETAG_BINARY => {}
      boxed::BOXTYPETAG_BINARY_MATCH_CTX => {}
      _ => {
        runtime_ctx.jump(fail);
        return Ok(DispatchResult::Normal);
      }
    }

    Ok(DispatchResult::Normal)
  }
}

// impl OpcodeBsStartMatch2 {
//  // TODO: Define a smarter way to fetch only args which are used
//  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (LTerm, LTerm, LTerm, usize, LTerm) {
//    let fail = ctx.fetch_term();
//    let context = ctx.fetch_and_load(&mut curr_p.heap);
//    ctx.live = ctx.fetch_term().get_small_unsigned();
//    let src = ctx.fetch_term();
//    let slots = ctx.fetch_term().get_small_unsigned();
//    let ctxr = ctx.fetch_term();
//    (fail, context, src, slots, ctxr)
//  }
//}
