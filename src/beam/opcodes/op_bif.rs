//! Module implements opcodes related to calling built-in functions (BIF).

use crate::{
  beam::{disp_result::DispatchResult},
  emulator::{
    process::Process,
    runtime_ctx::{call_bif, Context},
    vm::VM,
  },
  fail::RtResult,
  term::lterm::*,
};

/// Call a bif defined by `m:f/0`, a `HOImport` import object stored on heap
/// there is no way it can fail for bif0 so there is no fail label for bif0,
/// Result is stored into `dst`.
/// Structure: bif0(import:boxed, dst:dst)
pub struct OpcodeBif0 {}

impl OpcodeBif0 {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let target = ctx.fetch_and_load(&curr_p.heap);
    let dst = ctx.fetch_term();

    // Note: bif0 cannot fail (fail_label=NIL)

    let cb_target = call_bif::CallBifTarget::ImportTerm(target);
    call_bif::apply(ctx, curr_p, LTerm::nil(), cb_target, &[], dst, false)
  }
}

/// Structure: bif1(fail:cp, import:boxed, arg1:lterm, dst:dst)
pub struct OpcodeBif1 {}

impl OpcodeBif1 {
  pub const ARITY: usize = 4;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let fail = ctx.fetch_term();
    let target = ctx.fetch_and_load(&curr_p.heap);
    let args = ctx.fetch_slice(1);
    let dst = ctx.fetch_term();

    let cb_target = call_bif::CallBifTarget::ImportTerm(target);
    call_bif::apply(ctx, curr_p, fail, cb_target, args, dst, false)
  }
}

/// Structure: bif1(fail:cp, import:boxed, arg1..2:lterm, dst:dst)
pub struct OpcodeBif2 {}

impl OpcodeBif2 {
  pub const ARITY: usize = 5;
  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let fail = ctx.fetch_term();
    let target = ctx.fetch_and_load(&curr_p.heap);
    let args = ctx.fetch_slice(2);
    let dst = ctx.fetch_term();

    let cb_target = call_bif::CallBifTarget::ImportTerm(target);
    call_bif::apply(ctx, curr_p, fail, cb_target, args, dst, false)
  }
}

/// Structure: gc_bif1(fail:cp, live:small, import:boxed, arg1:lterm, dst:dst)
pub struct OpcodeGcBif1 {}

impl OpcodeGcBif1 {
  pub const ARITY: usize = 5;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let fail = ctx.fetch_term();
    ctx.live = ctx.fetch_term().get_small_unsigned();
    let target = ctx.fetch_and_load(&curr_p.heap);
    let args = ctx.fetch_slice(1);
    let dst = ctx.fetch_term();

    let cb_target = call_bif::CallBifTarget::ImportTerm(target);
    call_bif::apply(ctx, curr_p, fail, cb_target, args, dst, true)
  }
}

// Structure: gc_bif2(fail:CP, live:small, import:boxed, arg1:lterm,
//                    arg2:lterm, dst:dst)
pub struct OpcodeGcBif2 {}

impl OpcodeGcBif2 {
  pub const ARITY: usize = 6;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let fail = ctx.fetch_term();
    ctx.live = ctx.fetch_term().get_small_unsigned();
    let target = ctx.fetch_and_load(&curr_p.heap);
    let args = ctx.fetch_slice(2);
    let dst = ctx.fetch_term();

    let cb_target = call_bif::CallBifTarget::ImportTerm(target);
    call_bif::apply(ctx, curr_p, fail, cb_target, args, dst, true)
  }
}

/// Structure: gc_bif3(fail:CP, live:small, import:boxed, arg1:lterm,
///                    arg2:lterm, arg3:lterm, dst:dst)
pub struct OpcodeGcBif3 {}

impl OpcodeGcBif3 {
  pub const ARITY: usize = 7;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let fail = ctx.fetch_term();
    ctx.live = ctx.fetch_term().get_small_unsigned();
    let target = ctx.fetch_and_load(&curr_p.heap);
    let args = ctx.fetch_slice(3);
    let dst = ctx.fetch_term();

    let cb_target = call_bif::CallBifTarget::ImportTerm(target);
    call_bif::apply(ctx, curr_p, fail, cb_target, args, dst, true)
  }
}
