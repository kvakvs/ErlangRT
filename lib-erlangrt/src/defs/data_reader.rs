use crate::defs::{self, BitSize, SizeBytes};

/// A generic byte reader interface, to be replaced by either byte aligned reader
/// or bit reader, depending whether the data is byte-aligned. In the future
/// there might as well appear another word-aligned reader?
pub trait TDataReader {
  fn get_bit_size(&self) -> BitSize;
  fn read(&self, n: usize) -> u8;
}

/// Points to bit data with possible misaligned bit offset from the beginning.
/// Bit reader is naturally slower than byte-aligned reader.
pub struct BitReader {
  data: &'static [u8],
  offset: BitSize,
}

impl BitReader {
  #[allow(dead_code)]
  pub fn new(data: &'static [u8], offset: BitSize) -> Self {
    Self { data, offset }
  }

  pub fn add_bit_offset(&self, offs: BitSize) -> Self {
    Self {
      data: self.data,
      offset: self.offset + offs,
    }
  }
}

impl TDataReader for BitReader {
  fn get_bit_size(&self) -> BitSize {
    // Length of the data minus the bit offset.
    // The last byte will be possibly incomplete.
    BitSize::with_bits(self.data.len() * defs::BYTE_BITS - self.offset.bits)
  }

  #[inline]
  fn read(&self, _n: usize) -> u8 {
    unimplemented!("read for bitdatareader")
  }
}

/// Points to read-only byte-aligned data.
/// Byte reader is naturally faster than the bit reader.
pub struct ByteReader {
  data: &'static [u8],
}

impl ByteReader {
  pub fn new(data: *const u8, size: usize) -> Self {
    // println!("Creating data reader size={}", size);
    Self {
      data: unsafe { core::slice::from_raw_parts(data, size) },
    }
  }

  /// From a byte reader create one shorter byte reader, offset forward by `size`
  pub unsafe fn set_offset_and_size(self, offs: SizeBytes, size: SizeBytes) -> Self {
    // println!("Setting data reader offset={}", offs);
    let new_ptr = self.data.as_ptr().add(offs.bytes());
    let new_len = core::cmp::min(self.data.len() - offs.bytes(), size.bytes());
    Self {
      data: core::slice::from_raw_parts(new_ptr, new_len),
    }
  }
}

impl TDataReader for ByteReader {
  #[inline]
  fn get_bit_size(&self) -> BitSize {
    BitSize::with_bytes(self.data.len())
  }

  #[inline]
  fn read(&self, n: usize) -> u8 {
    self.data[n]
  }
}
