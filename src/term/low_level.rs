//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance
//!
use term::immediate;
use term::immediate::{IMM2_SPECIAL_NIL_PREFIX, IMM2_SPECIAL_NONVALUE_PREFIX};
use term::primary;

use defs;
use defs::{Word, SWord, MAX_UNSIG_SMALL, MIN_SIG_SMALL, MAX_SIG_SMALL};
//type Word = defs::Word;

use std::cmp::Ordering;
use std::fmt;


/// A low-level term, packed conveniently in a Word, or containing a
/// pointer to heap.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LTerm {
  value: Word
}


impl Ord for LTerm {
  fn cmp(&self, other: &LTerm) -> Ordering {
    self.value.cmp(&other.value)
  }
}


impl PartialOrd for LTerm {
  fn partial_cmp(&self, other: &LTerm) -> Option<Ordering> {
    Some(self.value.cmp(&other.value))
  }
}


impl LTerm {
  pub fn raw(&self) -> Word { self.value }

  pub fn nil() -> LTerm { LTerm { value: IMM2_SPECIAL_NIL_PREFIX } }

  pub fn is_nil(&self) -> bool {
    self.value == IMM2_SPECIAL_NIL_PREFIX
  }

  pub fn non_value() -> LTerm {
    LTerm { value: IMM2_SPECIAL_NONVALUE_PREFIX }
  }

  pub fn is_non_value(&self) -> bool {
    self.value == IMM2_SPECIAL_NONVALUE_PREFIX
  }

  pub fn is_pid(&self) -> bool {
    return immediate::is_pid_raw(self.value)
  }

  pub fn is_atom(&self) -> bool {
    return immediate::is_atom_raw(self.value)
  }

  pub fn atom_index(&self) -> Word { immediate::imm2_value(self.value) }

  // Get primary tag bits from a raw term
  #[inline]
  pub fn primary_tag(&self) -> primary::Tag {
    primary::from_word(self.value)
  }

  #[inline]
  pub fn is_imm(&self) -> bool {
    self.primary_tag() == primary::Tag::Immediate
  }

  #[inline]
  pub fn is_box(&self) -> bool {
    self.primary_tag() == primary::Tag::Box
  }

  #[inline]
  pub fn is_cons(&self) -> bool {
    self.primary_tag() == primary::Tag::Cons
  }

  #[inline]
  pub fn is_header(&self) -> bool {
    self.primary_tag() == primary::Tag::Header
  }

  #[inline]
  pub fn get_raw(&self) -> Word { self.value }

  //
  // Construction
  //

  /// Any raw word becomes a term, possibly invalid
  #[inline]
  pub fn make_from_raw(w: Word) -> LTerm {
    LTerm { value: w }
  }

  /// From atom index create an atom. To create from string use vm::new_atom
  #[inline]
  pub fn make_atom(index: Word) -> LTerm {

    LTerm { value: immediate::make_atom_raw(index)
    }
  }

  /// From internal process index create a pid. To create a process use vm::create_process
  #[inline]
  pub fn make_pid(pindex: Word) -> LTerm {
    LTerm { value: immediate::make_pid_raw(pindex) }
  }

  #[inline]
  pub fn make_xreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_xreg_raw(n) }
  }

  #[inline]
  pub fn make_yreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_yreg_raw(n) }
  }

  #[inline]
  pub fn make_fpreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_fpreg_raw(n) }
  }

  #[inline]
  pub fn make_label(n: Word) -> LTerm {
    LTerm { value: immediate::make_label_raw(n) }
  }

  //
  // Box services - boxing, unboxing, checking
  //

  #[inline]
  pub fn box_ptr(&self) -> *const Word {
    primary::pointer(self.value)
  }

  //
  // Small integer handling
  //
  #[inline]
  pub fn make_small_u(n: Word) -> LTerm {
    assert!(n < MAX_UNSIG_SMALL);
    LTerm { value: immediate::make_small_raw(n) }
  }

  #[inline]
  pub fn make_small_i(n: SWord) -> LTerm {
    // TODO: Do the proper min neg small
    assert!(n < MAX_SIG_SMALL && n > MIN_SIG_SMALL);
    let un = defs::unsafe_sword_to_word(n);
    LTerm { value: immediate::make_small_raw(un) }
  }

  #[inline]
  pub fn small_get(&self) -> SWord {
    let n = immediate::imm1_value(self.value);
    return defs::unsafe_word_to_sword(n);
  }
}

// Printing low_level Terms as "{}"
impl fmt::Display for LTerm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let v = self.value;

    match primary::primary_tag(v) {
      primary::Tag::Box => write!(f, "Box({:?})", self.box_ptr()),

      primary::Tag::Cons => write!(f, "Cons({})", v),

      primary::Tag::Immediate =>
        match immediate::get_imm1_tag(v) {
          immediate::Immediate1::Small => write!(f, "{}", self.small_get()),

          immediate::Immediate1::Immed2 =>
            match immediate::get_imm2_tag(v) {
              immediate::Immediate2::Atom =>
                write!(f, "Atom({})", self.atom_index()),
              _ => write!(f, "Imm2({})", self.value),
            },

          _ => write!(f, "Imm1({})", self.value),
        },
      primary::Tag::Header => write!(f, "Header({})", v),
    }
  }
}
