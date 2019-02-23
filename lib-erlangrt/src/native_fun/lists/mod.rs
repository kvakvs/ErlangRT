pub mod misc;

use crate::{
  emulator::atom,
  native_fun::{lists::misc::*, fn_entry::NativeFnEntry, module::NativeModule},
};

pub fn new() -> NativeModule {
  let mut m = NativeModule::new(atom::from_str("lists"));
  let fn_entries: Vec<NativeFnEntry> = vec![
    NativeFnEntry::with_str("member", 2, nativefun_member_2),
  ];
  m.init_with(fn_entries.iter());
  m
}
