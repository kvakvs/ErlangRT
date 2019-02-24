use crate::{
  emulator::{atom, process::Process, vm::VM},
  fail::{self, RtResult},
  native_fun::assert_arity,
  term::lterm::{cons, LTerm},
};

pub fn nativefun_atom_to_list_1(
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
define_nativefun!(_vm, proc, args,
  name: "erlang:integer_to_list/1", struct_name: NfErlangInt2List2, arity: 1,
  invoke: { integer_to_list_1(proc, val) },
  args: term(val),
);

#[inline]
pub fn integer_to_list_1(curr_p: &mut Process, val: LTerm) -> RtResult<LTerm> {
  if !val.is_integer() {
    return fail::create::badarg();
  }
  unsafe { cons::integer_to_list(val, &mut curr_p.heap) }
}

/// Returns list `list` reversed with `tail` appended (any term).
define_nativefun!(_vm, proc, args,
  name: "erlang:list_to_binary/1", struct_name: NfErlangL2b1, arity: 1,
  invoke: { unsafe { list_to_binary_1(proc, list) } },
  args: list(list),
);

#[inline]
unsafe fn list_to_binary_1(_proc: &mut Process, list: LTerm) -> RtResult<LTerm> {
  Ok(list)
}
