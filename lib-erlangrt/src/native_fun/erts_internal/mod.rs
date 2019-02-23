pub mod misc;

use crate::{
  emulator::atom,
  native_fun::{erts_internal::misc::*, fn_entry::NativeFnEntry, module::NativeModule},
};

pub fn new() -> NativeModule {
  let mut m = NativeModule::new(atom::from_str("erts_internal"));
  let fn_entries: Vec<NativeFnEntry> = vec![NativeFnEntry::with_str(
    "spawn_system_process",
    3,
    NfErtsiSpawnSysProc3::_f,
  )];
  m.init_with(fn_entries.iter());
  m
}
