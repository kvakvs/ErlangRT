use crate::{
  emulator::{mfa::MFArity, process::Process, vm::VM},
  fail::{self, RtResult},
  term::lterm::LTerm,
};

pub mod gen_bif; // generated

// Bif definitions grouped by topic
//
pub mod bif_arith;
pub mod bif_compare;
pub mod bif_lists;
pub mod bif_process;
pub mod bif_sys;
pub mod bif_type_conv;

pub use crate::bif::{
  bif_arith::*, bif_compare::*, bif_lists::*, bif_process::*, bif_sys::*,
  bif_type_conv::*,
};

/// A BIF function which runs under some process, takes some args (encoded in
/// its name and hardcoded in its code), and returns an `LTerm`.
/// In case of error the `NON_VALUE` should be returned and the process is
/// informed about error situation (error reason and type are set etc).
pub type BifFn =
  fn(vm: &mut VM, cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm>;

pub fn is_bif(mfa: &MFArity) -> bool {
  // Naive implementation. TODO: Binary search or a hashmap
  for bt in gen_bif::BIF_TABLE {
    if bt.m == mfa.m && bt.f == mfa.f && bt.arity == mfa.arity {
      return true;
    }
  }
  false
}

pub fn find_bif(mfa: &MFArity) -> RtResult<BifFn> {
  // Naive implementation. TODO: Binary search or a hashmap
  for bt in gen_bif::BIF_TABLE {
    if bt.m == mfa.m && bt.f == mfa.f && bt.arity == mfa.arity {
      return Ok(bt.func);
    }
  }
  // TODO: This string formatting is not efficient at all
  Err(fail::Error::BifNotFound(format!("{}", mfa)))
}

#[inline]
pub fn assert_arity(fn_name: &str, have_arity: usize, args: &[LTerm]) {
  let have_args = args.len();
  debug_assert_eq!(
    have_arity, have_args,
    "{} arity is {}, called with {} args",
    fn_name, have_arity, have_args
  );
}
