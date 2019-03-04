use core::fmt;

use crate::defs::{self, ByteSize, WordSize};
use std::ops::Sub;

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
  #[inline]
  pub const fn with_bits(bit_count: usize) -> Self {
    Self { bit_count }
  }

  #[allow(dead_code)]
  #[inline]
  pub const fn with_bytesize(size: ByteSize) -> Self {
    Self {
      bit_count: size.bytes() * defs::BYTE_BITS,
    }
  }

  #[inline]
  pub const fn with_bytes(size: usize) -> Self {
    Self {
      bit_count: size * defs::BYTE_BITS,
    }
  }

  pub fn with_unit(size: usize, unit: usize) -> Self {
    if cfg!(debug_assertions) {
      assert!(
        size < core::usize::MAX / unit,
        "Bitsize: the size={} * unit={} does not fit usize datatype",
        size,
        unit
      );
    }
    Self {
      bit_count: size * unit,
    }
  }

  pub const fn with_unit_const(size: usize, unit: usize) -> Self {
    Self {
      bit_count: size * unit,
    }
  }

  #[allow(dead_code)]
  #[inline]
  pub const fn get_last_byte_bits(&self) -> usize {
    self.bit_count & (defs::BYTE_BITS - 1)
  }

  #[inline]
  pub const fn get_byte_size_rounded_down(&self) -> ByteSize {
    ByteSize::new(self.bit_count >> defs::BYTE_POF2_BITS)
  }

  pub const fn get_byte_size_rounded_up(self) -> ByteSize {
    ByteSize::new((self.bit_count + defs::BYTE_BITS - 1) / defs::BYTE_BITS)
  }

  #[inline]
  pub const fn get_words_rounded_up(self) -> WordSize {
    let b = self.get_byte_size_rounded_down().bytes();
    WordSize::new((b + defs::WORD_BYTES - 1) / defs::WORD_BYTES)
  }
}

impl Sub for BitSize {
  type Output = BitSize;

  fn sub(self, other: BitSize) -> BitSize {
    BitSize {
      bit_count: self.bit_count - other.bit_count,
    }
  }
}
