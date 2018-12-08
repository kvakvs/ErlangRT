//!
//! Implement Fun/Arity pair, printing, ordering etc
//!
use crate::rt_defs::Arity;
use crate::term::lterm::*;

use std::cmp::Ordering;
use core::fmt;


/// Reference to an internal function in some module.
#[derive(Debug, Eq, Clone)]
pub struct FunArity {
  pub f: LTerm,
  pub arity: Arity,
}


impl FunArity {
  //  /// Create an uninitialized function pointer for deferred initialization.
  //  pub fn new_uninit() -> FunArity {
  //    FunArity {
  //      f: LTerm::non_value(),
  //      arity: 0,
  //    }
  //  }

  /// Create from a function name and arity.
  pub fn new(f: LTerm, arity: Arity) -> FunArity {
    FunArity { f, arity }
  }
}


impl Ord for FunArity {
  fn cmp(&self, other: &FunArity) -> Ordering {
    let fa = (self.f, self.arity);
    fa.cmp(&(other.f, other.arity))
  }
}


impl PartialOrd for FunArity {
  fn partial_cmp(&self, other: &FunArity) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}


impl PartialEq for FunArity {
  fn eq(&self, other: &FunArity) -> bool {
    self.f == other.f && self.arity == other.arity
  }
}


// Printing funarities as "{}"
impl fmt::Display for FunArity {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}/{}", self.f, self.arity)
  }
}
