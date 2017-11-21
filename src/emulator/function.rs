//use rt_defs::Arity;
use emulator::mfa::MFArity;
//use term::lterm::LTerm;


/// Result of Lambda Table loading prepared for use in the runtime.
#[derive(Debug)]
pub struct FunEntry {
  pub mfa: MFArity,
  //  code_pos: u32,
  //  index: u32,
  pub nfree: u32,
  //  ouniq: u32,
}

impl FunEntry {
  pub fn new(mfa: MFArity, nfree: u32) -> FunEntry {
    FunEntry { mfa, nfree }
  }
}
