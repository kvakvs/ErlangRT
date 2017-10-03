//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance
//!
use term::immediate;
use term::immediate::{IMM2_SPECIAL_NIL_PREFIX, IMM2_SPECIAL_NONVALUE_PREFIX};
use term::primary_tag;

use defs;
type Word = defs::Word;

use std::cmp::Ordering;


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
  pub fn primary_tag(&self) -> primary_tag::Tag {
    primary_tag::from_word(self.value)
  }

  pub fn is_imm(&self) -> bool {
    self.primary_tag() == primary_tag::Tag::Immediate
  }

  pub fn is_box(&self) -> bool {
    self.primary_tag() == primary_tag::Tag::Box
  }

  pub fn is_cons(&self) -> bool {
    self.primary_tag() == primary_tag::Tag::Cons
  }

  pub fn is_header(&self) -> bool {
    self.primary_tag() == primary_tag::Tag::Header
  }

  pub fn get_raw(&self) -> Word { self.value }

  //
  // Construction
  //

  /// Any raw word becomes a term, possibly invalid
  pub fn make_from_raw(w: Word) -> LTerm {
    LTerm { value: w }
  }

  /// From atom index create an atom. To create from string use vm::new_atom
  pub fn make_atom(index: Word) -> LTerm {
    LTerm { value: immediate::make_atom_raw(index) }
  }

  pub fn make_small(n: Word) -> LTerm {
    assert!(0 <= n && n < defs::MAX_POS_SMALL);
    LTerm { value: immediate::make_pid_raw(n) }
  }

  /// From internal process index create a pid. To create a process use vm::create_process
  pub fn make_pid(pindex: Word) -> LTerm {
    LTerm { value: immediate::make_pid_raw(pindex) }
  }

  pub fn make_xreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_xreg_raw(n) }
  }
  pub fn make_yreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_yreg_raw(n) }
  }
  pub fn make_fpreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_fpreg_raw(n) }
  }
}
