use crate::{
  bif::assert_arity,
  defs::exc_type::ExceptionType,
  emulator::{
    gen_atoms,
    heap::Heap,
    mfa::{Args, MFASomething, MFArity},
    process::{self, Process},
    scheduler::Prio,
    vm::VM,
  },
  fail::{self, Error, RtResult},
  term::{boxed, lterm::*},
};

pub fn ubif_erlang_self_0(
  _vm: &mut VM,
  cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:self", 0, args);
  Ok(cur_proc.pid)
}

/// Create a function pointer from atom(), atom(), smallint()
pub fn bif_erlang_make_fun_3(
  _vm: &mut VM,
  cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:make_fun", 3, args);
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
  assert_arity("erlang:spawn", 3, args);
  let mfargs = MFASomething::new(args[0], args[1], Args::AsList(args[2]));
  let pid = vm.create_process(LTerm::nil(), &mfargs, Prio::Normal)?;

  Ok(pid)
}

pub fn bif_erlang_is_process_alive_1(
  vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:is_process_alive", 1, args);
  let result = vm.processes.lookup_pid(args[0]).is_some();
  Ok(LTerm::make_bool(result))
}

/// erlang:register(RegName :: atom(), Pid_or_Port)
pub fn bif_erlang_register_2(
  vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:register/2", 2, args);
  if !args[0].is_atom()
    || args[0] == gen_atoms::UNDEFINED
    || !(args[1].is_pid() || args[1].is_port())
    || vm.processes.find_registered(args[0]).is_some()
  {
    return fail::create::badarg();
  }
  vm.processes.register_name(args[0], args[1]);
  Ok(gen_atoms::TRUE)
}

pub fn bif_erlang_registered_0(
  vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:registered/0", 0, args);
  panic!("not implemented")
}

pub fn bif_erlang_process_flag_2(
  vm: &mut VM,
  cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:process_flag/2", 2, args);
  do_erlang_process_flag(cur_proc, args[0], args[1])
}

pub fn bif_erlang_process_flag_3(
  vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:process_flag/3", 3, args);
  let proc_p = vm.processes.unsafe_lookup_pid_mut(args[0]);
  if proc_p.is_null() {
    return fail::create::badarg();
  }
  let p = unsafe { &mut (*proc_p) };
  do_erlang_process_flag(p, args[1], args[2])
}

fn do_erlang_process_flag(
  p: &mut Process,
  flag: LTerm,
  value: LTerm,
) -> RtResult<LTerm> {
  match flag {
    gen_atoms::TRAP_EXIT => {
      if !value.is_bool() {
        return fail::create::badarg();
      }
      Ok(LTerm::make_bool(p.set_process_flag(
        process::TRAP_EXIT,
        value == gen_atoms::TRUE,
      )))
    }
    _ => fail::create::badarg_val(flag, &mut p.heap),
  }
}
