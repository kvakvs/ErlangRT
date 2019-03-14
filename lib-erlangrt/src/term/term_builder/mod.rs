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
  big,
  defs::BitSize,
  emulator::{atom, heap::Heap},
  fail::RtResult,
  term::{
    boxed::{self, bignum, endianness::Endianness},
    lterm::*,
  },
};

/// Term Builder implementation for `Term` and ERT VM.
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

  /// Given an array of bytes with little-endian order, create a bignum on the
  /// provided heap, or fail.
  pub unsafe fn create_bignum_le(
    &self,
    sign: bignum::sign::Sign,
    digits: Vec<u8>,
  ) -> RtResult<Term> {
    let ref_heap = self.heap.as_mut().unwrap();
    let limbs = big::make_limbs_from_bytes(Endianness::Little, digits);
    let p = boxed::Bignum::create_into(ref_heap, sign, &limbs)?;
    Ok(Term::make_boxed(p))
  }

  pub unsafe fn create_binary(&mut self, data: &[u8]) -> RtResult<Term> {
    debug_assert!(!self.heap.is_null());
    let hp = self.heap.as_mut().unwrap();
    // Allocate uninitialized binary of some type
    let rbin = boxed::Binary::create_into(BitSize::with_bytes(data.len()), hp)?;
    // Store bytes
    (*rbin).store(data)?;
    Ok((*rbin).make_term())
  }

  #[inline]
  pub fn create_atom_str(&self, a: &str) -> Term {
    atom::from_str(a)
  }

  #[inline]
  pub fn create_small_s(&self, n: isize) -> Term {
    Term::make_small_signed(n)
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
