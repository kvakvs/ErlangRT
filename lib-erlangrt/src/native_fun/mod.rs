use crate::{
  emulator::{process::Process, vm::VM},
  fail::{RtResult},
  term::lterm::LTerm,
};

pub mod gen_native_fun; // generated
pub mod registry;
pub mod module;
pub mod fn_entry;
#[macro_use]
pub mod macros;

// Native Modules (precompiled and preloaded)
//
pub mod erlang;

// Bif definitions grouped by topic
//
pub mod bif_arith;
pub mod bif_compare;
pub mod bif_erts_internal;
pub mod bif_lists;
pub mod bif_sys;
pub mod bif_type_conv;

pub use crate::native_fun::{
  bif_arith::*, bif_compare::*, bif_erts_internal::*, bif_lists::*,
  bif_sys::*, bif_type_conv::*,
};

/// A BIF function which runs under some process, takes some args (encoded in
/// its name and hardcoded in its code), and returns an `LTerm`.
/// In case of error the `NON_VALUE` should be returned and the process is
/// informed about error situation (error reason and type are set etc).
pub type NativeFn =
  fn(vm: &mut VM, cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm>;

#[inline]
pub fn assert_arity(fn_name: &str, have_arity: usize, args: &[LTerm]) {
  let have_args = args.len();
  debug_assert_eq!(
    have_arity, have_args,
    "{} arity is {}, called with {} args",
    fn_name, have_arity, have_args
  );
}
