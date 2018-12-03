use emulator::process::Process;
use rt_defs::ExceptionType;
use term::lterm::LTerm;
use term::builders::make_badfun_n;
use fail::{Hopefully, Error};


#[allow(dead_code)]
fn module() -> &'static str { "bif_sys: " }


/// Create an error for a NIF not loaded/not implemented.
pub fn bif_nif_error_1(cur_proc: &mut Process,
                       args: &[LTerm]) -> Hopefully<LTerm>
{
  Err(Error::Exception(ExceptionType::Error,
                       make_badfun_n(args, &mut cur_proc.heap)?))
}


/// Create an error for a NIF not loaded/not implemented.
pub fn bif_nif_error_2(cur_proc: &mut Process,
                       args: &[LTerm]) -> Hopefully<LTerm>
{
  Err(Error::Exception(ExceptionType::Error,
                       make_badfun_n(args, &mut cur_proc.heap)?))
}
