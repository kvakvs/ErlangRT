use crate::{
  defs::exc_type::ExceptionType,
  emulator::{
    gen_atoms,
    mfa::{MFArity, ModFunArgs},
    process::Process,
    process_flags,
    spawn_options::SpawnOptions,
    vm::VM,
  },
  fail::{self, Error, RtResult},
  native_fun::assert_arity,
  term::{boxed, lterm::*},
};

#[allow(dead_code)]
fn module() -> &'static str {
  "native funs module for erlang[process]: "
}

define_nativefun!(_vm, proc, _args,
  name: "erlang:self/0", struct_name: NfErlangSelf0, arity: 0,
  invoke: { Ok(proc.pid) },
  args:
);

/// Create a function pointer from atom(), atom(), smallint()
pub fn nativefun_make_fun_3(
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
define_nativefun!(vm, _proc, _args,
  name: "erlang:spawn/3", struct_name: NfErlangSpawn3, arity: 3,
  invoke: {
    let mfargs = ModFunArgs::with_args_list(m, f, args);
    let spawn_opts = SpawnOptions::default();
    vm.create_process(LTerm::nil(), &mfargs, &spawn_opts)
  },
  args: atom(m), atom(f), list(args)
);

define_nativefun!(vm, _proc, args,
  name: "erlang:is_process_alive/1", struct_name: NfErlangIsPAlive1, arity: 1,
  invoke: { Ok(LTerm::make_bool(vm.processes.lookup_pid(pid).is_some())) },
  args: pid(pid)
);

/// erlang:register(RegName :: atom(), Pid_or_Port)
define_nativefun!(vm, _proc, _args,
  name: "erlang:register/2", struct_name: NfErlangRegister2, arity: 2,
  invoke: {
    register_2(vm, name, pid_or_port)
  },
  args: atom(name), pid_port(pid_or_port)
);

pub fn register_2(vm: &mut VM, name: LTerm, pid_or_port: LTerm) -> RtResult<LTerm> {
  if name == gen_atoms::UNDEFINED || vm.processes.find_registered(name).is_some() {
    return fail::create::badarg();
  }
  vm.processes.register_name(name, pid_or_port);
  Ok(gen_atoms::TRUE)
}

define_nativefun!(_vm, _proc, _args,
  name: "erlang:registered/0", struct_name: NfErlangRegistered0, arity: 0,
  invoke: { panic!("not implemented") },
  args:
);

pub fn nativefun_process_flag_2(
  _vm: &mut VM,
  cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erlang:process_flag/2", 2, args);
  do_erlang_process_flag(cur_proc, args[0], args[1])
}

/// Set a supported process flag.
pub fn nativefun_process_flag_3(
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

#[inline]
fn do_erlang_process_flag(p: &mut Process, flag: LTerm, value: LTerm) -> RtResult<LTerm> {
  match flag {
    gen_atoms::TRAP_EXIT => {
      if !value.is_bool() {
        return fail::create::badarg();
      }
      Ok(LTerm::make_bool(p.process_flags.read_and_set(
        process_flags::TRAP_EXIT,
        value == gen_atoms::TRUE,
      )))
    }
    _ => fail::create::badarg_val(flag, &mut p.heap),
  }
}
