//! Implements term builder for use with library term algorithms (used to
//! decouple libraries from the actual term implementation).
use emulator::atom;
use emulator::heap::{Heap};
use emulator::heap::IHeap;
use rt_defs::term_builder::{ITermBuilder, IListBuilder, ITupleBuilder};
use term::lterm::*;
use term::raw::*;
use term::boxed;

use num;
use fail::Hopefully;


// TODO: Remove templating on term type here

/// A specific tuple builder implementation for `LTerm` and ERT VM.
pub struct TupleBuilder {
  p: *mut boxed::Tuple
}

impl TupleBuilder {
  pub fn new(p: *mut boxed::Tuple) -> TupleBuilder {
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
  p0: *mut LTerm,
  // current (last) cell
  p: *mut LTerm,
  // because i can't into lifetimes :( but it lives short anyway
  heap: *mut Heap,
}

impl ListBuilder {
  unsafe fn new(heap: *mut Heap) -> Hopefully<ListBuilder> {
    let p = (*heap).heap_allocate(2, true)?;

    Ok(ListBuilder {
      p: p as *mut LTerm,
      p0: p as *mut LTerm,
      heap,
    })
  }
}

impl IListBuilder<LTerm> for ListBuilder {
  unsafe fn set(&mut self, val: LTerm) {
    self.p.set_hd(val)
  }

  unsafe fn next(&mut self) {
    let new_cell = (*self.heap).heap_allocate(
      2, true
    )? as *mut LTerm;
    self.p.set_tl(new_cell.make_cons());
    self.p = new_cell
  }

  unsafe fn end(&mut self, _tl: LTerm) {
    self.p.set_tl(LTerm::nil())
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


  unsafe fn create_bignum(&self, n: num::BigInt) -> Hopefully<Self::TermT> {
    let ref_heap = self.heap.as_mut().unwrap();
    let big_p = boxed::Bignum::place_into(ref_heap, n)?;
    Ok(LTerm::make_boxed(big_p))
  }


  unsafe fn create_binary(&mut self, data: &[u8]) -> Hopefully<Self::TermT> {
    debug_assert!(self.heap.is_null() == false);
    let hp = self.heap.as_mut().unwrap();
    let rbin = boxed::Binary::place_into(hp, data.len())?;
    boxed::Binary::store(rbin, data);
    Ok(LTerm::make_boxed(rbin))
  }


  #[inline]
  fn create_atom_str(&self, a: &str) -> Self::TermT {
    atom::from_str(a)
  }


  #[inline]
  fn create_nil(&self) -> Self::TermT {
    LTerm::nil()
  }


  #[inline]
  fn create_small_s(&self, n: isize) -> Self::TermT {
    LTerm::make_small_signed(n)
  }


  #[inline]
  fn create_empty_binary(&self) -> Self::TermT {
    LTerm::make_empty_binary()
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
