pub mod key_ops;
pub mod misc;

use crate::{
  emulator::atom,
  native_fun::{
    fn_entry::NativeFnEntry,
    lists::{key_ops::*, misc::*},
    module::NativeModule,
  },
};

pub fn new() -> NativeModule {
  let mut m = NativeModule::new(atom::from_str("lists"));
  let fn_entries: Vec<NativeFnEntry> = vec![
    NativeFnEntry::with_str("keyfind", 3, NfListsKeyfind3::_f),
    NativeFnEntry::with_str("member", 2, NfListsMember2::_f),
    NativeFnEntry::with_str("reverse", 2, NfListsReverse2::_f),
  ];
  m.init_with(fn_entries.iter());
  m
}
