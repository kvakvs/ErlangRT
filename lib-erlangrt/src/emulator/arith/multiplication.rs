use crate::{big, emulator::heap::THeap, fail::RtResult, term::*};

#[allow(dead_code)]
fn module() -> &'static str {
  "arith.multiplication: "
}

pub fn multiply(hp: &mut dyn THeap, x: Term, y: Term) -> RtResult<Term> {
  if x.is_small() {
    if y.is_small() {
      // Both a and b are small, check how many bits will be there in result
      if x == Term::small_0() || y == Term::small_0() {
        return Ok(Term::small_0());
      } else if x == Term::small_1() {
        return Ok(y);
      } else if y == Term::small_1() {
        return Ok(x);
      }
      let result = multiply_two_small(hp, x.get_small_signed(), y.get_small_signed())?;
      return Ok(result);
    } else {
      // TODO: See Erlang OTP erl_arith.c function erts_mixed_times
      unimplemented!("{}b={} other than small", module(), y)
    }
  }
  unimplemented!("{}a={} other than small", module(), x)
}

/// Optimistic multiplication which will possibly fit the small int without
/// creating three big ints.
#[cfg(target_pointer_width = "64")]
#[inline]
fn optimistic_mul(a: isize, b: isize) -> Option<Term> {
  let maybe_small = a as i128 * b as i128;
  if Term::small_fits_i128(maybe_small) {
    return Some(Term::make_small_signed(maybe_small as isize));
  }
  None
}

#[cfg(target_pointer_width = "32")]
#[inline]
fn optimistic_mul(a: isize, b: isize) -> Option<Term> {
  let maybe_small = a as i64 * b as i64;
  if Term::small_fits_i64(maybe_small) {
    return Some(Term::make_small_signed(maybe_small as isize));
  }
  None
}

/// Implement multiplication for two signed integers, possibly creating a bigint.
pub fn multiply_two_small(hp: &mut dyn THeap, x: isize, y: isize) -> RtResult<Term> {
  if let Some(val) = optimistic_mul(x, y) {
    return Ok(val);
  }

  // Pessimistic case, the result did not fit, so proceed with 2 big ints
  // creating a third big int.
  let big_x = big::from_isize(hp, x)?;
  let big_y = big::from_isize(hp, y)?;
  big::mul(hp, big_x, big_y)
}
