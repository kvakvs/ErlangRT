use crate::emulator::mfa::MFArity;

/// Result of Lambda Table loading prepared for use in the runtime.
#[derive(Debug)]
pub struct FunEntry {
  pub mfa: MFArity,
  //  code_pos: usize,
  //  index: usize,
  pub nfrozen: usize,
  //  ouniq: usize,
}

impl FunEntry {
  pub fn new(mfa: MFArity, nfrozen: usize) -> FunEntry {
    FunEntry { mfa, nfrozen }
  }
}

// impl fmt::Debug for CallableLocation {
//  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//    write!(f, "CallableLocation()")
//  }
//}
