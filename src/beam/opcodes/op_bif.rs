//! Module implements opcodes related to calling built-in functions (BIF).

use crate::{
  beam::{disp_result::DispatchResult, gen_op, opcodes::assert_arity},
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
#[inline]
pub fn opcode_bif0(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: bif0(import:boxed, dst:dst)
  assert_arity(gen_op::OPCODE_BIF0, 2);

  let target = ctx.fetch_and_load(&curr_p.heap);
  let dst = ctx.fetch_term();

  // Note: bif0 cannot fail (fail_label=NIL)

  let cb_target = call_bif::CallBifTarget::ImportTerm(target);
  call_bif::apply(ctx, curr_p, LTerm::nil(), cb_target, &[], dst, false)
}

#[inline]
pub fn opcode_bif1(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: bif1(fail:cp, import:boxed, arg1:lterm, dst:dst)
  assert_arity(gen_op::OPCODE_BIF1, 4);

  let fail = ctx.fetch_term();
  let target = ctx.fetch_and_load(&curr_p.heap);
  let args = ctx.fetch_slice(1);
  let dst = ctx.fetch_term();

  let cb_target = call_bif::CallBifTarget::ImportTerm(target);
  call_bif::apply(ctx, curr_p, fail, cb_target, args, dst, false)
}

#[inline]
pub fn opcode_bif2(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: bif1(fail:cp, import:boxed, arg1..2:lterm, dst:dst)
  assert_arity(gen_op::OPCODE_BIF2, 5);

  let fail = ctx.fetch_term();
  let target = ctx.fetch_and_load(&curr_p.heap);
  let args = ctx.fetch_slice(2);
  let dst = ctx.fetch_term();

  let cb_target = call_bif::CallBifTarget::ImportTerm(target);
  call_bif::apply(ctx, curr_p, fail, cb_target, args, dst, false)
}

#[inline]
pub fn opcode_gc_bif1(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: gc_bif1(fail:cp, live:small, import:boxed, arg1:lterm, dst:dst)
  assert_arity(gen_op::OPCODE_GC_BIF1, 5);

  let fail = ctx.fetch_term();
  ctx.live = ctx.fetch_term().get_small_unsigned();
  let target = ctx.fetch_and_load(&curr_p.heap);
  let args = ctx.fetch_slice(1);
  let dst = ctx.fetch_term();

  let cb_target = call_bif::CallBifTarget::ImportTerm(target);
  call_bif::apply(ctx, curr_p, fail, cb_target, args, dst, true)
}

#[inline]
pub fn opcode_gc_bif2(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: gc_bif2(fail:CP, live:small, import:boxed, arg1:lterm,
  //                    arg2:lterm, dst:dst)
  assert_arity(gen_op::OPCODE_GC_BIF2, 6);

  let fail = ctx.fetch_term();
  ctx.live = ctx.fetch_term().get_small_unsigned();
  let target = ctx.fetch_and_load(&curr_p.heap);
  let args = ctx.fetch_slice(2);
  let dst = ctx.fetch_term();

  let cb_target = call_bif::CallBifTarget::ImportTerm(target);
  call_bif::apply(ctx, curr_p, fail, cb_target, args, dst, true)
}

#[inline]
pub fn opcode_gc_bif3(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: gc_bif3(fail:CP, live:small, import:boxed, arg1:lterm,
  //                    arg2:lterm, arg3:lterm, dst:dst)
  assert_arity(gen_op::OPCODE_GC_BIF2, 7);

  let fail = ctx.fetch_term();
  ctx.live = ctx.fetch_term().get_small_unsigned();
  let target = ctx.fetch_and_load(&curr_p.heap);
  let args = ctx.fetch_slice(3);
  let dst = ctx.fetch_term();

  let cb_target = call_bif::CallBifTarget::ImportTerm(target);
  call_bif::apply(ctx, curr_p, fail, cb_target, args, dst, true)
}
