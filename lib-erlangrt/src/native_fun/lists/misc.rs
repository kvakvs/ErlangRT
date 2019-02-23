use crate::{
  emulator::{process::Process, vm::VM},
  fail::{self, RtResult},
  term::{compare, lterm::*},
};
use core::cmp::Ordering;
use crate::term::boxed;
use crate::emulator::gen_atoms;

define_nativefun!(_vm, _proc, args,
  name: "lists:member/2", struct_name: NfListsMember2, arity: 2,
  invoke: { member_2(sample, list) },
  args: term(sample), list(list)
);

#[inline]
fn member_2(sample: LTerm, list: LTerm) -> RtResult<LTerm> {
  if list == LTerm::nil() {
    return Ok(LTerm::make_bool(false));
  }
  let result = cons::any(list, |elem| {
    if let Ok(cmp_result) = compare::cmp_terms(sample, elem, true) {
      cmp_result == Ordering::Equal
    } else {
      return false;
    }
  });
  return Ok(LTerm::make_bool(result));
}

define_nativefun!(_vm, _proc, args,
  name: "lists:keyfind/3", struct_name: NfListsKeyfind3, arity: 3,
  invoke: { keyfind_3(key, pos, list) },
  args: term(key), usize(pos), list(list)
);

#[inline]
fn keyfind_3(sample: LTerm, pos: usize, list: LTerm) -> RtResult<LTerm> {
  if let Some(first) = cons::find_first(list, |elem| {
    if !elem.is_tuple() {
      return false;
    }

    // TODO: Count reductions to pay for the complexity

    let tuple_p = elem.get_tuple_ptr();
    if unsafe { (*tuple_p).get_arity() } <= pos {
      return false;
    }
    let tuple_element = unsafe { boxed::Tuple::get_element_base0(tuple_p, pos) };
    if let Ok(cmp_result) = compare::cmp_terms(sample, tuple_element, true) {
      cmp_result == Ordering::Equal
    } else {
      return false;
    }
  }) {
    Ok(LTerm::make_boxed(first))
  } else {
    Ok(gen_atoms::FALSE)
  }
}
