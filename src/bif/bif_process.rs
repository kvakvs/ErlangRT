use crate::{
  defs::ExceptionType,
  emulator::{
    gen_atoms,
    mfa::{Args, MFASomething, MFArity},
    process::Process,
    scheduler::Prio,
    vm::VM,
  },
  fail::{Error, RtResult},
  term::{boxed, lterm::*},
};

pub fn ubif_erlang_self_0(
  _vm: &mut VM,
  cur_proc: &mut Process,
  _args: &[LTerm],
) -> RtResult<LTerm> {
  Ok(cur_proc.pid)
}

/// Create a function pointer from atom(), atom(), smallint()
pub fn bif_erlang_make_fun_3(
  _vm: &mut VM,
  cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  if !args[0].is_atom() || !args[1].is_atom() || !args[2].is_small() {
    return Err(Error::Exception(ExceptionType::Error, gen_atoms::BADARG));
  }

  let hp = &mut cur_proc.heap;
  let mfa = MFArity::from_slice(&args[0..3]);

  // Create an export on heap and return it
  let expt = unsafe { boxed::Export::create_into(hp, &mfa)? };
  Ok(expt)
}

/// Creates a new process specified by `module:function/arity` with `args`
/// (args are passed as list), `arity` is length of args list.
/// Spec: erlang:spawn(mod, fun, args:list)
pub fn bif_erlang_spawn_3(
  vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  let mfargs = MFASomething::new(args[0], args[1], Args::AsList(args[2]));
  let pid = vm.create_process(LTerm::nil(), &mfargs, Prio::Normal)?;

  Ok(pid)
}
