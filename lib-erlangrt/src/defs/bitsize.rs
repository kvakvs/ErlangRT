use core::fmt;

use crate::defs::{self, ByteSize, WordSize};
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BitSize {
  pub bits: usize,
}

impl fmt::Display for BitSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} bits", self.bits)
  }
}

impl fmt::Debug for BitSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} bits", self.bits)
  }
}

impl BitSize {
  pub const fn zero() -> Self {
    Self { bits: 0 }
  }

  #[inline]
  pub const fn is_empty(self) -> bool {
    self.bits == 0
  }

  #[inline]
  pub const fn with_bits(bit_count: usize) -> Self {
    Self { bits: bit_count }
  }

  #[allow(dead_code)]
  #[inline]
  pub const fn with_bytesize(size: ByteSize) -> Self {
    Self {
      bits: size.bytes() * defs::BYTE_BITS,
    }
  }

  #[inline]
  pub const fn with_bytes(size: usize) -> Self {
    Self {
      bits: size * defs::BYTE_BITS,
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
      bits: size * unit,
    }
  }

  pub const fn with_unit_const(size: usize, unit: usize) -> Self {
    Self {
      bits: size * unit,
    }
  }

  /// Returns how many hanging bits are there. Value range is 0..7, 0 means
  /// all bytes are whole and no hanging bits.
  #[inline]
  pub const fn get_last_byte_bits(&self) -> usize {
    self.bits & defs::BYTE_SHIFT_RANGE_MASK
  }

  #[inline]
  pub const fn get_byte_size_rounded_down(&self) -> ByteSize {
    ByteSize::new(self.bits >> defs::BYTE_POF2_BITS)
  }

  #[inline]
  pub const fn get_bytes_rounded_down(&self) -> usize {
    self.bits >> defs::BYTE_POF2_BITS
  }

  pub const fn get_byte_size_rounded_up(self) -> ByteSize {
    ByteSize::new((self.bits + defs::BYTE_SHIFT_RANGE_MASK) / defs::BYTE_BITS)
  }

  #[inline]
  pub const fn get_words_rounded_up(self) -> WordSize {
    let b = self.get_byte_size_rounded_down().bytes();
    WordSize::new((b + defs::WORD_BYTES - 1) / defs::WORD_BYTES)
  }
}

impl Add for BitSize {
  type Output = BitSize;

  fn add(self, other: BitSize) -> BitSize {
    BitSize {
      bits: self.bits + other.bits,
    }
  }
}

impl Sub for BitSize {
  type Output = BitSize;

  fn sub(self, other: BitSize) -> BitSize {
    BitSize {
      bits: self.bits - other.bits,
    }
  }
}
