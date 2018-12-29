use crate::{
  bif::assert_arity,
  defs::exc_type::ExceptionType,
  emulator::{process::Process, vm::VM},
  fail::{Error, RtResult},
  term::{builders::make_badfun_n, lterm::LTerm, term_builder::TupleBuilder},
};

#[allow(dead_code)]
fn module() -> &'static str {
  "bif_sys: "
}

/// Create an error for a NIF not loaded/not implemented.
pub fn bif_erlang_nif_error_1(
  _vm: &mut VM,
  cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:nif_error", 1, args);
  Err(Error::Exception(
    ExceptionType::Error,
    make_badfun_n(args, &mut cur_proc.heap)?,
  ))
}

/// Create an error for a NIF not loaded/not implemented.
pub fn bif_erlang_nif_error_2(
  _vm: &mut VM,
  cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:nif_error", 2, args);
  Err(Error::Exception(
    ExceptionType::Error,
    make_badfun_n(args, &mut cur_proc.heap)?,
  ))
}

/// Create an exception of type `error` with an argument.
pub fn bif_erlang_error_2(
  _vm: &mut VM,
  cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  let tb = TupleBuilder::with_arity(&mut cur_proc.heap, 2)?;
  unsafe {
    tb.set_element_base0(0, args[0]);
    tb.set_element_base0(1, args[1]);
  }
  Err(Error::Exception(ExceptionType::Error, tb.make_term()))
}

/// Create an exception of type `error`.
pub fn bif_erlang_error_1(
  _vm: &mut VM,
  _curr_p: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  Err(Error::Exception(ExceptionType::Error, args[0]))
}
