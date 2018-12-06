use crate::emulator::process::Process;
use crate::fail::{Error, RtResult};
use crate::rt_defs::ExceptionType;
use crate::term::builders::make_badfun_n;
use crate::term::lterm::LTerm;


#[allow(dead_code)]
fn module() -> &'static str {
  "bif_sys: "
}


/// Create an error for a NIF not loaded/not implemented.
pub fn bif_nif_error_1(cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  Err(Error::Exception(
    ExceptionType::Error,
    make_badfun_n(args, &mut cur_proc.heap)?,
  ))
}


/// Create an error for a NIF not loaded/not implemented.
pub fn bif_nif_error_2(cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  Err(Error::Exception(
    ExceptionType::Error,
    make_badfun_n(args, &mut cur_proc.heap)?,
  ))
}
