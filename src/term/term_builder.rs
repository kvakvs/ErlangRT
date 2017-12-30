//! Implements term builder for use with library term algorithms (used to
//! decouple libraries from the actual term implementation).
use emulator::atom;
use emulator::heap::{Heap, allocate_tuple};
use rt_defs::heap::IHeap;
use rt_defs::term_builder::{ITermBuilder, IListBuilder, ITupleBuilder};
use term::lterm::*;
use term::raw::*;

use num;


/// A specific tuple builder implementation for `LTerm` and ERT VM.
pub struct TupleBuilder {
  p: rtuple::PtrMut
}

impl TupleBuilder {
  pub fn new(p: rtuple::PtrMut) -> TupleBuilder {
    TupleBuilder { p }
  }
}


impl ITupleBuilder<LTerm> for TupleBuilder {
  unsafe fn set_element_base0(&mut self, i: usize, val: LTerm) {
    self.p.set_element_base0(i, val)
  }

  fn make_term(&self) -> LTerm {
    self.p.make_term()
  }
}

/// A forward list builder implementation for `LTerm` and ERT VM.
pub struct ListBuilder {
  // first cell where the building started
  p0: rcons::PtrMut,
  // current (last) cell
  p: rcons::PtrMut,
  // because i can't into lifetimes :( but it lives short anyway
  heap: *mut Heap,
}

impl ListBuilder {
  unsafe fn new(heap: *mut Heap) -> ListBuilder {
    let p = (*heap).heap_allocate(2, true).unwrap();

    ListBuilder {
      p: rcons::PtrMut::from_pointer(p),
      p0: rcons::PtrMut::from_pointer(p),
      heap,
    }
  }
}

impl IListBuilder<LTerm> for ListBuilder {
  unsafe fn set(&mut self, val: LTerm) {
    self.p.set_hd(val)
  }

  unsafe fn next(&mut self) {
    let new_cell = rcons::PtrMut::from_pointer(
      (*self.heap).heap_allocate(2, true).unwrap()
    );
    self.p.set_tl(new_cell.make_cons());
    self.p = new_cell
  }

  unsafe fn end(&mut self, _tl: LTerm) {
    self.p.set_tl(nil())
  }

  fn make_term(&self) -> LTerm {
    self.p0.make_cons()
  }
}


/// Term Builder implementation for `LTerm` and ERT VM.
pub struct TermBuilder {
  // because i can't into lifetimes :( but it lives short anyway
  heap: *mut Heap,
}


impl TermBuilder {
  pub fn new(hp: &mut Heap) -> TermBuilder {
    TermBuilder { heap: hp as *mut Heap }
  }
}


impl ITermBuilder for TermBuilder {
  type TermT = LTerm;
  type TupleBuilderT = TupleBuilder;
  type ListBuilderT = ListBuilder;


  unsafe fn create_bignum(&self, n: num::BigInt) -> Self::TermT {
    let ref_heap = self.heap.as_mut().unwrap();
    let big_p = HOBignum::place_into(ref_heap, n).unwrap();
    HOBignum::make_term(big_p)
  }


  unsafe fn create_binary(&mut self, b: &[u8]) -> Self::TermT {
    let ref_heap = self.heap.as_mut().unwrap();
    let rbin = HOBinary::place_into(ref_heap, b.len()).unwrap();
    HOBinary::store(rbin, b);
    HOBinary::make_term(rbin)
  }


  #[inline]
  fn create_atom_str(&self, a: &str) -> Self::TermT {
    atom::from_str(a)
  }


  #[inline]
  fn create_nil(&self) -> Self::TermT {
    nil()
  }


  #[inline]
  fn create_small_s(&self, n: isize) -> Self::TermT {
    make_small_s(n)
  }


  #[inline]
  fn create_empty_binary(&self) -> Self::TermT {
    empty_binary()
  }


  fn create_tuple_builder(&mut self, sz: usize) -> Self::TupleBuilderT {
    let ref_heap = unsafe { self.heap.as_mut() }.unwrap();
    let raw_tuple = allocate_tuple(ref_heap, sz).unwrap();
    TupleBuilder::new(raw_tuple)
  }


  fn create_list_builder(&mut self) -> Self::ListBuilderT {
    unsafe { ListBuilder::new(self.heap) }
  }
}
