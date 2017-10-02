//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance
//!
use types::Word;
use term::immediate;
use term::immediate::{RAW_NIL, RAW_NON_VALUE, RAW_PID, RAW_ATOM};
use term::primary_tag;

use std::cmp::Ordering;


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Term {
  value: Word
}


impl Ord for Term {
  fn cmp(&self, other: &Term) -> Ordering {
    self.value.cmp(&other.value)
  }
}


impl PartialOrd for Term {
  fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
    Some(self.value.cmp(&other.value))
  }
}


impl Term {
  pub fn nil() -> Term { Term { value: RAW_NIL } }

  pub fn is_nil(&self) -> bool {
    self.value == RAW_NIL
  }

  pub fn non_value() -> Term {
    Term { value: RAW_NON_VALUE }
  }

  pub fn is_non_value(&self) -> bool {
    self.value == RAW_NON_VALUE
  }

  pub fn is_pid(&self) -> bool {
    return (self.value & immediate::IMM1_MASK) == RAW_PID
  }

  pub fn is_atom(&self) -> bool {
    (self.value & immediate::IMM2_MASK) == RAW_ATOM
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

  // Any word becomes a term
  pub fn make_from_raw(w: Word) -> Term {
    Term { value: w }
  }

  // From atom index create an atom. To create from string use vm::new_atom
  pub fn make_atom(index: Word) -> Term {
    Term { value: immediate::make_atom_raw(index) }
  }

  // From internal process index create a pid. To create a process use vm::create_process
  pub fn make_pid(pindex: Word) -> Term {
    Term { value: immediate::make_pid_raw(pindex) }
  }
}
