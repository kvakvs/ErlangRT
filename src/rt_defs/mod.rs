//!
//! Helper module defines types used everywhere in the VM runtime
//!
pub mod stack;
pub mod term_builder;

//extern crate num;


//------------------------------------------------------------------------------

use std::{isize, usize};

pub type Word = usize;
pub type SWord = isize;

#[derive(Copy, Clone)]
pub struct WordSize(usize);

impl WordSize {
  #[inline]
  pub const fn new(words: usize) -> WordSize {
    WordSize(words)
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

#[derive(Copy, Clone)]
pub struct ByteSize(usize);

impl ByteSize {
  #[inline]
  pub const fn new(bytes: usize) -> ByteSize {
    ByteSize(bytes)
  }

  #[inline]
  pub const fn bytes(self) -> usize {
    self.0
  }

  #[inline]
  pub const fn words_rounded_down(self) -> usize {
    self.0 / WORD_BYTES
  }

  #[inline]
  pub const fn words_rounded_up(self) -> usize {
    (self.0 + WORD_BYTES - 1) / WORD_BYTES
  }
}

pub type Arity = usize;

//pub use term::immediate::SMALL_BITS;

#[cfg(target_pointer_width = "32")]
pub const WORD_BITS: Word = 32;

#[cfg(target_pointer_width = "64")]
pub const WORD_BITS: Word = 64;

/// This bit is set on boxed values which are CP pointers
pub const HIGHEST_BIT_CP: Word = 1 << (WORD_BITS - 1);

pub const WORD_BYTES: Word = WORD_BITS / 8;

pub const MAX_XREGS: Word = 256;
pub const MAX_FPREGS: Word = 32;


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum ExceptionType {
  Throw,
  Error,
  Exit,
}


/// For n bytes calculate how many words are required to store this
#[inline]
pub const fn storage_bytes_to_words(n: Word) -> WordSize {
  WordSize::new((n + WORD_BYTES - 1) / WORD_BYTES)
}
