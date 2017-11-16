//! Implements term builder for use with library term algorithms (used to
//! decouple libraries from the actual term implementation).
use emulator::heap::Heap;
use emulator::atom;
use term::lterm::LTerm;
use term::lterm::aspect_list::{nil};
use term::lterm::aspect_binary::{empty_binary};
use rt_defs::term_builder::{ITermBuilder, IListBuilder, ITupleBuilder};

use num;


/// A specific tuple builder implementation for LTerm and ERT VM.
pub struct TupleBuilder {}


impl ITupleBuilder<LTerm> for TupleBuilder {
  unsafe fn set_element_base0(&mut self, _i: usize, _val: LTerm) {
    unimplemented!()
  }

  fn make_term(&self) -> LTerm {
    unimplemented!()
  }
}


/// A forward list builder implementation for LTerm and ERT VM.
pub struct ListBuilder {}

impl IListBuilder<LTerm> for ListBuilder {
  unsafe fn set(&mut self, _val: LTerm) {
    unimplemented!()
  }

  unsafe fn next(&mut self) {
    unimplemented!()
  }

  unsafe fn end(&mut self, _tl: LTerm) {
    unimplemented!()
  }

  fn make_term(&self) -> LTerm {
    unimplemented!()
  }
}


/// Term Builder implementation for LTerm and ERT VM.
pub struct TermBuilder {}


impl TermBuilder {
  pub fn new(_hp: &mut Heap) -> TermBuilder {
    TermBuilder{}
  }
}


impl ITermBuilder<LTerm> for TermBuilder {
  fn create_bignum(&self, _n: num::BigInt) -> LTerm {
    unimplemented!()
  }

  fn create_binary(&mut self, _b: &[u8]) -> LTerm {
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

  fn create_small_s(&self, _n: isize) -> LTerm {
    unimplemented!()
  }

  #[inline]
  fn create_empty_binary(&self) -> LTerm {
    empty_binary()
  }

  fn create_tuple_builder(&mut self, _sz: usize) -> Box<ITupleBuilder<LTerm>> {
    unimplemented!()
  }

  fn create_list_builder(&mut self) -> Box<IListBuilder<LTerm>> {
    unimplemented!()
  }
}
