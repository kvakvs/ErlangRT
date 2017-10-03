//! Module contains reference structs to external and internal functions.
//! M:F/Arity (external), M:F(Args) (apply style), F/Arity (internal).
//use term::friendly;
use term::low_level::LTerm;

use std::cmp::Ordering;

pub type Arity = u32;

/// MFArgs or MFArity should be able to give us mod and fun whenever, so
/// this trait is there to allow it.
pub trait IMFArity {
  fn get_mod(&self) -> LTerm;
  fn get_fun(&self) -> LTerm;
  fn get_arity(&self) -> Arity;
}

/// Reference to an internal function in some module.
#[derive(Eq)]
pub struct FunArity {
  pub f: LTerm,
  pub arity: Arity,
}

impl FunArity {
  pub fn new() -> FunArity {
    FunArity {
      f: LTerm::non_value(),
      arity: 0,
    }
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