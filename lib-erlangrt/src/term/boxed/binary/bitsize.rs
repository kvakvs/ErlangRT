use crate::defs::{self, ByteSize, WordSize};
use core::fmt;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BitSize {
  pub bit_count: usize,
}

impl fmt::Display for BitSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} bits", self.bit_count)
  }
}

impl BitSize {
  #[allow(dead_code)]
  pub fn with_bytesize(size: ByteSize) -> Self {
    Self {
      bit_count: size.bytes() * defs::BYTE_BITS,
    }
  }

  pub fn with_unit(size: usize, unit: usize) -> Self {
    assert!(
      size < 1usize << (core::mem::size_of::<usize>() * defs::BYTE_BITS - unit),
      "Bitsize: the size={} * unit={} does not fit usize datatype",
      size,
      unit
    );
    Self {
      bit_count: size * unit,
    }
  }

  #[allow(dead_code)]
  #[inline]
  pub const fn get_last_byte_bits(&self) -> usize {
    self.bit_count & defs::BYTE_SHIFT
  }

  #[inline]
  pub const fn get_full_bytes(&self) -> usize {
    self.bit_count >> defs::BYTE_SHIFT
  }

  #[inline]
  pub const fn words_rounded_up(self) -> WordSize {
    WordSize::new((self.get_full_bytes() + defs::WORD_BYTES - 1) / defs::WORD_BYTES)
  }
}
