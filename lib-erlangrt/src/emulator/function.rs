use crate::emulator::mfa::ModFunArity;

/// Result of Lambda Table loading prepared for use in the runtime.
#[derive(Debug)]
pub struct FunEntry {
  pub mfa: ModFunArity,
  //  code_pos: usize,
  //  index: usize,
  pub nfrozen: usize,
  //  ouniq: usize,
}

impl FunEntry {
  pub fn new(mfa: ModFunArity, nfrozen: usize) -> FunEntry {
    FunEntry { mfa, nfrozen }
  }
}

// impl fmt::Debug for CallableLocation {
//  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//    write!(f, "CallableLocation()")
//  }
//}
