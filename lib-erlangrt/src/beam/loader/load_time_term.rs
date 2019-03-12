////! Loadtime term library.
////! Representing Erlang terms as a complex Rust enum, more developer friendly,
////! there's an memory cost, but we don't care yet. This is only used at the
////! loading time, not for internal VM logic. VM uses its own `Term` which is
////! a memory efficient representation.
// use crate::{
//  big,
//  defs::{SWord, Word},
//  emulator::heap::Heap,
//  term::lterm::*,
//};
// fn module() -> &'static str {
//  "loader/lt_term: "
//}
//
///// A friendly Rust-enum representing Erlang term both runtime and load-time
///// values. Make sure to crash nicely when runtime mixes with load-time.
//#[repr(u8)]
//#[derive(Debug, PartialEq, Clone)]
//#[allow(dead_code)]
//// TODO: Remove deadcode directive later and fix
// pub enum LtTerm {
//  /// Runtime atom index in the VM atom table
//  Atom(Word),
//  SmallInt(SWord),
//  BigInt(big::Big),
//  /// A regular cons cell with a head and a tail
//  Cons(Box<[LtTerm]>),
//  /// NIL [] - an empty list
//  Nil,
//  Tuple(Vec<LtTerm>),
//  /// zero sized tuple
//  Tuple0,
//  Float(f64),
//
//  // Internal values not visible in the user data
//  /// A runtime index of X register
//  XRegister(Word),
//  /// A runtime index of a stack cell relative to the stack top (Y register)
//  YRegister(Word),
//  /// A runtime index of a floating-point register
//  FloatRegister(Word),
//
//  // BEAM loader specials, these never occur at runtime and finding them
//  // in runtime must be an error.
//  /// A load-time index of label
//  LoadtimeLabel(Word),
//  /// A load-time atom index in the loader atom table
//  LoadtimeAtom(usize),
//  // /// A load-time word value literally specified
//  // LoadtimeInt(SWord),
//  /// A load-time index in literal heap
//  LoadtimeLiteral(Word),
//  /// A list of value/label pairs, a jump table
//  LoadtimeExtlist(Vec<LtTerm>),
//  LoadtimeAlloclist,
//}
// impl LtTerm {
//  /// Given a word, determine if it fits into Smallint (word size - 4 bits)
//  /// otherwise form a BigInt
//  pub fn from_word(s: SWord) -> LtTerm {
//    if Term::small_fits(s) {
//      return LtTerm::SmallInt(s as SWord);
//    }
//    LtTerm::BigInt(big::Big::from_isize(s))
//  }
//
//  /// Parse self as Int_ (load-time integer) and return the contained value.
//  pub fn loadtime_word(&self) -> SWord {
//    if let LtTerm::SmallInt(w) = *self {
//      return w;
//    }
//    panic!("{}Expected a smallint, got {:?}", module(), self)
//  }
//
//  /// Convert a high level (friendly) term to a compact low-level term.
//  /// Some terms cannot be converted, consider checking `to_lterm_vec()`
//  pub fn to_lterm(&self, _heap: &mut Heap, lit_tab: &Vec<Term>) -> Term {
//    match *self {
//      LtTerm::Atom(i) => Term::make_atom(i),
//      LtTerm::XRegister(i) => Term::make_regx(i),
//      LtTerm::YRegister(i) => Term::make_regy(i),
//      LtTerm::FloatRegister(i) => Term::make_regfp(i),
//      LtTerm::SmallInt(i) => Term::make_small_signed(i),
//      LtTerm::Nil => Term::nil(),
//      LtTerm::LoadtimeLiteral(lit_index) => lit_tab[lit_index],
//      _ => panic!("{}Don't know how to convert {:?} to Term", module(), self),
//    }
//  }
//}
