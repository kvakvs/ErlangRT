use emulator::mfa::MFArity;
use emulator::process::{Process, ExceptionType};
use fail::{Hopefully, Error};
use term::lterm::*;

use std::fmt;


pub mod gen_bif; // generated
pub mod bif_arith;
pub mod bif_compare;
pub mod bif_process;

pub use bif::bif_arith::*;
pub use bif::bif_compare::*;
pub use bif::bif_process::*;


/// Returned by all BIFs to indicate an error, a value, or another condition.
#[allow(dead_code)]
pub enum BifResult {
  /// Totally legit result was returned.
  Value(LTerm),
  /// The bif has created an exception.
  Exception(ExceptionType, LTerm),
  /// Something has failed in the runtime.
  Fail(Error),
}


impl fmt::Display for BifResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &BifResult::Value(t) => write!(f, "Value({})", t),
      &BifResult::Exception(et, rsn) => write!(f, "Exc({:?}, {})", et, rsn),
      &BifResult::Fail(ref e) => write!(f, "{:?}", e),
    }
  }
}


/// A BIF function which runs under some process, takes some args (encoded in
/// its name and hardcoded in its code), and returns an `LTerm`.
/// In case of error the `NON_VALUE` should be returned and the process is
/// informed about error situation (error reason and type are set etc).
pub type BifFn = fn(cur_proc: &mut Process, args: &[LTerm]) -> BifResult;

pub fn is_bif(mfa: &MFArity) -> bool {
  // Naive implementation. TODO: Binary search or a hashmap
  for bt in gen_bif::BIF_TABLE {
    if bt.m == mfa.m && bt.f == mfa.f && bt.arity == mfa.arity {
      return true
    }
  }
  false
}


pub fn find_bif(mfa: &MFArity) -> Hopefully<BifFn> {
  // Naive implementation. TODO: Binary search or a hashmap
  for bt in gen_bif::BIF_TABLE {
    if bt.m == mfa.m && bt.f == mfa.f && bt.arity == mfa.arity {
      return Ok(bt.func)
    }
  }
  Err(Error::BifNotFound(format!("{}", mfa)))
}
