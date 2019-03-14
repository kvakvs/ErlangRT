use crate::{big, emulator::heap::Heap, fail::RtResult, term::lterm::*};

#[allow(dead_code)]
fn module() -> &'static str {
  "arith.multiplication: "
}

pub fn multiply(hp: &mut Heap, x: Term, y: Term) -> RtResult<Term> {
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

/// Implement multiplication for two signed integers, possibly creating a bigint.
pub fn multiply_two_small(hp: &mut Heap, x: isize, y: isize) -> RtResult<Term> {
  let big_x = big::from_isize(hp, x)?;
  let big_y = big::from_isize(hp, y)?;
  big::mul(hp, big_x, big_y)
}
