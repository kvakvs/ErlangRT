//! Helper module defines types used everywhere in the VM runtime
//!
pub mod sizes;
pub use self::sizes::*;

use std::{isize, usize};

pub type Word = usize;
pub type SWord = isize;
pub type Arity = usize;

// pub use term::immediate::SMALL_BITS;

#[cfg(target_pointer_width = "32")]
pub const WORD_BITS: Word = 32;

#[cfg(target_pointer_width = "64")]
pub const WORD_BITS: Word = 64;

/// This bit is set on boxed values which are CP pointers
pub const HIGHEST_BIT_CP: Word = 1 << (WORD_BITS - 1);

pub const WORD_BYTES: Word = WORD_BITS / 8;

pub const MAX_XREGS: Word = 256;
pub const MAX_FPREGS: Word = 8;

/// How many function-calls/heavier opcodes we process before the process will
/// be scheduled out and give the way to other processes in the queue.
pub const DEFAULT_REDUCTIONS: isize = 200;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum ExceptionType {
  Throw,
  Error,
  Exit,
}

// / For n bytes calculate how many words are required to store this
//#[inline]
// pub const fn storage_bytes_to_words(n: Word) -> WordSize {
//  WordSize::new((n + WORD_BYTES - 1) / WORD_BYTES)
//}
