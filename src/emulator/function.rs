//use rt_defs::Arity;
//use term::lterm::LTerm;
use bif::BifFn;
use emulator::code::CodePtr;
use emulator::mfa::MFArity;


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


/// Defines where the export is pointing. Could be code pointer or a BIF.
pub enum MFADestination {
  /// The MFA of the export wasn't resolved yet or became invalid.
  NeedUpdate,
  /// Points to Erlang code.
  // TODO: Version/hash/seq id for codeptr if code is reloaded?
  Code(CodePtr),
  /// Points to a BIF callable function.
  Bif(BifFn),
}
