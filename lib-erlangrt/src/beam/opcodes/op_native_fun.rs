//! Module implements opcodes related to calling built-in functions (BIF).

use crate::{
  beam::disp_result::DispatchResult,
  emulator::{
    process::Process,
    runtime_ctx::{call_native_fun, Context},
    vm::VM,
  },
  fail::RtResult,
  term::lterm::*,
};

/// Call a native_fun defined by `m:f/0`, a `HOImport` import object stored on heap
/// there is no way it can fail for bif0 so there is no fail label for bif0,
/// Result is stored into `dst`.
/// Structure: bif0(import:boxed, dst:dst)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeBif0, arity: 2,
  run: { Self::bif0(vm, ctx, curr_p, target, dst) },
  args: load(target), term(dst)
);

impl OpcodeBif0 {
  #[inline]
  fn bif0(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    target: LTerm,
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    // NOTE: bif0 cannot fail (fail_label=NIL)
    println!("bif0 t={} dst={}", target, dst);
    let cb_target = call_native_fun::CallBifTarget::ImportTerm(target);
    call_native_fun::find_and_call_native_fun(vm, ctx, curr_p, LTerm::nil(), cb_target, &[], dst, false)
  }
}

/// Structure: bif1(fail:cp, import:boxed, arg1:lterm, dst:dst)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeBif1, arity: 4,
  run: { Self::bif1(vm, ctx, curr_p, fail, target, bif_args, dst) },
  args: cp_not_nil(fail), load(target), slice(bif_args, 1), term(dst)
);

impl OpcodeBif1 {
  #[inline]
  fn bif1(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    fail: LTerm,
    target: LTerm,
    args: &[LTerm],
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    let cb_target = call_native_fun::CallBifTarget::ImportTerm(target);
    call_native_fun::find_and_call_native_fun(vm, ctx, curr_p, fail, cb_target, args, dst, false)
  }
}

/// Structure: bif1(fail:cp, import:boxed, arg1..2:lterm, dst:dst)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeBif2, arity: 5,
  run: { Self::bif2(vm, ctx, curr_p, fail, target, bif_args, dst) },
  args: cp_not_nil(fail), load(target), slice(bif_args, 2), term(dst)
);

impl OpcodeBif2 {
  #[inline]
  fn bif2(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    fail: LTerm,
    target: LTerm,
    args: &[LTerm],
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    let cb_target = call_native_fun::CallBifTarget::ImportTerm(target);
    call_native_fun::find_and_call_native_fun(vm, ctx, curr_p, fail, cb_target, args, dst, false)
  }
}

/// Structure: gc_bif1(fail:cp, live:small, import:boxed, arg1:lterm, dst:dst)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeGcBif1, arity: 5,
  run: { Self::gc_bif1(vm, ctx, curr_p, fail, live, target, bif_args, dst) },
  args: cp_not_nil(fail), usize(live), load(target), slice(bif_args, 1), term(dst)
);

impl OpcodeGcBif1 {
  #[inline]
  fn gc_bif1(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    fail: LTerm,
    live: usize,
    target: LTerm,
    args: &[LTerm],
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    ctx.live = live;
    let cb_target = call_native_fun::CallBifTarget::ImportTerm(target);
    call_native_fun::find_and_call_native_fun(vm, ctx, curr_p, fail, cb_target, args, dst, true)
  }
}

/// Structure: gc_bif2(fail:CP, live:small, import:boxed, arg1:lterm,
///                    arg2:lterm, dst:dst)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeGcBif2, arity: 6,
  run: { Self::gc_bif2(vm, ctx, curr_p, fail, live, target, bif_args, dst) },
  args: cp_not_nil(fail), usize(live), load(target), slice(bif_args, 2), term(dst)
);

impl OpcodeGcBif2 {
  #[inline]
  fn gc_bif2(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    fail: LTerm,
    live: usize,
    target: LTerm,
    args: &[LTerm],
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    ctx.live = live;
    let cb_target = call_native_fun::CallBifTarget::ImportTerm(target);
    call_native_fun::find_and_call_native_fun(vm, ctx, curr_p, fail, cb_target, args, dst, true)
  }
}

/// Structure: gc_bif3(fail:CP, live:small, import:boxed, arg1:lterm,
///                    arg2:lterm, arg3:lterm, dst:dst)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeGcBif3, arity: 7,
  run: { Self::gc_bif3(vm, ctx, curr_p, fail, live, target, bif_args, dst) },
  args: cp_not_nil(fail), usize(live), load(target), slice(bif_args, 3), term(dst)
);

impl OpcodeGcBif3 {
  #[inline]
  fn gc_bif3(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    fail: LTerm,
    live: usize,
    target: LTerm,
    args: &[LTerm],
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    ctx.live = live;
    let cb_target = call_native_fun::CallBifTarget::ImportTerm(target);
    call_native_fun::find_and_call_native_fun(vm, ctx, curr_p, fail, cb_target, args, dst, true)
  }
}
