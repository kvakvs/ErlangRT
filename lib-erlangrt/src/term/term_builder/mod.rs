//! Implements term builder for use with library term algorithms (used to
//! decouple libraries from the actual term implementation).

pub mod bin_builder;
pub mod list_builder;
pub mod map_builder;
pub mod tuple_builder;

pub use self::{
  bin_builder::BinaryBuilder, list_builder::ListBuilder, map_builder::MapBuilder,
  tuple_builder::TupleBuilder,
};

use crate::{
  defs::BitSize,
  emulator::{atom, heap::Heap},
  fail::RtResult,
  term::{boxed, lterm::*},
};
use num;

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
    // Allocate uninitialized binary of some type
    let rbin = boxed::Binary::create_into(BitSize::with_bytes(data.len()), hp)?;
    // Store bytes
    (*rbin).store(data)?;
    Ok((*rbin).make_term())
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
    ListBuilder::new(self.heap)
  }

  pub fn create_map_builder(&mut self, size_hint: usize) -> RtResult<MapBuilder> {
    unsafe { MapBuilder::new(&mut (*self.heap), size_hint) }
  }
}
