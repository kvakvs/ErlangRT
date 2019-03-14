use crate::{
  emulator::{process::Process, vm::VM},
  fail::RtResult,
  term::value::Term,
};

pub mod fn_entry;
pub mod gen_native_fun; // generated
pub mod module;
pub mod registry;
#[macro_use]
pub mod macros;

// Native Modules (precompiled and preloaded)
//
pub mod erlang;
pub mod erts_internal;
pub mod lists;

/// A BIF function which runs under some process, takes some args (encoded in
/// its name and hardcoded in its code), and returns an `Term`.
/// In case of error the `NON_VALUE` should be returned and the process is
/// informed about error situation (error reason and type are set etc).
pub type NativeFn =
  fn(vm: &mut VM, cur_proc: &mut Process, args: &[Term]) -> RtResult<Term>;

#[inline]
pub fn assert_arity(fn_name: &str, have_arity: usize, args: &[Term]) {
  let have_args = args.len();
  debug_assert_eq!(
    have_arity, have_args,
    "{} arity is {}, called with {} args",
    fn_name, have_arity, have_args
  );
}
