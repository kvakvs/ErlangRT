use crate::defs::{self, BitSize};
use core::fmt;
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct WordSize {
  pub words: usize,
}

#[allow(dead_code)]
impl WordSize {
  pub const fn one() -> Self {
    Self { words: 1 }
  }
  pub const fn zero() -> Self {
    Self { words: 0 }
  }

  pub const fn new(words: usize) -> Self {
    Self { words }
  }

  pub const fn add(self, n: usize) -> Self {
    Self {
      words: self.words + n,
    }
  }

  pub const fn bytes(self) -> usize {
    self.words * defs::WORD_BYTES
  }
}

impl Add for WordSize {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      words: self.words + other.words,
    }
  }
}

impl Sub for WordSize {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      words: self.words - other.words,
    }
  }
}

impl fmt::Display for WordSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}W", self.words)
  }
}

#[derive(Debug, Copy, Clone)]
pub struct ByteSize {
  pub bytes: usize,
}

#[allow(dead_code)]
impl ByteSize {
  #[inline]
  pub const fn new(bytes: usize) -> ByteSize {
    Self { bytes }
  }
  pub const fn one() -> Self {
    Self { bytes: 1 }
  }
  pub const fn zero() -> Self {
    Self { bytes: 0 }
  }

  pub fn add(&mut self, n: usize) {
    self.bytes += n
  }

  // TODO: impl Add trait
  pub fn add_bytesize(&mut self, other: ByteSize) {
    self.bytes = self.bytes + other.bytes
  }

  #[inline]
  pub const fn bytes(self) -> usize {
    self.bytes
  }

  #[inline]
  pub const fn get_words_rounded_down(self) -> WordSize {
    WordSize::new(self.bytes / defs::WORD_BYTES)
  }

  #[inline]
  pub const fn get_words_rounded_up(self) -> WordSize {
    WordSize::new((self.bytes + defs::WORD_BYTES - 1) / defs::WORD_BYTES)
  }

  #[inline]
  pub const fn get_bits(self) -> BitSize {
    BitSize::with_unit_const(self.bytes, defs::WORD_BYTES)
  }
}

impl fmt::Display for ByteSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}B", self.bytes)
  }
}
