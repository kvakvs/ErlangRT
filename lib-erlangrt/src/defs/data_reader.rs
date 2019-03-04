use crate::defs::{self, BitSize};
use crate::defs::ByteSize;

/// A generic byte reader interface, to be replaced by either byte aligned reader
/// or bit reader, depending whether the data is byte-aligned. In the future
/// there might as well appear another word-aligned reader?
pub trait TDataReader {
  fn get_size(&self) -> BitSize;
  fn read(&self, n: usize) -> u8;
}

/// Points to bit data with possible misaligned bit offset from the beginning.
/// Bit reader is naturally slower than byte-aligned reader.
pub struct BitDataReader {
  data: &'static [u8],
  offset: BitSize,
}

impl BitDataReader {
  pub fn new(data: &'static [u8], offset: BitSize) -> Self {
    Self { data, offset }
  }
}

impl TDataReader for BitDataReader {
  fn get_size(&self) -> BitSize {
    // Length of the data minus the bit offset.
    // The last byte will be possibly incomplete.
    BitSize::with_bits(self.data.len() * defs::BYTE_BITS - self.offset.bit_count)
  }

  #[inline]
  fn read(&self, _n: usize) -> u8 {
    unimplemented!("read for bitdatareader")
  }
}

/// Points to read-only byte-aligned data.
/// Byte reader is naturally faster than the bit reader.
pub struct ByteDataReader {
  data: &'static [u8],
}

impl ByteDataReader {
  pub fn new(data: *const u8, size: usize) -> Self {
    Self {
      data: unsafe { core::slice::from_raw_parts(data, size) },
    }
  }

  /// From a byte reader create one shorter byte reader, offset forward by `size`
  pub unsafe fn set_byte_offset(self, offs: ByteSize) -> Self {
    let new_ptr = self.data.as_ptr().add(offs.bytes());
    let new_len = self.data.len() - offs.bytes();
    Self {
      data: core::slice::from_raw_parts(new_ptr, new_len)
    }
  }
}

impl TDataReader for ByteDataReader {
  #[inline]
  fn get_size(&self) -> BitSize {
    BitSize::with_bytes(self.data.len())
  }

  #[inline]
  fn read(&self, n: usize) -> u8 {
    self.data[n]
  }
}
