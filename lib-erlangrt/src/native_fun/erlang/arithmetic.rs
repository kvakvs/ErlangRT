use crate::{
  big,
  emulator::{arith::multiplication, heap::THeapOwner, process::Process, vm::VM},
  fail::RtResult,
  term::*,
};

fn module() -> &'static str {
  "native funs module for erlang[arith]: "
}

/// Subtraction for 2 mixed terms. Algorithm comes from Erlang/OTP file
/// `erl_arith.c`, function `erts_mixed_minus`
pub fn nativefun_minus_2(
  _vm: &mut VM,
  cur_proc: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_eq!(args.len(), 2, "{}'-'/2 takes 2 args", module());
  let a: Term = args[0];
  let b: Term = args[1];
  if a.is_small() {
    if b.is_small() {
      subtract_two_small(cur_proc, a, b)
    } else {
      // TODO: See Erlang OTP erl_arith.c function erts_mixed_minus
      unimplemented!("{}subtract: b={} other than small", module(), b)
    }
  } else {
    unimplemented!("{}subtract: a={} other than small", module(), a)
  }
}

/// Addition for 2 mixed terms.
pub fn nativefun_plus_2(
  _vm: &mut VM,
  cur_proc: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_eq!(args.len(), 2, "{}ubif_sminus_2_2 takes 2 args", module());
  let a: Term = args[0];
  let b: Term = args[1];
  if a.is_small() {
    if b.is_small() {
      add_two_small(cur_proc, a, b)
    } else {
      // TODO: See Erlang OTP erl_arith.c function erts_mixed_plus
      unimplemented!("{}subtract: b={} other than small", module(), b)
    }
  } else {
    unimplemented!("{}subtract: a={} other than small", module(), a)
  }
}

/// So the check above has concluded that `a` and `b` are both small integers.
/// Implement subtraction, possibly creating a big integer.
fn subtract_two_small(cur_proc: &mut Process, a: Term, b: Term) -> RtResult<Term> {
  // Both a and b are small, we've got an easy time
  let iresult = a.get_small_signed() - b.get_small_signed();
  // Even better: the result is also a small
  if Term::small_fits(iresult) {
    return Ok(Term::make_small_signed(iresult));
  }

  create_bigint(cur_proc, iresult)
}

/// So the check above has concluded that `a` and `b` are both small integers.
/// Implement addition, possibly creating a big integer.
fn add_two_small(cur_proc: &mut Process, a: Term, b: Term) -> RtResult<Term> {
  // Both a and b are small, we've got an easy time.
  // The overflow in addition of two smalls will always fit Rust integer because
  // small use less bits than a Rust integer.
  let iresult = a.get_small_signed() + b.get_small_signed();
  // Even better: the result is also a small
  if Term::small_fits(iresult) {
    return Ok(Term::make_small_signed(iresult));
  }
  create_bigint(cur_proc, iresult)
}

/// Multiplication for 2 mixed terms.
pub fn nativefun_multiply_2(
  _vm: &mut VM,
  cur_proc: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_eq!(args.len(), 2, "{}ubif_stimes_2_2 takes 2 args", module());
  let a: Term = args[0];
  let b: Term = args[1];
  return multiplication::multiply(cur_proc.get_heap_mut(), a, b);
}

// TODO: shorten, use only heap of the process, inline, move to a lib module in arith
fn create_bigint(cur_proc: &mut Process, iresult: isize) -> RtResult<Term> {
  // We're out of luck - the result is not a small, but we have BigInt!
  let p = big::from_isize(cur_proc.get_heap_mut(), iresult)?;
  Ok(p)
}
