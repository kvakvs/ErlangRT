//!
//! Representing Erlang terms as a complex Rust enum, more developer friendly,
//! possibly there's an unknown performance/memory cost, we don't care yet.
//!
use defs;
use defs::{Word, SWord};

use std;
use num::bigint::BigInt;
use num::FromPrimitive;

#[derive(Debug, PartialEq)]
pub enum Term {
  /// Runtime atom index in the VM atom table
  Atom(Word),
  SmallInt(defs::SWord),
  BigInt(Box<BigInt>),
  /// A regular cons cell with a head and a tail
  Cons(Box<[Term]>),
  /// NIL [] zero sized list
  Nil,
  Tuple(Vec<Term>),
  /// zero sized tuple
  Tuple0,
  Float(defs::Float),

  //
  // Internal values not visible in the user data
  //

  /// A runtime index of X register
  X_(Word),
  /// A runtime index of a stack cell relative to the stack top (Y register)
  Y_(Word),
  /// A runtime index of a floating-point register
  FP_(Word),

  //
  // BEAM loader specials, these never occur at runtime and finding them
  // in runtime must be an error.
  //

  /// A load-time index of label
  Label_(Word),
  /// A load-time atom index in the loader atom table
  Atom_(Word),
  /// A load-time word value literally specified
  Int_(Word),
  /// A load-time index in literal heap
  Lit_(Word),
  AllocList_,
}

impl Term {
  /// Given a word, determine if it fits into Smallint (word size - 4 bits)
  /// otherwise form a BigInt
  pub fn from_word(w: Word) -> Term {
    if w < defs::MAX_POS_SMALL {
      return Term::SmallInt(w as SWord)
    }
    Term::BigInt(Box::new(BigInt::from_usize(w).unwrap()))
  }
}
