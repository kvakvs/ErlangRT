//! Module contains reference structs to external and internal functions.
//! M:F/Arity (external), M:F(Args) (apply style), F/Arity (internal).
//use term::friendly;
use term::lterm::LTerm;

use std::cmp::Ordering;

use defs::Arity;

/// MFArgs or MFArity should be able to give us mod and fun whenever, so
/// this trait is there to allow it.
pub trait IMFArity {
  fn get_mod(&self) -> LTerm;
  fn get_fun(&self) -> LTerm;
  fn get_arity(&self) -> Arity;
}

/// Reference to an M:F(Args) function, ready to be called with arguments.
pub struct MFArgs {
  m: LTerm,
  f: LTerm,
  args: Vec<LTerm>
}
//
//pub struct MFArity {
//  m: Term,
//  f: Term,
//  arity: Arity
//}

impl IMFArity for MFArgs {
  fn get_mod(&self) -> LTerm { self.m }
  fn get_fun(&self) -> LTerm { self.f }
  fn get_arity(&self) -> Arity {
    assert!(self.args.len() < Arity::max_value() as usize);
    self.args.len() as Arity
  }
}

impl MFArgs {
  pub fn new(m: LTerm, f: LTerm, args: Vec<LTerm>) -> MFArgs {
    MFArgs{m, f, args}
  }
}

//impl MFArity {
//  pub fn new(m: Term, f: Term, arity: Arity) -> MFArity {
//    MFArity{m, f, arity}
//  }
//}