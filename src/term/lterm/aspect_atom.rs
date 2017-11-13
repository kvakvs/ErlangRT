//! Functions to manipulate an LTerm as an Erlang atom. Part of LTerm impl.

use defs::Word;
use term::immediate;


pub trait AtomTerm {
  /// Check whether a value is a runtime atom.
  fn is_atom(&self) -> bool;
  /// For an atom value, get index.
  fn atom_index(&self) -> Word;
}


impl AtomTerm for super::LTerm {
  #[inline]
  fn is_atom(&self) -> bool {
    immediate::is_atom_raw(self.value)
  }


  #[inline]
  fn atom_index(&self) -> Word {
    assert!(self.is_atom());
    immediate::get_imm2_value(self.value)
  }
}


/// From atom index create an atom. To create from string use vm::new_atom
#[inline]
pub fn make_atom(index: Word) -> super::LTerm {
  super::LTerm { value: immediate::make_atom_raw(index) }
}
