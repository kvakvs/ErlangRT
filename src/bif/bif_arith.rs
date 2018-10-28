use bif::result::{BifResult};
use emulator::process::Process;
use term::boxed;
use term::lterm::*;

use num;


fn module() -> &'static str { "bif_arith: " }


/// Subtraction for 2 mixed terms. Algorithm comes from Erlang/OTP file
/// `erl_arith.c`, function `erts_mixed_minus`
pub fn ubif_sminus_2_2(cur_proc: &mut Process,
                       args: &[LTerm]) -> BifResult {
  assert_eq!(args.len(), 2, "{}ubif_sminus_2_2 takes 2 args", module());
  let a: LTerm = args[0];
  let b: LTerm = args[1];
  if a.is_small() {
    if b.is_small() {
      subtract_two_small(cur_proc, a, b)
    } else {
      panic!("{}subtract: b={} other than small notimpl", module(), b)
    }
  } else {
    panic!("{}subtract: a={} other than small notimpl", module(), a)
  }
}


/// Addition for 2 mixed terms.
pub fn ubif_splus_2_2(cur_proc: &mut Process,
                      args: &[LTerm]) -> BifResult {
  assert_eq!(args.len(), 2, "{}ubif_sminus_2_2 takes 2 args", module());
  let a: LTerm = args[0];
  let b: LTerm = args[1];
  if a.is_small() {
    if b.is_small() {
      add_two_small(cur_proc, a, b)
    } else {
      panic!("{}subtract: b={} other than small notimpl", module(), b)
    }
  } else {
    panic!("{}subtract: a={} other than small notimpl", module(), a)
  }
}


/// So the check above has concluded that `a` and `b` are both small integers.
/// Implement subtraction, possibly creating a big integer.
fn subtract_two_small(cur_proc: &mut Process, a: LTerm, b: LTerm) -> BifResult
{
  // Both a and b are small, we've got an easy time
  let iresult = a.get_small_signed() - b.get_small_signed();
  // Even better: the result is also a small
  if LTerm::small_fits(iresult) {
    return BifResult::Value(LTerm::make_small_signed(iresult))
  }
  create_bigint(cur_proc, iresult)
}


/// So the check above has concluded that `a` and `b` are both small integers.
/// Implement addition, possibly creating a big integer.
fn add_two_small(cur_proc: &mut Process, a: LTerm, b: LTerm) -> BifResult
{
  // Both a and b are small, we've got an easy time
  let iresult = a.get_small_signed() + b.get_small_signed();
  // Even better: the result is also a small
  if LTerm::small_fits(iresult) {
    return BifResult::Value(LTerm::make_small_signed(iresult))
  }
  create_bigint(cur_proc, iresult)
}


fn create_bigint(cur_proc: &mut Process, iresult: isize) -> BifResult {
  // We're out of luck - the result is not a small, but we have BigInt!
  let big = num::BigInt::from(iresult);
  // Place a new BigInt on heap
  // TODO: Make a tool function for this, also ext_term_format:decode_big
  let heap = &mut cur_proc.heap;
  let rbig_result = unsafe {
    boxed::Bignum::create_into(heap, big)
  };

  match rbig_result {
    Ok(rbig) => BifResult::Value(boxed::Bignum::make_term(rbig)),
    Err(f) => BifResult::Fail(f),
  }
}
