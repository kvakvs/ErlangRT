use core::fmt;

use crate::emulator::{code::pointer::FarCodePointer, mfa::MFArity};


/// Result of Lambda Table loading prepared for use in the runtime.
#[derive(Debug)]
pub struct FunEntry {
  pub mfa: MFArity,
  //  code_pos: usize,
  //  index: usize,
  pub nfree: usize,
  //  ouniq: usize,
}


impl FunEntry {
  pub fn new(mfa: MFArity, nfree: usize) -> FunEntry {
    FunEntry { mfa, nfree }
  }
}


/// Defines where the export is pointing. Could be code pointer or a BIF and
/// is terminated by a tail call or a return opcode (i.e. safely callable).
#[derive(PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum CallableLocation {
  /// The MFA of the export wasn't resolved yet or became invalid.
  NeedUpdate,
  /// Points to Erlang code.
  // TODO: Version/hash/seq id for codeptr if code is reloaded?
  Code(FarCodePointer),
  //  /// Points to a BIF callable function.
  //  Bif(BifFn),
}


impl fmt::Debug for CallableLocation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CallableLocation()")
  }
}
