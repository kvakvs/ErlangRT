//use crate::defs::BitSize;
//use core::ptr::Unique;
//use ramp::ll::{
//  limb::Limb,
//  limb_ptr::{Limbs, LimbsMut},
//};
//
///// Limbs are Ramps' pointer to data, each limb is an usize-sized 'digit'
//pub type BigDigits = Limbs;
//pub type BigDigitsMut = LimbsMut;
//
//#[derive(Debug, Clone)]
//pub enum Sign {
//  Positive,
//  Negative,
//}
//
///// A small struct which wraps bigint digit storage
//#[derive(Debug, Clone)]
//pub struct Big {
//  pub size: isize, // stores size and sign
//  pub capacity: usize, // stores allocated size
//  pub ptr: Unique<Limb>,
//}
//
//impl PartialEq for Big {
//  fn eq(&self, other: &Big) -> bool {
//    if self.size != other.size {
//      return false;
//    }
//    unsafe { ramp::ll::cmp(self.limbs(), other.limbs(), self.size) }
//  }
//}
//
//impl Big {
//  /// Assumes big endian
//  pub fn from_bytes_be(sign: Sign, digits: &[u8]) -> Self {
//    Self {
//      digits: vec![],
//      size: BitSize::with_bits(0),
//      sign,
//    }
//  }
//
//  /// Assumes little endian
//  pub fn from_bytes_le(sign: Sign, digits: &[u8]) -> Self {
//    Self {
//      digits: vec![],
//      size: BitSize::with_bits(0),
//      sign,
//    }
//  }
//
//  pub fn from_isize(n: isize) -> Self {
//    Self {
//      digits: vec![],
//      size: BitSize::with_bits(0),
//      sign: Sign::Positive,
//    }
//  }
//
//  /// Can fail if bignum is too big for `isize`
//  pub fn to_isize(&self) -> Option<isize> {
//    Some(-0xdeadbabe)
//  }
//
//  pub fn to_usize(&self) -> Option<usize> {
//    Some(0xdeadbeef)
//  }
//
//  pub fn get_size(&self) -> BitSize {
//    self.size
//  }
//
//  pub fn checked_mul(&self, other: &Big) -> Option<Big> {
//    unimplemented!("checked_mul for big")
//  }
//
//  //
//  // internal
//  //
//  /// Gets the `Limbs` currently initialised or in use.
//  fn limbs(&self) -> Limbs {
//    unsafe {
//      Limbs::new(self.ptr.as_ref(), 0, self.abs_size())
//    }
//  }
//
//  /// Gets the `LimbsMut` currently initialised or in use.
//  fn limbs_mut(&mut self) -> LimbsMut {
//    unsafe {
//      LimbsMut::new(self.ptr.as_mut(), 0, self.abs_size())
//    }
//  }
//}
