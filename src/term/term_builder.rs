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
  pub fn with_arity(arity: usize, hp: &mut Heap) -> RtResult<Self> {
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
  // first cell where the building started (used to make the list term, also
  // used to prepend to list)
  pub head_p: *mut boxed::Cons,
  // last cell (used to append to list)
  pub tail_p: *mut boxed::Cons,
  // because i can't into lifetimes :( but it lives short anyway
  heap: *mut Heap,
}

impl ListBuilder {
  pub unsafe fn new(heap: *mut Heap) -> RtResult<ListBuilder> {
    Ok(ListBuilder {
      head_p: core::ptr::null_mut(),
      tail_p: core::ptr::null_mut(),
      heap,
    })
  }

  /// Creates a new cons cell to grow the list either back or forward
  #[inline]
  unsafe fn make_cell(&self) -> RtResult<*mut boxed::Cons> {
    (*self.heap).alloc::<boxed::Cons>(WordSize::new(2), true)
  }

  /// Build list forward: Set current tail to a newly allocated cons (next cell).
  /// New cell becomes the current.
  /// Remember to terminate with NIL.
  pub unsafe fn append(&mut self, val: LTerm) -> RtResult<()> {
    if self.head_p.is_null() {
      // First cell in the list, make it the only cell in list
      self.tail_p = self.make_cell()?;
      self.head_p = self.tail_p;
    } else {
      // Link old tail to new cell
      let new_cell = self.make_cell()?;
      (*self.tail_p).set_tl(LTerm::make_cons(new_cell));
      self.tail_p = new_cell;
    }
    (*self.tail_p).set_hd(val);
    Ok(())
  }

  /// Build list back: Create a new cons, where tail points to current.
  /// New previous cell becomes the current.
  /// Remember to terminate the first cell of the list with NIL.
  pub unsafe fn prepend(&mut self, val: LTerm) -> RtResult<()> {
    if self.head_p.is_null() {
      self.head_p = self.make_cell()?;
      self.tail_p = self.head_p;
    } else {
      let new_cell = self.make_cell()?;
      (*new_cell).set_tl(LTerm::make_cons(self.head_p));
      self.head_p = new_cell;
    }
    (*self.head_p).set_hd(val);
    Ok(())
  }

  pub unsafe fn set_tail(&mut self, tl: LTerm) {
    (*self.tail_p).set_tl(tl)
  }

  pub fn make_term(&self) -> LTerm {
    LTerm::make_cons(self.head_p)
  }

  pub unsafe fn make_term_with_tail(&mut self, tail: LTerm) -> LTerm {
    // Cannot set tail if no cells were allocated
    assert!(!self.head_p.is_null());
    self.set_tail(tail);
    LTerm::make_cons(self.head_p)
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
  pub fn create_small_s(&self, n: isize) -> LTerm {
    LTerm::make_small_signed(n)
  }

  pub fn create_tuple_builder(&mut self, sz: usize) -> RtResult<TupleBuilder> {
    let ref_heap = unsafe { self.heap.as_mut() }.unwrap();
    let raw_tuple = boxed::Tuple::create_into(ref_heap, sz)?;
    Ok(TupleBuilder::new(raw_tuple))
  }

  pub fn create_list_builder(&mut self) -> RtResult<ListBuilder> {
    unsafe { ListBuilder::new(self.heap) }
  }
}
