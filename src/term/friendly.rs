//!
//! Representing Erlang terms as a complex Rust enum, more developer friendly,
//! possibly there's an unknown performance/memory cost, we don't care yet.
//!
use types;
type Word = types::Word;

#[derive(Debug, PartialEq)]
pub enum Term {
  Atom(Word),
  Int(types::Integral),
  Cons(Box<[Term]>),
  Nil,
  // an empty [] list
  Tuple { elements: Vec<Term> },
  Float(types::Float),
  // Internal values not visible in the user data
  XReg(Word),
  YReg(Word),
  FPReg(Word),
  Label(Word),
  // BEAM loader specials
  LiteralInt(Word), // a word value literally specified
  LiteralIndex(Word), // an index in literal heap
  AllocList,
}
