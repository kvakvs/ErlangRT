use bif::result::{BifResult};
use emulator::gen_atoms;
use emulator::mfa::{MFArity};
use emulator::process::{Process};
use fail::{Hopefully};
use rt_defs::{ExceptionType, Arity};
use term::boxed;
use term::lterm::*;


pub fn ubif_self_0(cur_proc: &mut Process, _args: &[LTerm]) -> Hopefully<BifResult> {
  Ok(BifResult::Value(cur_proc.pid))
}


/// Create a function pointer from atom(), atom(), smallint()
pub fn bif_make_fun_3(cur_proc: &mut Process, args: &[LTerm]) -> Hopefully<BifResult> {
  if !args[0].is_atom() || !args[1].is_atom() || !args[2].is_small() {
    return Ok(BifResult::Exception(ExceptionType::Error, gen_atoms::BADARG));
  }

  let hp = &mut cur_proc.heap;
  let mfa = MFArity::new(args[0], args[1],
                         args[2].get_small_unsigned() as Arity);

  // Create an export on heap and return it
  let expt = unsafe { boxed::Export::place_into(hp, &mfa)? };
  Ok(BifResult::Value(expt))
}
