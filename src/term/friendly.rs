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
  Cons(Box<[Term]>), // a regular cons cell
  Nil, // NIL [] zero sized list
  Tuple(Vec<Term>),
  Tuple0, // zero sized tuple
  Float(types::Float),
  // Internal values not visible in the user data
  X_(Word),
  Y_(Word),
  FP_(Word),
  Label_(Word),
  // BEAM loader specials
  Int_(Word), // a word value literally specified
  Lit_(Word), // an index in literal heap
  AllocList_,
}
