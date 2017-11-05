//use defs::Word;
use emulator::mfa::MFArity;
use emulator::process::Process;
use fail::{Hopefully, Error};
use term::lterm::LTerm;


pub mod gen_bif; // generated
pub mod bif_process;
pub mod bif_arith;

pub use bif::bif_process::*;
pub use bif::bif_arith::*;


/// A BIF function which runs under some process, takes some args (encoded in
/// its name and hardcoded in its code), and returns an `LTerm`.
/// In case of error the `NON_VALUE` should be returned and the process is
/// informed about error situation (error reason and type are set etc).
pub type BifFn = fn(cur_proc: &mut Process, args: &[LTerm]) -> LTerm;

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
