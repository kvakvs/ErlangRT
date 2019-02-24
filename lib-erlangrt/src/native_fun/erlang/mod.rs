use crate::{
  emulator::gen_atoms,
  native_fun::{
    erlang::{
      arithmetic::*, compare::*, list::*, predicate::*, process::*, sys::*,
      type_conversions::*,
    },
    fn_entry::NativeFnEntry,
    module::NativeModule,
  },
};

pub mod arithmetic;
pub mod compare;
pub mod list;
pub mod predicate;
pub mod process;
pub mod sys;
pub mod type_conversions;

pub fn new() -> NativeModule {
  let mut m = NativeModule::new(gen_atoms::ERLANG);
  let fn_entries: Vec<NativeFnEntry> = vec![
    NativeFnEntry::with_str("*", 2, nativefun_multiply_2),
    NativeFnEntry::with_str("+", 2, nativefun_plus_2),
    NativeFnEntry::with_str("++", 2, NfErlangPlusPlus2::_f),
    NativeFnEntry::with_str("-", 2, nativefun_minus_2),
    NativeFnEntry::with_str("/=", 2, nativefun_notequal_2),
    NativeFnEntry::with_str("<", 2, nativefun_lessthan_2),
    NativeFnEntry::with_str("=/=", 2, nativefun_notequal_exact_2),
    NativeFnEntry::with_str("=:=", 2, nativefun_equal_exact_2),
    NativeFnEntry::with_str("=<", 2, nativefun_lessequal_2),
    NativeFnEntry::with_str("==", 2, nativefun_equalequal_2),
    NativeFnEntry::with_str(">", 2, nativefun_greaterthan_2),
    NativeFnEntry::with_str(">=", 2, nativefun_greaterequal_2),
    NativeFnEntry::with_str("atom_to_list", 1, nativefun_atom_to_list_1),
    NativeFnEntry::with_str("error", 1, nativefun_error_1),
    NativeFnEntry::with_str("error", 2, NfErlangError2::_f),
    NativeFnEntry::with_str("hd", 1, NfErlangHd1::_f),
    NativeFnEntry::with_str("integer_to_list", 1, NfErlangInt2List2::_f),
    NativeFnEntry::with_str("is_boolean", 1, nativefun_is_boolean_1),
    NativeFnEntry::with_str("is_process_alive", 1, NfErlangIsPAlive1::_f),
    NativeFnEntry::with_str("length", 1, NfErlangLength1::_f),
    NativeFnEntry::with_str("list_to_binary", 1, NfErlangL2b1::_f),
    NativeFnEntry::with_str("load_nif", 2, nativefun_load_nif_2),
    NativeFnEntry::with_str("make_fun", 3, nativefun_make_fun_3),
    NativeFnEntry::with_str("nif_error", 1, NfErlangNifError1::_f),
    NativeFnEntry::with_str("nif_error", 2, NfErlangNifError2::_f),
    NativeFnEntry::with_str("process_flag", 2, NfErlangProcFlag2::_f),
    NativeFnEntry::with_str("process_flag", 3, NfErlangProcFlag3::_f),
    NativeFnEntry::with_str("register", 2, NfErlangRegister2::_f),
    NativeFnEntry::with_str("registered", 0, NfErlangRegistered0::_f),
    NativeFnEntry::with_str("self", 0, NfErlangSelf0::_f),
    NativeFnEntry::with_str("spawn", 3, NfErlangSpawn3::_f),
    NativeFnEntry::with_str("tl", 1, NfErlangTl1::_f),
  ];
  m.init_with(fn_entries.iter());
  m
}
