pub mod process;

use crate::{
  emulator::gen_atoms,
  native_fun::{erlang::process::*, fn_entry::NativeFnEntry, module::NativeModule},
};

pub fn new() -> NativeModule {
  let mut m = NativeModule::new(gen_atoms::ERLANG);
  let fn_entries: Vec<NativeFnEntry> = vec![
    //    NativeFnEntry::with_str("*", 2, ubif_erlang_multiply_2),
    //    NativeFnEntry::with_str("+", 2, ubif_erlang_plus_2),
    //    NativeFnEntry::with_str("++", 2, bif_erlang_plusplus_2),
    //    NativeFnEntry::with_str("-", 2, ubif_erlang_minus_2),
    //    NativeFnEntry::with_str("/=", 2, ubif_erlang_notequal_2),
    //    NativeFnEntry::with_str("<", 2, ubif_erlang_lessthan_2),
    //    NativeFnEntry::with_str("=/=", 2, ubif_erlang_notequal_exact_2),
    //    NativeFnEntry::with_str("=:=", 2, ubif_erlang_equal_exact_2),
    //    NativeFnEntry::with_str("=<", 2, ubif_erlang_lessequal_2),
    //    NativeFnEntry::with_str("==", 2, ubif_erlang_equalequal_2),
    //    NativeFnEntry::with_str(">", 2, ubif_erlang_greaterthan_2),
    //    NativeFnEntry::with_str(">=", 2, ubif_erlang_greaterequal_2),
    //    NativeFnEntry::with_str("atom_to_list", 1, bif_erlang_atom_to_list_1),
    //    NativeFnEntry::with_str("error", 1, bif_erlang_error_1),
    //    NativeFnEntry::with_str("error", 2, bif_erlang_error_2),
    //    NativeFnEntry::with_str("hd", 1, ubif_erlang_hd_1),
    //    NativeFnEntry::with_str("integer_to_list", 1, bif_erlang_integer_to_list_1),
    //    NativeFnEntry::with_str("is_boolean", 1, ubif_erlang_is_boolean_1),
    NativeFnEntry::with_str("is_process_alive", 1, native_is_process_alive_1),
    //    NativeFnEntry::with_str("length", 1, gcbif_erlang_length_1),
    //    NativeFnEntry::with_str("load_nif", 2, bif_erlang_load_nif_2),
    NativeFnEntry::with_str("make_fun", 3, nativefun_make_fun_3),
    //    NativeFnEntry::with_str("nif_error", 1, bif_erlang_nif_error_1),
    //    NativeFnEntry::with_str("nif_error", 2, bif_erlang_nif_error_2),
    NativeFnEntry::with_str("process_flag", 2, nativefun_process_flag_2),
    NativeFnEntry::with_str("process_flag", 3, nativefun_process_flag_3),
    NativeFnEntry::with_str("register", 2, nativefun_register_2),
    NativeFnEntry::with_str("registered", 0, nativefun_registered_0),
    NativeFnEntry::with_str("self", 0, nativefun_self_0),
    NativeFnEntry::with_str("spawn", 3, nativefun_spawn_3),
    //    NativeFnEntry::with_str("tl", 1, ubif_erlang_tl_1),
  ];
  m.init_with(fn_entries.iter());
  m
}
