use crate::{
  bif::assert_arity,
  emulator::{mfa::ModFunArgs, process::Process, spawn_options::SpawnOptions, vm::VM},
  fail::RtResult,
  term::lterm::LTerm,
};

#[allow(dead_code)]
fn module() -> &'static str {
  "bif_erts_internal: "
}

/// Spawns special system process
/// Usage: erts_internal:spawn_system_process(Mod, Fun, Args)
pub fn bif_erts_internal_spawn_system_process_3(
  vm: &mut VM,
  cur_proc: &mut Process,
  args: &[LTerm],
) -> RtResult<LTerm> {
  assert_arity("erts_internal:spawn_system_process", 3, args);
  let mfa = ModFunArgs::with_args_list(args[0], args[1], args[2]);
  let so = SpawnOptions::default();
  vm.spawn_system_process(cur_proc.pid, &mfa, so)
}
