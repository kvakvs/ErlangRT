use core::cmp::Ordering;

use colored::Colorize;

use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context},
  fail::RtResult,
  term::{compare, lterm::Term},
};

// Checks exact equality between arg1 and arg2, on false jump to arg0
// Structure: is_eq_exact(on_false:CP, a:src, b:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsEqExact, arity: 3,
  run: {
    // exact comparison
    generic_comparison(ctx, fail, a, b,
      CmpPrecision::Exact, Ordering::Equal, CmpInvert::Just)
  },
  args: cp_or_nil(fail), load(a), load(b),
);

// Checks exact inequality between arg1 and arg2, on false jump to arg0
// Structure: is_ne_exact(on_false:CP, a:src, b:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsNeExact, arity: 3,
  run: {
    // exact comparison, Not
    generic_comparison(ctx, fail, a, b,
      CmpPrecision::Exact, Ordering::Equal, CmpInvert::Not)
  },
  args: cp_or_nil(fail), load(a), load(b),
);

// Checks relation, that arg1 IS LESS than arg2, jump to arg0 otherwise.
// Structure: is_lt(on_false:CP, a:src, b:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsLt, arity: 3,
  run: {
    // not exact comparison
    generic_comparison(ctx, fail, a, b,
      CmpPrecision::Relaxed, Ordering::Less, CmpInvert::Just)
  },
  args: cp_or_nil(fail), load(a), load(b),
);

// Checks relation, that arg1 IS EQUAL(soft) to arg2, jump to arg0 otherwise.
// Structure: is_eq(on_false:CP, a:src, b:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsEq, arity: 3,
  run: {
    // not exact comparison
    generic_comparison(ctx, fail, a, b,
      CmpPrecision::Relaxed, Ordering::Equal, CmpInvert::Just)
  },
  args: cp_or_nil(fail), load(a), load(b),
);

// Checks relation, that arg1 IS NO LESS than arg2, jump to arg0 otherwise.
// Structure: is_ge(on_false:CP, a:src, b:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsGe, arity: 3,
  run: {
    // not exact comparison
    // Not, other than less will be fail
    generic_comparison(ctx, fail, a, b,
      CmpPrecision::Relaxed, Ordering::Less, CmpInvert::Not)
  },
  args: cp_or_nil(fail), load(a), load(b),
);

#[derive(Eq, PartialEq, Debug)]
enum CmpPrecision {
  Relaxed,
  Exact,
}

#[derive(Eq, PartialEq, Debug)]
enum CmpInvert {
  Just,
  Not,
}

#[inline]
/// Shared code for equality checks. Assumes arg0 - fail label, arg1,2 - values
fn generic_comparison(
  ctx: &mut Context,
  fail_label: Term,
  a: Term,
  b: Term,
  exact: CmpPrecision,
  desired_result: Ordering,
  invert: CmpInvert,
) -> RtResult<DispatchResult> {
  let result = compare::cmp_terms(a, b, exact == CmpPrecision::Exact)?;
  // Not flag is xor-ed with result =/= desired
  if (result != desired_result) ^ (invert == CmpInvert::Not) {
    if cfg!(feature = "trace_comparisons") {
      println!(
        "Comparison {} {:?} {} ? {} => {:?} (desired {:?} {:?})",
        "failed".red(),
        exact,
        a,
        b,
        result,
        invert,
        desired_result
      );
    }
    ctx.jump(fail_label)
  }

  Ok(DispatchResult::Normal)
}
