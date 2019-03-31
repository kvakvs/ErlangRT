//! Generic binary trait used to access various types of binary

use crate::{
  defs::{data_reader::BitDataReader, BitSize, ByteDataReader, ByteSize},
  fail::RtResult,
  term::{boxed::binary::BinaryType, value::Term},
};

pub enum SizeOrAll {
  Bits(BitSize),
  All,
}

/// Trait represents any type of binary with generic access functions.
pub trait TBinary {
  fn get_type(&self) -> BinaryType;
  // fn get_byte(&self, index: usize) -> u8;
  fn get_byte_size(&self) -> ByteSize;
  fn get_bit_size(&self) -> BitSize;

  /// Get slice for read access to the bytes.
  /// The call may fail for binary slices, in that case `get_bit_reader` should be used (slower).
  fn get_byte_reader(&self) -> Option<ByteDataReader>;

  /// Get slice for read-write access to the bytes
  unsafe fn get_data_mut(&mut self) -> &mut [u8];

  /// Used for readonly bit offsets in binary slices. Will only be called if
  /// get_data has returned NULL.
  fn get_bit_reader(&self) -> BitDataReader;

  /// Write to the binary from position 0
  fn store(&mut self, data: &[u8]) -> RtResult<()>;

  fn make_term(&self) -> Term;

  // Writing support
  //

  /// Writes integer `val` (small or big) of `size`, the offset has to be increased
  /// by `size` by the caller.
  unsafe fn put_integer(
    &mut self,
    val: Term,
    size: BitSize,
    offset: BitSize,
    flags: crate::beam::opcodes::BsFlags,
  ) -> RtResult<()>;

  /// Writes another binary `val` into this binary, with a bit offset.
  /// Arg `src`: TBinary serving as data source,
  /// Arg `dst_offset`: Bit offset in the destination where the bits go
  /// Arg `size`: How many bits to copy, or `All`
  /// Returns: Bit count copied
  unsafe fn put_binary(
    &mut self,
    src: *const TBinary,
    dst_offset: BitSize,
    size: SizeOrAll,
    flags: crate::beam::opcodes::BsFlags,
  ) -> RtResult<BitSize>;
}
