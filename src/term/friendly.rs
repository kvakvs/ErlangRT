//!
//! Representing Erlang terms as a complex Rust enum, more developer friendly,
//! possibly there's an unknown performance/memory cost, we don't care yet.
//!
use types;
use types::{Word, SWord};

use std;
use num::bigint::BigInt;

#[derive(Debug, PartialEq)]
pub enum Term {
  Atom(Word),
  SmallInt(types::SWord),
  BigInt(Box<BigInt>),
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

impl Term {
  /// Given a word, determine if it fits into Smallint (word size - 4 bits)
  /// otherwise form a BigInt
  pub fn from_word(w: Word) -> Term {
    if w < types::platf_bits() - 4 - 1 { // 4 bits for imm1 tag and 1 for sign
      Term::SmallInt(w as SWord)
    }
    Term::BigInt(BigInt::from::<usize>(w))
  }
}

