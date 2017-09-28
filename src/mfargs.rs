use term::Term;

type Arity = u16;

// MFArgs or MFArity should be able to give us mod and fun whenever
pub trait IMFArity {
  fn get_mod(&self) -> Term;
  fn get_fun(&self) -> Term;
  fn get_arity(&self) -> Arity;
}

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