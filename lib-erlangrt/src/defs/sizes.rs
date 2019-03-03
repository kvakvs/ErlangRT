use crate::defs;
use core::fmt;
use crate::defs::BitSize;

#[derive(Copy, Clone)]
pub struct WordSize(usize);

#[allow(dead_code)]
impl WordSize {
  #[inline]
  pub const fn new(words: usize) -> WordSize {
    WordSize(words)
  }

  pub const fn add(self, n: usize) -> WordSize {
    WordSize(self.0 + n)
  }

  #[inline]
  pub const fn words(self) -> usize {
    self.0
  }

  #[inline]
  pub const fn bytes(self) -> usize {
    self.0 * defs::WORD_BYTES
  }
}

impl fmt::Display for WordSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} words", self.0)
  }
}

#[derive(Debug, Copy, Clone)]
pub struct ByteSize(usize);

#[allow(dead_code)]
impl ByteSize {
  #[inline]
  pub const fn new(bytes: usize) -> ByteSize {
    ByteSize(bytes)
  }

  pub fn add(&mut self, n: usize) {
    self.0 += n
  }

  pub fn add_bytesize(&mut self, other: ByteSize) {
    self.0 = self.0 + other.0
  }

  #[inline]
  pub const fn bytes(self) -> usize {
    self.0
  }

  #[inline]
  pub const fn get_words_rounded_down(self) -> WordSize {
    WordSize::new(self.0 / defs::WORD_BYTES)
  }

  #[inline]
  pub const fn get_words_rounded_up(self) -> WordSize {
    WordSize::new((self.0 + defs::WORD_BYTES - 1) / defs::WORD_BYTES)
  }

  #[inline]
  pub const fn get_bits(self) -> BitSize {
    BitSize::with_unit_const(self.0, 1)
  }
}

impl fmt::Display for ByteSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} bytes", self.0)
  }
}
