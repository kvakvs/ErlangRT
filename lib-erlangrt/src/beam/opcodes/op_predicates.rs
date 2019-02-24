use core::cmp::Ordering;

use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::{compare, lterm::LTerm},
};

/// Checks exact equality between arg1 and arg2, on false jump to arg0
/// Structure: is_eq_exact(on_false:CP, a:src, b:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsEqExact, arity: 3,
  run: {
    // exact comparison
    shared_equality(ctx, fail, a, b, true, Ordering::Equal, false)
  },
  args: cp_not_nil(fail), load(a), load(b),
);

/// Checks relation, that arg1 IS LESS than arg2, jump to arg0 otherwise.
/// Structure: is_lt(on_false:CP, a:src, b:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsLt, arity: 3,
  run: {
   // not exact comparison
   shared_equality(ctx, fail, a, b, false, Ordering::Less, false)
  },
  args: cp_not_nil(fail), load(a), load(b),
);

/// Checks relation, that arg1 IS EQUAL(soft) to arg2, jump to arg0 otherwise.
/// Structure: is_eq(on_false:CP, a:src, b:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsEq, arity: 3,
  run: {
   // not exact comparison
   shared_equality(ctx, fail, a, b, false, Ordering::Equal, false)
  },
  args: cp_not_nil(fail), load(a), load(b),
);

/// Checks relation, that arg1 IS NO LESS than arg2, jump to arg0 otherwise.
/// Structure: is_ge(on_false:CP, a:src, b:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsGe, arity: 3,
  run: {
    // not exact comparison
    // inverted, other than less will be fail
    shared_equality(ctx, fail, a, b, false, Ordering::Less, true)
  },
  args: cp_not_nil(fail), load(a), load(b),
);

#[inline]
/// Shared code for equality checks. Assumes arg0 - fail label, arg1,2 - values
fn shared_equality(
  ctx: &mut Context,
  fail_label: LTerm,
  a: LTerm,
  b: LTerm,
  exact: bool,
  desired_result: Ordering,
  invert: bool,
) -> RtResult<DispatchResult> {
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
