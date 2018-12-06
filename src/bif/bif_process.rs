use crate::emulator::gen_atoms;
use crate::emulator::mfa::MFArity;
use crate::emulator::process::Process;
use crate::fail::Error;
use crate::fail::RtResult;
use crate::rt_defs::{Arity, ExceptionType};
use crate::term::boxed;
use crate::term::lterm::*;


pub fn ubif_self_0(cur_proc: &mut Process, _args: &[LTerm]) -> RtResult<LTerm> {
  Ok(cur_proc.pid)
}


/// Create a function pointer from atom(), atom(), smallint()
pub fn bif_make_fun_3(cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  if !args[0].is_atom() || !args[1].is_atom() || !args[2].is_small() {
    return Err(Error::Exception(ExceptionType::Error, gen_atoms::BADARG));
  }

  let hp = &mut cur_proc.heap;
  let mfa = MFArity::new(args[0], args[1], args[2].get_small_unsigned() as Arity);

  // Create an export on heap and return it
  let expt = unsafe { boxed::Export::create_into(hp, &mfa)? };
  Ok(expt)
}
