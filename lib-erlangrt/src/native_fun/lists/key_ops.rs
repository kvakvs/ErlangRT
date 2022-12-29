//! Implements Keyfind/Keysearch/... tuple-operations on a list.
use crate::{
  emulator::gen_atoms,
  fail::RtResult,
  term::{compare, cons, Term},
};
use std::cmp::Ordering;

define_nativefun!(_vm, _proc, args,
  name: "lists:keyfind/3", struct_name: NfListsKeyfind3, arity: 3,
  invoke: { keyfind_3(key, pos, list) },
  args: term(key), usize(pos), list(list),
);

#[inline]
fn keyfind_3(sample: Term, pos: usize, list: Term) -> RtResult<Term> {
  let predicate = |elem: Term| {
    if !elem.is_tuple() {
      return false;
    }

    // TODO: Count reductions to pay for the complexity

    let tuple_p = elem.get_tuple_ptr();
    if unsafe { (*tuple_p).get_arity() } <= pos {
      return false;
    }
    let tuple_element = unsafe { (*tuple_p).get_element(pos) };
    if let Ok(cmp_result) = compare::cmp_terms(sample, tuple_element, true) {
      cmp_result == Ordering::Equal
    } else {
      false
    }
  };

  if let Some(first) = cons::find_first(list, predicate) {
    Ok(Term::make_boxed(first))
  } else {
    Ok(gen_atoms::FALSE)
  }
}
