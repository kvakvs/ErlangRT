pub mod gen_bif; // generated
pub mod bif_process;

use defs::Word;
use emulator::mfa::MFArity;
use emulator::process::Process;
use fail::{Hopefully, Error};
use term::lterm::LTerm;


pub type BifFn = fn(cur_proc: *mut Process,
                    args: *const Word) -> LTerm;

pub use bif::bif_process::*;


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
