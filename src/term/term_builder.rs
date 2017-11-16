//! Implements term builder for use with library term algorithms (used to
//! decouple libraries from the actual term implementation).
use rt_defs::heap::IHeap;
use emulator::atom;
use term::lterm::LTerm;
use term::lterm::aspect_list::{nil};
use term::lterm::aspect_binary::{empty_binary};
use rt_defs::term_builder::{ITermBuilder, IListBuilder, ITupleBuilder};

use num;


/// A specific tuple builder implementation for LTerm and ERT VM.
pub struct TupleBuilder {}


impl ITupleBuilder<LTerm> for TupleBuilder {
  unsafe fn set_element_base0(&mut self, i: usize, val: LTerm) {
    unimplemented!()
  }

  fn make_term(&self) -> LTerm {
    unimplemented!()
  }
}


/// A forward list builder implementation for LTerm and ERT VM.
pub struct ListBuilder {}

impl IListBuilder<LTerm> for ListBuilder {
  unsafe fn set(&mut self, val: LTerm) {
    unimplemented!()
  }

  unsafe fn next(&mut self) {
    unimplemented!()
  }

  unsafe fn end(&mut self, tl: LTerm) {
    unimplemented!()
  }

  fn make_term(&self) -> LTerm {
    unimplemented!()
  }
}


/// Term Builder implementation for LTerm and ERT VM.
pub struct TermBuilder {}


impl TermBuilder {
  pub fn new(hp: &mut IHeap) -> TermBuilder {
    TermBuilder{}
  }
}


impl ITermBuilder<LTerm> for TermBuilder {
  fn create_bignum(&self, n: num::BigInt) -> LTerm {
    unimplemented!()
  }

  fn create_binary(&mut self, b: &[u8]) -> LTerm {
    unimplemented!()
  }

  #[inline]
  fn create_atom_str(&self, a: &str) -> LTerm {
    atom::from_str(a)
  }

  #[inline]
  fn create_nil(&self) -> LTerm {
    nil()
  }

  fn create_small_s(&self, n: isize) -> LTerm {
    unimplemented!()
  }

  #[inline]
  fn create_empty_binary(&self) -> LTerm {
    empty_binary()
  }

  fn create_tuple_builder(&mut self, sz: usize) -> Box<ITupleBuilder<LTerm>> {
    unimplemented!()
  }

  fn create_list_builder(&mut self) -> Box<IListBuilder<LTerm>> {
    unimplemented!()
  }
}
