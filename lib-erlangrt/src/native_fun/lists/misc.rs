use crate::{
  emulator::{process::Process, vm::VM},
  fail::{self, RtResult},
  native_fun::assert_arity,
  term::{compare, lterm::*},
};
use core::cmp::Ordering;

// fn module() -> &'static str {
//  "bif_compare: "
//}


pub fn nativefun_member_2(
  _vm: &mut VM,
  _curr_p: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("lists:member", 2, args);
  let list = args[1];
  if list == LTerm::nil() {
    return Ok(LTerm::make_bool(false));
  }
  if !list.is_cons() {
    return fail::create::badarg();
  }
  let sample = args[0];
  let result = cons::any(list, |elem| {
    let cmp_result = compare::cmp_terms(sample, elem, true);
    if cmp_result.is_err() {
      return false;
    }
    cmp_result.unwrap() == Ordering::Equal
  });
  return Ok(LTerm::make_bool(result));
}
