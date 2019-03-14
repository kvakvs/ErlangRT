use crate::{
  emulator::{atom, process::Process},
  fail::{self, RtResult},
  term::{
    value::{cons, Term},
    term_builder::BinaryBuilder,
  },
};

// Converts an atom to Erlang string.
define_nativefun!(_vm, proc, args,
  name: "erlang:atom_to_list/1", struct_name: NfErlangA2List2, arity: 1,
  invoke: { atom_to_list_1(proc, atom_val) },
  args: atom(atom_val),
);

#[inline]
pub fn atom_to_list_1(proc: &mut Process, atom_val: Term) -> RtResult<Term> {
  let atom_p = atom::lookup(atom_val);
  if atom_p.is_null() {
    return fail::create::badarg();
  }
  unsafe {
    let s = cons::rust_str_to_list(&(*atom_p).name, &mut proc.heap)?;
    Ok(s)
  }
}

// Converts an integer to Erlang string (list of integers)
define_nativefun!(_vm, proc, args,
  name: "erlang:integer_to_list/1", struct_name: NfErlangInt2List2, arity: 1,
  invoke: { integer_to_list_1(proc, val) },
  args: term(val),
);

#[inline]
pub fn integer_to_list_1(curr_p: &mut Process, val: Term) -> RtResult<Term> {
  if !val.is_integer() {
    return fail::create::badarg();
  }
  unsafe { cons::integer_to_list(val, &mut curr_p.heap) }
}

// Returns list `list` reversed with `tail` appended (any term).
define_nativefun!(_vm, proc, args,
  name: "erlang:list_to_binary/1", struct_name: NfErlangL2b1, arity: 1,
  invoke: { unsafe { list_to_binary_1(proc, list) } },
  args: list(list),
);

#[inline]
unsafe fn list_to_binary_1(proc: &mut Process, list: Term) -> RtResult<Term> {
  let size = cons::get_iolist_size(list);
  if size.bytes() == 0 {
    Ok(Term::empty_binary())
  } else {
    let mut bb = BinaryBuilder::with_size(size, &mut proc.heap)?;
    list_to_binary_1_recursive(&mut bb, list)?;
    Ok(bb.make_term())
  }
}

unsafe fn list_to_binary_1_recursive(
  bb: &mut BinaryBuilder,
  list: Term,
) -> RtResult<Term> {
  cons::for_each(list, |elem| {
    if elem.is_small() {
      // Any small integer even larger than 256 counts as 1 byte
      bb.write_byte(elem.get_small_unsigned() as u8);
    } else if elem.is_binary() {
      unimplemented!("l2b: appending a binary");
    // bb.write_binary(elem.get_box_ptr());
    } else if elem.is_cons() {
      list_to_binary_1_recursive(bb, elem)?;
    }
    Ok(())
  })?;
  Ok(list)
}
