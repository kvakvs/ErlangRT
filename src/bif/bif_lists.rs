use crate::{
  bif::assert_arity,
  emulator::{process::Process, vm::VM},
  fail::{self, RtResult},
  term::{compare, lterm::*},
};
use core::cmp::Ordering;

// fn module() -> &'static str {
//  "bif_compare: "
//}

/// Calculate length of a list by traversing it.
pub fn gcbif_erlang_length_1(
  _vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:length", 1, args);

  let result = cons::list_length(args[0])?;
  Ok(LTerm::make_small_unsigned(result))
}

/// Calculate a new list made of two lists joined together.
/// Arg1 must be list or NIL.
pub fn bif_erlang_ebif_plusplus_2_2(
  _vm: &mut VM,
  curr_p: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:++", 2, args);

  if !args[0].is_list() {
    return fail::create::badarg();
  }

  // Doing [] ++ X -> X
  if args[0] == LTerm::nil() {
    return Ok(args[1]);
  }

  // Copy the list args[0] without setting its tail, ...
  let hp = &mut curr_p.heap;
  let (l1, tail) = unsafe { cons::copy_list_leave_tail(args[0], hp) }?;

  // then append the tail
  unsafe {
    (*tail).set_tl(args[1]);
  }

  // Return what we got joined together
  Ok(l1)
}

/// Takes head of a cons value, otherwise returns badarg.
pub fn ubif_erlang_hd_1(
  _vm: &mut VM,
  _curr_p: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:hd", 1, args);
  if !args[0].is_cons() {
    return fail::create::badarg();
  }
  let p = args[0].get_cons_ptr();
  unsafe { Ok((*p).hd()) }
}

/// Takes tail of a cons value, otherwise returns badarg.
pub fn ubif_erlang_tl_1(
  _vm: &mut VM,
  _curr_p: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:hd", 1, args);
  if !args[0].is_cons() {
    return fail::create::badarg();
  }
  let p = args[0].get_cons_ptr();
  unsafe { Ok((*p).tl()) }
}

pub fn bif_lists_member_2(
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
