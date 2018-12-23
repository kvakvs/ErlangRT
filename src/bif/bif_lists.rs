use crate::{
  emulator::{process::Process, vm::VM},
  fail::RtResult,
  term::lterm::*,
};

fn module() -> &'static str {
  "bif_compare: "
}

/// Calculate length of a list by traversing it.
pub fn gcbif_erlang_length_1(
  _vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_eq!(args.len(), 1, "{}gcbif_length_1 takes 1 arg", module());

  let result = cons::list_length(args[0])?;
  Ok(LTerm::make_small_unsigned(result))
}
