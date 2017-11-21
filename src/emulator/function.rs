use rt_defs::Arity;
use term::lterm::LTerm;

/// Result of Lambda Table loading prepared for use in the runtime.
#[derive(Debug)]
pub struct FunEntry {
  pub fun: LTerm,
  pub arity: Arity,
  //  code_pos: u32,
  //  index: u32,
  pub nfree: u32,
  //  ouniq: u32,
}

impl FunEntry {
  pub fn new(fun: LTerm, arity: Arity, nfree: u32) -> FunEntry {
    FunEntry { fun, arity, nfree }
  }
}
