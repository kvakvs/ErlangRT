use crate::defs::SWord;
use crate::emulator::heap::Heap;
use crate::fail::RtResult;
use crate::term::boxed::bignum::Bignum;
use crate::term::lterm::*;
use num::bigint::BigInt;
use num::cast::ToPrimitive;

fn module() -> &'static str {
  "arith.multiplication: "
}

pub fn multiply(hp: &mut Heap, x: LTerm, y: LTerm) -> RtResult<LTerm> {
  if x.is_small() {
    if y.is_small() {
      // Both a and b are small, check how many bits will be there in result
      if x == LTerm::small_0() || y == LTerm::small_0() {
        return Ok(LTerm::small_0());
      } else if x == LTerm::small_1() {
        return Ok(y);
      } else if y == LTerm::small_1() {
        return Ok(x);
      }
      let result = multiply_two_small(hp, x.get_small_signed(), y.get_small_signed())?;
      return Ok(result);
    } else {
      // TODO: See Erlang OTP erl_arith.c function erts_mixed_times
      panic!("{}b={} other than small notimpl", module(), y)
    }
  }
  panic!("{}a={} other than small notimpl", module(), x)
}

/// Implement multiplication for two signed integers, possibly creating a bigint.
pub fn multiply_two_small(hp: &mut Heap, x: SWord, y: SWord) -> RtResult<LTerm> {
  let big_x = BigInt::from(x);
  let big_y = BigInt::from(y);
  let result = big_x.checked_mul(&big_y).unwrap();

  // If the result fits into smallint, return it as smallint
  if let Some(i) = result.to_isize() {
    if LTerm::small_fits(i) {
      return Ok(LTerm::make_small_signed(i));
    }
  }

  let boxptr = unsafe { Bignum::create_into(hp, result) }?;
  Ok(LTerm::make_boxed(boxptr))
}
