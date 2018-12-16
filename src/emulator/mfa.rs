//! Module contains reference structs to external and internal functions.
//! M:F/Arity (external), M:F(Args) (apply style), F/Arity (internal).

use crate::{defs::Arity, emulator::funarity::FunArity, term::lterm::*};

use core::fmt;

/// Reference to an M:F(Args) function, ready to be called with arguments.
pub struct MFArgs {
  m: LTerm,
  f: LTerm,
  args: Vec<LTerm>,
}

impl MFArgs {
  pub fn new(m: LTerm, f: LTerm, args: Vec<LTerm>) -> MFArgs {
    MFArgs { m, f, args }
  }

  pub fn get_mfarity(&self) -> MFArity {
    MFArity {
      m: self.m,
      f: self.f,
      arity: self.args.len() as Arity,
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct MFArity {
  pub m: LTerm,
  pub f: LTerm,
  pub arity: Arity,
}

impl MFArity {
  pub fn new(m: LTerm, f: LTerm, arity: Arity) -> MFArity {
    MFArity { m, f, arity }
  }

  pub fn from_slice(lterms: &[LTerm]) -> MFArity {
    MFArity {
      m: lterms[0],
      f: lterms[1],
      arity: lterms[2].get_small_unsigned(),
    }
  }

  pub fn new_from_funarity(m: LTerm, fa: &FunArity) -> MFArity {
    MFArity {
      m,
      f: fa.f,
      arity: fa.arity,
    }
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
