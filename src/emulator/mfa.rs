//! Module contains reference structs to external and internal functions.
//! M:F/Arity (external), M:F(Args) (apply style), F/Arity (internal).

use crate::{defs::Arity, emulator::funarity::FunArity, term::lterm::*};
use core::fmt;

#[derive(Debug)]
pub enum Args {
  // list of args
  AsList(LTerm),
  /* pointer to args with size
   * AsSlice(*const LTerm, usize), */
}

/// Reference to an M:F(Args) function, ready to be called with arguments.
pub struct MFASomething {
  m: LTerm,
  f: LTerm,
  args: Args,
}

impl MFASomething {
  pub fn new(m: LTerm, f: LTerm, args: Args) -> MFASomething {
    MFASomething { m, f, args }
  }

  pub fn get_mfarity(&self) -> MFArity {
    MFArity {
      m: self.m,
      f: self.f,
      arity: self.get_arity(),
    }
  }

  fn get_arity(&self) -> usize {
    match self.args {
      Args::AsList(lst) => {
        if let Ok(len) = cons::list_length(lst) {
          return len;
        }
      }
    }
    panic!("Can't find length for {:?}", self.args)
  }

  pub fn for_each_arg<T>(&self, func: T) where T: FnMut(LTerm) {
    match self.args {
      Args::AsList(lst) => {
        cons::for_each(lst, func)
      }
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
