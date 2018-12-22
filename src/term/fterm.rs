//! Friendly term library
//!
//! Representing Erlang terms as a complex Rust enum, more developer friendly,
//! there's an memory cost, but we don't care yet. This is only used at the
//! loading time, not for internal VM logic. VM uses `low_level::LTerm`
//!
use crate::{
  defs::{SWord, Word},
  emulator::heap::Heap,
  term::lterm::*,
};
use num::{bigint::BigInt, FromPrimitive};

fn module() -> &'static str {
  "term::friendly: "
}

/// A friendly Rust-enum representing Erlang term both runtime and load-time
/// values. Make sure to crash nicely when runtime mixes with load-time.
#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
// TODO: Remove deadcode directive later and fix
pub enum FTerm {
  /// Runtime atom index in the VM atom table
  Atom(Word),
  SmallInt(SWord),
  BigInt(Box<BigInt>),
  /// A regular cons cell with a head and a tail
  Cons(Box<[FTerm]>),
  /// NIL [] zero sized list
  Nil,
  Tuple(Vec<FTerm>),
  /// zero sized tuple
  Tuple0,
  Float(f64),

  // Internal values not visible in the user data
  /// A runtime index of X register
  X_(Word),
  /// A runtime index of a stack cell relative to the stack top (Y register)
  Y_(Word),
  /// A runtime index of a floating-point register
  FP_(Word),

  // BEAM loader specials, these never occur at runtime and finding them
  // in runtime must be an error.
  /// A load-time index of label
  LoadTimeLabel(Word),
  /// A load-time atom index in the loader atom table
  LoadTimeAtom(usize),
  // /// A load-time word value literally specified
  // LoadTimeInt(SWord),
  /// A load-time index in literal heap
  LoadTimeLit(Word),
  /// A list of value/label pairs, a jump table
  LoadTimeExtlist(Vec<FTerm>),
  LoadTimeAlloclist,
}

impl FTerm {
  /// Given a word, determine if it fits into Smallint (word size - 4 bits)
  /// otherwise form a BigInt
  pub fn from_word(s: SWord) -> FTerm {
    if LTerm::small_fits(s) {
      return FTerm::SmallInt(s as SWord);
    }
    FTerm::BigInt(Box::new(BigInt::from_isize(s).unwrap()))
  }

  /// Parse self as Int_ (load-time integer) and return the contained value.
  pub fn loadtime_word(&self) -> SWord {
    if let FTerm::SmallInt(w) = *self {
      return w;
    }
    panic!("{}Expected a smallint, got {:?}", module(), self)
  }

  /// Convert a high level (friendly) term to a compact low-level term.
  /// Some terms cannot be converted, consider checking `to_lterm_vec()`
  pub fn to_lterm(&self, _heap: &mut Heap) -> LTerm {
    match *self {
      FTerm::Atom(i) => LTerm::make_atom(i),
      FTerm::X_(i) => LTerm::make_regx(i),
      FTerm::Y_(i) => LTerm::make_regy(i),
      FTerm::FP_(i) => LTerm::make_regfp(i),
      FTerm::SmallInt(i) => LTerm::make_small_signed(i),
      FTerm::Nil => LTerm::nil(),
      _ => panic!("{}Don't know how to convert {:?} to LTerm", module(), self),
    }
  }

  //  /// Converts a few special friendly terms, which hold longer structures into
  //  /// an array of Words (raw values of low_level LTerms).
  //  pub fn to_lterm_vec(&self) -> Vec<LTerm> {
  //    match self {
  //      &FTerm::ExtList_(ref v) => {
  //        let mut result: Vec<LTerm> = Vec::with_capacity(v.len() + 1);
  //        result.push(LTerm::make_header(v.len()));
  //        for x in v.iter() {
  //          result.push(x.to_lterm())
  //        };
  //        result
  //      },
  //      _ => panic!("{}Don't know how to convert {:?} to LTerm[]", module(), self)
  //    }
  //  }
}
