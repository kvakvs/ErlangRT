//!
//! Representing Erlang terms as a complex Rust enum, more developer friendly,
//! possibly there's an unknown performance/memory cost, we don't care yet.
//!
use defs;
use defs::{Word, SWord};
use term::low_level::LTerm;

use std;
use num::bigint::BigInt;
use num::FromPrimitive;

fn module() -> &'static str { "term::friendly: " }

/// A friendly Rust-enum representing Erlang term both runtime and load-time
/// values. Make sure to crash nicely when they mix.
#[derive(Debug, PartialEq)]
pub enum FTerm {
  /// Runtime atom index in the VM atom table
  Atom(Word),
  SmallInt(defs::SWord),
  BigInt(Box<BigInt>),
  /// A regular cons cell with a head and a tail
  Cons(Box<[FTerm]>),
  /// NIL [] zero sized list
  Nil,
  Tuple(Vec<FTerm>),
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
  /// A list of value/label pairs, a jump table
  ExtList_(Vec<FTerm>),
  AllocList_,
}

impl FTerm {
  /// Given a word, determine if it fits into Smallint (word size - 4 bits)
  /// otherwise form a BigInt
  pub fn from_word(w: Word) -> FTerm {
    if w < defs::MAX_UNSIG_SMALL {
      return FTerm::SmallInt(w as SWord)
    }
    FTerm::BigInt(Box::new(BigInt::from_usize(w).unwrap()))
  }

  /// Parse self as Atom_ (load-time atom) and return index to use with code loader.
  pub fn loadtime_atom_index(&self) -> Word {
    match self {
      &FTerm::Atom_(w) => w,
      _ => panic!("{}Expected load-time atom, got {:?}", module(), self)
    }
  }

  /// Parse self as Int_ (load-time integer) and return the contained value.
  pub fn loadtime_word(&self) -> Word {
    match self {
      &FTerm::Int_(w) => w,
      _ => panic!("{}Expected load-time int, got {:?}", module(), self)
    }
  }

  /// Convert a high level (friendly) term to a compact low-level term.
  /// Some terms cannot be converted, consider checking `to_lterm_vec()`
  pub fn to_lterm(&self) -> LTerm {
    match self {
      &FTerm::Atom(i) => LTerm::make_atom(i),
      &FTerm::X_(i) => LTerm::make_xreg(i),
      &FTerm::Y_(i) => LTerm::make_yreg(i),
      &FTerm::FP_(i) => LTerm::make_fpreg(i),
      &FTerm::Label_(i) => LTerm::make_label(i),
      &FTerm::SmallInt(i) => LTerm::make_small_i(i),
      &FTerm::Int_(i) => LTerm::make_small_u(i),
      &FTerm::Nil => LTerm::nil(),
      _ => panic!("{}Don't know how to convert {:?} to LTerm", module(), self)
    }
  }

  /// Converts a few special friendly terms, which hold longer structures into
  /// an array of Words (raw values of low_level LTerms).
  pub fn to_lterm_vec(&self) -> Vec<LTerm> {
    match self {
      &FTerm::ExtList_(ref v) => {
        let mut result: Vec<LTerm> = Vec::with_capacity(v.len());
        for x in v {
          result.push(x.to_lterm())
        };
        result
      },
      _ => panic!("{}Don't know how to convert {:?} to LTerm[]", module(), self)
    }
  }

}
