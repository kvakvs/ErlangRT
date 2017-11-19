use std::cmp::Ordering;

use beam::gen_op;
use beam::opcodes::assert_arity;
use rt_defs::{DispatchResult};
use emulator::code::CodePtr;
use emulator::process::Process;
use emulator::runtime_ctx::Context;
use term::compare;
use term::lterm::aspect_list::ListAspect;


#[inline]
/// Shared code for equality checks. Assumes arg0 - fail label, arg1,2 - values
fn shared_equality_opcode(ctx: &mut Context,
                          curr_p: &mut Process,
                          exact: bool,
                          desired_result: Ordering) -> DispatchResult {
  let hp = &curr_p.heap;
  let fail_label = ctx.fetch_term();
  let a = ctx.fetch_and_load(hp);
  let b = ctx.fetch_and_load(hp);

  assert!(false == fail_label.is_nil());
  if compare::cmp_terms(a, b, exact) != desired_result {
    ctx.ip = CodePtr::from_cp(fail_label)
  }

  DispatchResult::Normal
}


/// Checks exact equality between arg1 and arg2, on false jump to arg0
#[inline]
pub fn opcode_is_eq_exact(ctx: &mut Context,
                          curr_p: &mut Process) -> DispatchResult {
  // Structure: is_eq_exact(on_false:CP, a:src, b:src)
  assert_arity(gen_op::OPCODE_IS_EQ_EXACT, 3);
  shared_equality_opcode(ctx, curr_p, true, Ordering::Equal)
}


/// Checks relation, that arg1 IS LESS than arg2, jump to arg0 otherwise.
#[inline]
pub fn opcode_is_lt(ctx: &mut Context,
                    curr_p: &mut Process) -> DispatchResult {
  // Structure: is_lt(on_false:CP, a:src, b:src)
  assert_arity(gen_op::OPCODE_IS_LT, 3);
  shared_equality_opcode(ctx, curr_p, true, Ordering::Less)
}



/// Checks relation, that arg1 IS LESS than arg2, jump to arg0 otherwise.
#[inline]
pub fn opcode_is_eq(ctx: &mut Context,
                    curr_p: &mut Process) -> DispatchResult {
  // Structure: is_eq(on_false:CP, a:src, b:src)
  assert_arity(gen_op::OPCODE_IS_EQ, 3);
  shared_equality_opcode(ctx, curr_p, false, Ordering::Equal)
}
