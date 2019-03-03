//! Generic binary trait used to access various types of binary

use crate::{
  defs::ByteSize,
  fail::RtResult,
  term::{
    boxed::binary::{bitsize::BitSize, BinaryType},
    lterm::LTerm,
  },
};
use core::fmt;

/// Trait represents any type of binary with generic access functions.
pub trait TBinary {
  fn get_type(&self) -> BinaryType;
  // fn get_byte(&self, index: usize) -> u8;
  fn get_size(&self) -> ByteSize;
  fn get_bit_size(&self) -> BitSize;
  fn get_data(&self) -> *const u8;
  fn get_data_mut(&mut self) -> *mut u8;

  /// Write to the binary from position 0
  fn store(&mut self, data: &[u8]) -> RtResult<()>;

  fn make_term(&self) -> LTerm;

  fn format(&self, f: &mut fmt::Formatter) -> fmt::Result;
}
