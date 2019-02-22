use crate::{
  native_fun::assert_arity,
  emulator::{atom, process::Process, vm::VM},
  fail::{self, RtResult},
  term::lterm::{cons, LTerm},
};

pub fn bif_erlang_atom_to_list_1(
  _vm: &mut VM,
  curr_p: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:atom_to_list", 1, args);
  if !args[0].is_atom() {
    return fail::create::badarg();
  }
  let atom_p = atom::lookup(args[0]);
  if atom_p.is_null() {
    return fail::create::badarg();
  }
  unsafe {
    let s = cons::rust_str_to_list(&(*atom_p).name, &mut curr_p.heap)?;
    Ok(s)
  }
}

/// Converts an integer to Erlang string (list of integers)
pub fn bif_erlang_integer_to_list_1(
  _vm: &mut VM,
  curr_p: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:integer_to_list", 1, args);
  let val = args[0];
  if !val.is_integer() {
    return fail::create::badarg();
  }
  unsafe { cons::integer_to_list(val, &mut curr_p.heap) }
}
