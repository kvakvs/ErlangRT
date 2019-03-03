//! Generic binary trait used to access various types of binary

use crate::{
  defs::{BitSize, ByteSize},
  fail::RtResult,
  term::{boxed::binary::BinaryType, lterm::LTerm},
};
use crate::defs::bitptr::BitDataPointer;

/// Trait represents any type of binary with generic access functions.
pub trait TBinary {
  fn get_type(&self) -> BinaryType;
  // fn get_byte(&self, index: usize) -> u8;
  fn get_byte_size(&self) -> ByteSize;
  fn get_bit_size(&self) -> BitSize;

  /// Get slice for read access to the bytes
  unsafe fn get_data(&self) -> &[u8];

  /// Get slice for read-write access to the bytes
  unsafe fn get_data_mut(&mut self) -> &mut [u8];

  /// Used for readonly bit offsets in binary slices. Will only be called if
  /// get_data has returned NULL.
  fn get_data_bitptr(&self) -> BitDataPointer;

  /// Write to the binary from position 0
  fn store(&mut self, data: &[u8]) -> RtResult<()>;

  fn make_term(&self) -> LTerm;
}
