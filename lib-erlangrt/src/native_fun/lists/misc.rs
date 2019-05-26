//! Implements misc and general purpose list operations.
use crate::{
  emulator::{heap::THeapOwner, process::Process},
  fail::RtResult,
  term::{compare, term_builder::ListBuilder, value::*},
};
use core::cmp::Ordering;

define_nativefun!(_vm, _proc, args,
  name: "lists:member/2", struct_name: NfListsMember2, arity: 2,
  invoke: { member_2(sample, list) },
  args: term(sample), list(list),
);

#[inline]
fn member_2(sample: Term, list: Term) -> RtResult<Term> {
  if list == Term::nil() {
    return Ok(Term::make_bool(false));
  }
  let result = cons::any(list, |elem| {
    if let Ok(cmp_result) = compare::cmp_terms(sample, elem, true) {
      cmp_result == Ordering::Equal
    } else {
      return false;
    }
  });
  return Ok(Term::make_bool(result));
}

// Returns list `list` reversed with `tail` appended (any term).
define_nativefun!(_vm, proc, args,
  name: "lists:reverse/2", struct_name: NfListsReverse2, arity: 2,
  invoke: { unsafe { reverse_2(proc, list, tail) } },
  args: list(list), term(tail),
);

#[inline]
unsafe fn reverse_2(proc: &mut Process, list: Term, tail: Term) -> RtResult<Term> {
  let mut lb = ListBuilder::new()?;
  let hp = proc.get_heap_mut();

  // Going forward the list, prepend values to the result
  cons::for_each(list, |elem| lb.prepend(elem, hp))?;

  // Last element's tail in the new list is set to `tail` argument
  Ok(lb.make_term_with_tail(tail))
}
