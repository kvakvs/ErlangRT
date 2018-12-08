use std::cmp::Ordering;

use crate::{
  beam::{disp_result::DispatchResult, gen_op, opcodes::assert_arity},
  emulator::{code::CodePtr, process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::{compare, lterm::LTerm},
};


/// Checks exact equality between arg1 and arg2, on false jump to arg0
#[inline]
pub fn opcode_is_eq_exact(
  vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: is_eq_exact(on_false:CP, a:src, b:src)
  assert_arity(gen_op::OPCODE_IS_EQ_EXACT, 3);
  shared_equality_opcode(vm, ctx, curr_p, true, Ordering::Equal, false)
}


/// Checks relation, that arg1 IS LESS than arg2, jump to arg0 otherwise.
#[inline]
pub fn opcode_is_lt(
  vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: is_lt(on_false:CP, a:src, b:src)
  assert_arity(gen_op::OPCODE_IS_LT, 3);
  shared_equality_opcode(vm, ctx, curr_p, true, Ordering::Less, false)
}


/// Checks relation, that arg1 IS EQUAL(soft) to arg2, jump to arg0 otherwise.
#[inline]
pub fn opcode_is_eq(
  vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: is_eq(on_false:CP, a:src, b:src)
  assert_arity(gen_op::OPCODE_IS_EQ, 3);
  shared_equality_opcode(vm, ctx, curr_p, false, Ordering::Equal, false)
}


/// Checks relation, that arg1 IS NO LESS than arg2, jump to arg0 otherwise.
#[inline]
pub fn opcode_is_ge(
  vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: is_eq(on_false:CP, a:src, b:src)
  assert_arity(gen_op::OPCODE_IS_EQ, 3);
  shared_equality_opcode(vm, ctx, curr_p, false, Ordering::Less, true) // inverted, other than less will be fail
}


#[inline]
/// Shared code for equality checks. Assumes arg0 - fail label, arg1,2 - values
fn shared_equality_opcode(
  _vm: &VM,
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
      ctx.ip = CodePtr::from_cp(fail_label)
    }
  } else if compare::cmp_terms(a, b, exact)? != desired_result {
    // Other than desired_recult will cause jump to 'fail'
    ctx.ip = CodePtr::from_cp(fail_label)
  }

  Ok(DispatchResult::Normal)
}
