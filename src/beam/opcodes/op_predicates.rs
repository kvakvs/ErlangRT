use std::cmp::Ordering;

use beam::gen_op;
use beam::opcodes::assert_arity;
use rt_defs::{DispatchResult};
use emulator::code::CodePtr;
use emulator::process::Process;
use emulator::runtime_ctx::Context;
use term::compare;


/// Checks exact equality between arg1 and arg2, on false jump to arg0
#[inline]
pub fn opcode_is_eq_exact(ctx: &mut Context,
                          curr_p: &mut Process) -> DispatchResult {
  // Structure: is_eq_exact(on_false:CP, a:src, b:src)
  assert_arity(gen_op::OPCODE_IS_EQ_EXACT, 3);

  let hp = &curr_p.heap;
  let not_equal = ctx.fetch_term();
  let a = ctx.fetch_and_load(hp);
  let b = ctx.fetch_and_load(hp);

  if compare::cmp_terms(a, b, true) != Ordering::Equal {
    ctx.ip = CodePtr::from_cp(not_equal)
  }

  DispatchResult::Normal
}


/// Checks relation, that arg1 IS LESS than arg2, jump to arg0 otherwise.
#[inline]
pub fn opcode_is_lt(ctx: &mut Context,
                    curr_p: &mut Process) -> DispatchResult {
  // Structure: is_lt(on_false:CP, a:src, b:src)
  assert_arity(gen_op::OPCODE_IS_LT, 3);

  let hp = &curr_p.heap;
  let not_less = ctx.fetch_term();
  let a = ctx.fetch_and_load(hp);
  let b = ctx.fetch_and_load(hp);

  if compare::cmp_terms(a, b, true) != Ordering::Less {
    ctx.ip = CodePtr::from_cp(not_less)
  }

  DispatchResult::Normal
}
