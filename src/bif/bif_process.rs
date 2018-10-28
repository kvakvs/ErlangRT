use bif::result::{BifResult};
use emulator::gen_atoms;
use emulator::process::Process;
use rt_defs::{ExceptionType, Arity};
use emulator::mfa::MFArity;
use term::lterm::*;
use term::boxed;


pub fn ubif_self_0(cur_proc: &mut Process, _args: &[LTerm]) -> BifResult {
  BifResult::Value(cur_proc.pid)
}


/// Create a function pointer from atom(), atom(), smallint()
pub fn bif_make_fun_3(cur_proc: &mut Process, args: &[LTerm]) -> BifResult {
  if !args[0].is_atom() || !args[1].is_atom() || !args[2].is_small() {
    return BifResult::Exception(ExceptionType::Error, gen_atoms::BADARG);
  }

  let hp = &mut cur_proc.heap;
  let mfa = MFArity::new(args[0], args[1],
                         args[2].get_small_unsigned() as Arity);

  // Create an export on heap and return it
  match unsafe { boxed::Export::place_into(hp, &mfa) } {
    Ok(expt) => BifResult::Value(expt),
    Err(e) => panic!(e),
  }
}
