//! Module contains reference structs to external and internal functions.
//! M:F/Arity (external), M:F(Args) (apply style), F/Arity (internal).

use term::lterm::LTerm;
use defs::Arity;
use emulator::funarity::FunArity;

use std::fmt;


/// Reference to an M:F(Args) function, ready to be called with arguments.
pub struct MFArgs {
  m: LTerm,
  f: LTerm,
  args: Vec<LTerm>
}


impl MFArgs {

  pub fn new(m: LTerm, f: LTerm, args: Vec<LTerm>) -> MFArgs {
    MFArgs{m, f, args}
  }

  pub fn get_mfarity(&self) -> MFArity {
    MFArity {
      m: self.m,
      f: self.f,
      arity: self.args.len() as Arity
    }
  }
}


pub struct MFArity {
  pub m: LTerm,
  pub f: LTerm,
  pub arity: Arity
}


impl MFArity {

  pub fn new(m: LTerm, f: LTerm, arity: Arity) -> MFArity {
    MFArity { m, f, arity }
  }


  pub fn get_funarity(&self) -> FunArity {
    FunArity::new(self.f, self.arity)
  }

}

impl fmt::Display for MFArity {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}:{}/{}", self.m, self.f, self.arity)
  }
}