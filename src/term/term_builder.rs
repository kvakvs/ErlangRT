//! Implements term builder for use with library term algorithms (used to
//! decouple libraries from the actual term implementation).
use emulator::atom;
use emulator::heap::{Heap};
use term::lterm::*;
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

  pub unsafe fn set_element_base0(&mut self, i: usize, val: LTerm) {
    boxed::Tuple::set_element_base0(self.p, i, val)
  }

  #[inline]
  pub fn make_term(&self) -> LTerm {
    LTerm::make_boxed(self.p)
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
  pub unsafe fn new(heap: *mut Heap) -> Hopefully<ListBuilder> {
    let p = (*heap).alloc::<LTerm>(2, true)?;

    Ok(ListBuilder {
      p ,
      p0: p,
      heap,
    })
  }

  pub unsafe fn set(&mut self, val: LTerm) {
    core::ptr::write(self.p, val)
  }

  pub unsafe fn next(&mut self) -> Hopefully<()> {
    let new_cell = (*self.heap).alloc::<LTerm>(2, true)?;
    core::ptr::write(self.p.add(1), LTerm::make_cons(new_cell));
    self.p = new_cell;
    Ok(())
  }

  pub unsafe fn end(&mut self, _tl: LTerm) {
    core::ptr::write(self.p.add(1), LTerm::nil())
  }

  pub fn make_term(&self) -> LTerm {
    LTerm::make_cons(self.p0)
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


  pub unsafe fn create_bignum(&self, n: num::BigInt) -> Hopefully<LTerm> {
    let ref_heap = self.heap.as_mut().unwrap();
    let big_p = boxed::Bignum::create_into(ref_heap, n)?;
    Ok(LTerm::make_boxed(big_p))
  }


  pub unsafe fn create_binary(&mut self, data: &[u8]) -> Hopefully<LTerm> {
    debug_assert!(self.heap.is_null() == false);
    let hp = self.heap.as_mut().unwrap();
    let rbin = boxed::Binary::create_into(hp, data.len())?;
    boxed::Binary::store(rbin, data);
    Ok(LTerm::make_boxed(rbin))
  }


  #[inline]
  pub fn create_atom_str(&self, a: &str) -> LTerm {
    atom::from_str(a)
  }


  #[inline]
  pub fn create_nil(&self) -> LTerm {
    LTerm::nil()
  }


  #[inline]
  pub fn create_small_s(&self, n: isize) -> LTerm {
    LTerm::make_small_signed(n)
  }


//  #[inline]
//  pub fn create_empty_binary(&self) -> LTerm {
//    LTerm::make_empty_binary()
//  }


  pub fn create_tuple_builder(&mut self, sz: usize) -> Hopefully<TupleBuilder> {
    let ref_heap = unsafe { self.heap.as_mut() }.unwrap();
    let raw_tuple = boxed::Tuple::create_into(ref_heap, sz)?;
    Ok(TupleBuilder::new(raw_tuple))
  }


  pub fn create_list_builder(&mut self) -> Hopefully<ListBuilder> {
    unsafe { ListBuilder::new(self.heap) }
  }
}
