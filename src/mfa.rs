//! Module contains reference structs to external and internal functions.
//! M:F/Arity (external), M:F(Args) (apply style), F/Arity (internal).
use term::Term;

pub type Arity = u32;

/// MFArgs or MFArity should be able to give us mod and fun whenever, so
/// this trait is there to allow it.
pub trait IMFArity {
  fn get_mod(&self) -> Term;
  fn get_fun(&self) -> Term;
  fn get_arity(&self) -> Arity;
}

/// Reference to an internal function in some module.
pub struct FunArity {
  f: Term,
  arity: Arity,
}

/// Reference to an M:F(Args) function, ready to be called with arguments.
pub struct MFArgs {
  m: Term,
  f: Term,
  args: Vec<Term>
}
//
//pub struct MFArity {
//  m: Term,
//  f: Term,
//  arity: Arity
//}

impl IMFArity for MFArgs {
  fn get_mod(&self) -> Term { self.m }
  fn get_fun(&self) -> Term { self.f }
  fn get_arity(&self) -> Arity {
    assert!(self.args.len() < Arity::max_value() as usize);
    self.args.len() as Arity
  }
}

impl MFArgs {
  pub fn new(m: Term, f: Term, args: Vec<Term>) -> MFArgs {
    MFArgs{m, f, args}
  }
}

//impl MFArity {
//  pub fn new(m: Term, f: Term, arity: Arity) -> MFArity {
//    MFArity{m, f, arity}
//  }
//}