use bif::BifResult;
use emulator::gen_atoms;
use emulator::process::Process;
use rt_defs::ExceptionType;
use term::lterm::LTerm;


fn module() -> &'static str { "bif_sys: " }


/// Create an error for a NIF not loaded/not implemented.
pub fn bif_nif_error_1(cur_proc: &mut Process,
                       args: &[LTerm]) -> BifResult
{
  BifResult::Exception(ExceptionType::Error, gen_atoms::BADFUN)
}


/// Create an error for a NIF not loaded/not implemented.
pub fn bif_nif_error_2(cur_proc: &mut Process,
                       args: &[LTerm]) -> BifResult
{
  BifResult::Exception(ExceptionType::Error, gen_atoms::BADFUN)
}
