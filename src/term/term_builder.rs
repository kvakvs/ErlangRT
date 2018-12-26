//! Implements term builder for use with library term algorithms (used to
//! decouple libraries from the actual term implementation).
use crate::{
  defs::{ByteSize, WordSize},
  emulator::{atom, heap::Heap},
  fail::RtResult,
  term::{boxed, lterm::*},
};
use num;

// TODO: Remove templating on term type here

/// A specific tuple builder implementation for `LTerm` and ERT VM.
pub struct TupleBuilder {
  p: *mut boxed::Tuple,
}

impl TupleBuilder {
  #[inline]
  pub fn with_arity(hp: &mut Heap, arity: usize) -> RtResult<Self> {
    let p = boxed::Tuple::create_into(hp, arity)?;
    Ok(Self::new(p))
  }

  #[inline]
  pub fn new(p: *mut boxed::Tuple) -> Self {
    Self { p }
  }

  #[inline]
  pub unsafe fn set_element_base0(&self, i: usize, val: LTerm) {
    boxed::Tuple::set_element_base0(self.p, i, val)
  }

  #[inline]
  pub fn make_term(&self) -> LTerm {
    LTerm::make_boxed(self.p)
  }
}

/// A forward list builder implementation for `LTerm` and RT VM.
///
/// 1. Create ListBuilder with the heap where you want to build.
/// 2. Set first element with `set`
/// 3. Set tail of the first element to a new cell by calling `next`
/// 4. Finalize by writing last NIL by calling `end`.
pub struct ListBuilder {
  // first cell where the building started
  first_cell_p: *const boxed::Cons,
  // current (last) cell
  write_p: *mut boxed::Cons,
  // because i can't into lifetimes :( but it lives short anyway
  heap: *mut Heap,
}

impl ListBuilder {
  pub unsafe fn new(heap: *mut Heap) -> RtResult<ListBuilder> {
    let p = (*heap).alloc::<boxed::Cons>(WordSize::new(2), true)?;

    Ok(ListBuilder {
      write_p: p,
      first_cell_p: p,
      heap,
    })
  }

  pub fn get_write_p(&self) -> *mut boxed::Cons {
    self.write_p
  }

  pub unsafe fn set(&mut self, val: LTerm) {
    (*self.write_p).set_hd(val)
  }

  pub unsafe fn next(&mut self) -> RtResult<()> {
    let new_cell = (*self.heap).alloc::<boxed::Cons>(WordSize::new(2), true)?;
    (*self.write_p).set_tl(LTerm::make_cons(new_cell));
    self.write_p = new_cell;
    Ok(())
  }

  pub unsafe fn end(&mut self, tl: LTerm) {
    (*self.write_p).set_tl(tl)
  }

  #[allow(dead_code)]
  pub unsafe fn end_with_nil(&mut self) {
    (*self.write_p).set_tl(LTerm::nil())
  }

  pub fn make_term(&self) -> LTerm {
    LTerm::make_cons(self.first_cell_p)
  }
}

/// Term Builder implementation for `LTerm` and ERT VM.
pub struct TermBuilder {
  // because i can't into lifetimes :( but it lives short anyway
  heap: *mut Heap,
}

impl TermBuilder {
  pub fn new(hp: &mut Heap) -> TermBuilder {
    TermBuilder {
      heap: hp as *mut Heap,
    }
  }

  pub unsafe fn create_bignum(&self, n: num::BigInt) -> RtResult<LTerm> {
    let ref_heap = self.heap.as_mut().unwrap();
    let big_p = boxed::Bignum::create_into(ref_heap, n)?;
    Ok(LTerm::make_boxed(big_p))
  }

  pub unsafe fn create_binary(&mut self, data: &[u8]) -> RtResult<LTerm> {
    debug_assert!(!self.heap.is_null());
    let hp = self.heap.as_mut().unwrap();
    let rbin = boxed::Binary::create_into(hp, ByteSize::new(data.len()))?;
    boxed::Binary::store(rbin, data)?;
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

  pub fn create_tuple_builder(&mut self, sz: usize) -> RtResult<TupleBuilder> {
    let ref_heap = unsafe { self.heap.as_mut() }.unwrap();
    let raw_tuple = boxed::Tuple::create_into(ref_heap, sz)?;
    Ok(TupleBuilder::new(raw_tuple))
  }

  pub fn create_list_builder(&mut self) -> RtResult<ListBuilder> {
    unsafe { ListBuilder::new(self.heap) }
  }
}
