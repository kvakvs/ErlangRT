//! Implements term builder for use with library term algorithms (used to
//! decouple libraries from the actual term implementation).
use emulator::atom;
use emulator::heap::Heap;
use rt_defs::heap::IHeap;
use rt_defs::term_builder::{ITermBuilder, IListBuilder, ITupleBuilder};
use rt_defs::Word;
use term::lterm::aspect_binary::{empty_binary};
use term::lterm::aspect_list::{nil};
use term::lterm::LTerm;
use term::raw::rcons::ConsPtrMut;

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
pub struct ListBuilder {
  p: ConsPtrMut,
}

impl ListBuilder {
  pub fn new(p: *mut Word) -> ListBuilder {
    ListBuilder { p: ConsPtrMut::from_pointer(p) }
  }
}

impl IListBuilder<LTerm> for ListBuilder {
  unsafe fn set(&mut self, val: LTerm) {
    self.p.set_hd(val)
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
pub struct TermBuilder<'a> {
  heap: &'a mut Heap,
}


impl<'a> TermBuilder<'a> {
  pub fn new(hp: &mut Heap) -> TermBuilder {
    TermBuilder { heap: hp }
  }
}


impl<'a> ITermBuilder for TermBuilder<'a> {
  type TermT = LTerm;
  type TupleBuilderT = TupleBuilder;
  type ListBuilderT = ListBuilder;

  fn create_bignum(&self, _n: num::BigInt) -> Self::TermT {
    unimplemented!()
  }

  fn create_binary(&mut self, _b: &[u8]) -> Self::TermT {
    unimplemented!()
  }

  #[inline]
  fn create_atom_str(&self, a: &str) -> Self::TermT {
    atom::from_str(a)
  }

  #[inline]
  fn create_nil(&self) -> Self::TermT {
    nil()
  }

  fn create_small_s(&self, _n: isize) -> Self::TermT {
    unimplemented!()
  }

  #[inline]
  fn create_empty_binary(&self) -> Self::TermT {
    empty_binary()
  }

  fn create_tuple_builder(&mut self, _sz: usize) -> Self::TupleBuilderT {
    unimplemented!()
  }

  fn create_list_builder(&mut self) -> Self::ListBuilderT {
    let p = self.heap.heap_allocate(2, true).unwrap();
    ListBuilder::new(p)
  }
}
