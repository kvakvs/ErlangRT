use core::cmp::Ordering;

use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::{compare, lterm::LTerm},
};

/// Checks exact equality between arg1 and arg2, on false jump to arg0
/// Structure: is_eq_exact(on_false:CP, a:src, b:src)
pub struct OpcodeIsEqExact {}

impl OpcodeIsEqExact {
  pub const ARITY: usize = 3;

  #[inline]
  pub fn run(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    shared_equality_opcode(vm, ctx, curr_p, true, Ordering::Equal, false)
  }
}

/// Checks relation, that arg1 IS LESS than arg2, jump to arg0 otherwise.
/// Structure: is_lt(on_false:CP, a:src, b:src)
pub struct OpcodeIsLt {}

impl OpcodeIsLt {
  pub const ARITY: usize = 3;

  #[inline]
  pub fn run(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    shared_equality_opcode(vm, ctx, curr_p, true, Ordering::Less, false)
  }
}

/// Checks relation, that arg1 IS EQUAL(soft) to arg2, jump to arg0 otherwise.
/// Structure: is_eq(on_false:CP, a:src, b:src)
pub struct OpcodeIsEq {}

impl OpcodeIsEq {
  pub const ARITY: usize = 3;

  #[inline]
  pub fn run(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    shared_equality_opcode(vm, ctx, curr_p, false, Ordering::Equal, false)
  }
}

/// Checks relation, that arg1 IS NO LESS than arg2, jump to arg0 otherwise.
/// Structure: is_ge(on_false:CP, a:src, b:src)
pub struct OpcodeIsGe {}

impl OpcodeIsGe {
  pub const ARITY: usize = 3;

  #[inline]
  pub fn run(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    // inverted, other than less will be fail
    shared_equality_opcode(vm, ctx, curr_p, false, Ordering::Less, true)
  }
}

#[inline]
/// Shared code for equality checks. Assumes arg0 - fail label, arg1,2 - values
fn shared_equality_opcode(
  _vm: &mut VM,
  ctx: &mut Context,
  curr_p: &mut Process,
  exact: bool,
  desired_result: Ordering,
  invert: bool,
) -> RtResult<DispatchResult> {
  let hp = &curr_p.heap;
  let fail_label = ctx.fetch_term();
  let a = ctx.fetch_and_load(hp);
  let b = ctx.fetch_and_load(hp);

  assert_eq!(false, fail_label == LTerm::nil());

  if invert {
    // Invert defines opposite meaning, desired result becomes undesired
    if compare::cmp_terms(a, b, exact)? == desired_result {
      ctx.jump(fail_label)
    }
  } else if compare::cmp_terms(a, b, exact)? != desired_result {
    // Other than desired_recult will cause jump to 'fail'
    ctx.jump(fail_label)
  }

  Ok(DispatchResult::Normal)
}
