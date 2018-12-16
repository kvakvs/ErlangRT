use crate::{
  defs::ExceptionType,
  emulator::{gen_atoms, mfa::MFArity, process::Process},
  fail::{Error, RtResult},
  term::{boxed, lterm::*},
};

pub fn ubif_self_0(cur_proc: &mut Process, _args: &[LTerm]) -> RtResult<LTerm> {
  Ok(cur_proc.pid)
}

/// Create a function pointer from atom(), atom(), smallint()
pub fn bif_make_fun_3(cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  if !args[0].is_atom() || !args[1].is_atom() || !args[2].is_small() {
    return Err(Error::Exception(ExceptionType::Error, gen_atoms::BADARG));
  }

  let hp = &mut cur_proc.heap;
  let mfa = MFArity::from_slice(&args[0..3]);

  // Create an export on heap and return it
  let expt = unsafe { boxed::Export::create_into(hp, &mfa)? };
  Ok(expt)
}
