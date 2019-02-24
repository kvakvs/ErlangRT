use crate::{emulator::process::Process, fail::RtResult, term::lterm::*};

#[allow(dead_code)]
fn module() -> &'static str {
  "native funs module for erlang[list]: "
}

/// Calculate length of a list by traversing it.
define_nativefun!(_vm, _proc, args,
  name: "erlang:length/1", struct_name: NfErlangLength1, arity: 1,
  invoke: {
    let result = cons::list_length(list)?;
    Ok(LTerm::make_small_unsigned(result))
  },
  args: list(list),
);

/// Calculate a new list made of two lists joined together.
/// Arg1 must be list or NIL.
define_nativefun!(_vm, proc, args,
  name: "erlang:hd/1", struct_name: NfErlangPlusPlus2, arity: 2,
  invoke: { plusplus_2(proc, a, b) },
  args: list(a), term(b),
);

#[inline]
pub fn plusplus_2(curr_p: &mut Process, a: LTerm, b: LTerm) -> RtResult<LTerm> {
  // Doing [] ++ X -> X
  if a == LTerm::nil() {
    return Ok(b);
  }

  // Copy the list a without setting its tail, ...
  let hp = &mut curr_p.heap;
  let (l1, tail) = unsafe { cons::copy_list_leave_tail(a, hp) }?;

  // then append the tail
  unsafe {
    (*tail).set_tl(b);
  }

  // Return what we got joined together
  Ok(l1)
}

/// Takes head of a cons value, otherwise returns badarg.
define_nativefun!(_vm, _proc, args,
  name: "erlang:hd/1", struct_name: NfErlangHd1, arity: 1,
  invoke: {
    let p = list.get_cons_ptr();
    unsafe { Ok((*p).hd()) }
  },
  args: non_empty_list(list),
);

/// Takes tail of a cons value, otherwise returns badarg.
define_nativefun!(_vm, _proc, args,
  name: "erlang:tl/1", struct_name: NfErlangTl1, arity: 1,
  invoke: {
    let p = list.get_cons_ptr();
    unsafe { Ok((*p).tl()) }
  },
  args: non_empty_list(list),
);
