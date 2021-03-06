use crate::{
  emulator::{process::Process, vm::VM},
  fail::RtResult,
  native_fun::assert_arity,
  term::Term,
};

#[allow(dead_code)]
fn module() -> &'static str {
  "native funs module for erlang[predicate]: "
}

/// Return `true` if the value is a boolean (atom `true` or atom `false`)
pub fn nativefun_is_boolean_1(
  _vm: &mut VM,
  _curr_p: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_arity("erlang:is_boolean", 1, args);
  Ok(Term::make_bool(args[0].is_bool()))
}
