use crate::defs::WORD_BYTES;
use core::fmt;

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
    self.0 * WORD_BYTES
  }
}

impl fmt::Display for WordSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} words", self.0)
  }
}

#[derive(Copy, Clone)]
pub struct ByteSize(usize);

#[allow(dead_code)]
impl ByteSize {
  #[inline]
  pub const fn new(bytes: usize) -> ByteSize {
    ByteSize(bytes)
  }

  pub fn add(self, n: usize) -> ByteSize {
    ByteSize(self.0 + n)
  }

  #[inline]
  pub const fn bytes(self) -> usize {
    self.0
  }

  #[inline]
  pub const fn words_rounded_down(self) -> WordSize {
    WordSize::new(self.0 / WORD_BYTES)
  }

  #[inline]
  pub const fn words_rounded_up(self) -> WordSize {
    WordSize::new((self.0 + WORD_BYTES - 1) / WORD_BYTES)
  }
}

impl fmt::Display for ByteSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} bytes", self.0)
  }
}
