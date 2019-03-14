use crate::{
  emulator::{mfa::ModFunArgs, spawn_options::SpawnOptions},
  term::value::Term,
};

// Spawns special system process
// Usage: erts_internal:spawn_system_process(Mod, Fun, Args)
define_nativefun!(vm, proc, args,
  name: "erts_internal:spawn_system_process/3",
  struct_name: NfErtsiSpawnSysProc3, arity: 3,
  invoke: {
    let mfargs = ModFunArgs::with_args_list(m, f, a);
    let so = SpawnOptions::default();
    vm.spawn_system_process(proc.pid, &mfargs, so)
  },
  args: atom(m), atom(f), term(a),
);
