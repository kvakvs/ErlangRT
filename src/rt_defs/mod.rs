//!
//! Helper module defines types used everywhere in the VM runtime
//!
pub mod heap;
pub mod term_builder;
pub mod stack;

//extern crate num;


//------------------------------------------------------------------------------

use std::{usize, isize};

pub type Word = usize;
pub type SWord = isize;

/// Replace with appropriate f32 or fixed/compact for embedded platform
pub type Float = f64;

pub type Arity = usize;

//pub use term::immediate::SMALL_BITS;

#[cfg(target_pointer_width = "32")]
pub const WORD_BITS: Word = 32;

#[cfg(target_pointer_width = "64")]
pub const WORD_BITS: Word = 64;

pub const WORD_BYTES: Word = WORD_BITS / 8;

/// Max value for a positive small integer packed into immediate2 low level
/// Term. Assume word size minus 4 bits for imm1 tag and 1 for sign
pub const MAX_POS_SMALL: SWord = isize::MAX / 16;
pub const MAX_UNSIGNED_SMALL: Word = (isize::MAX / 16) as Word;
pub const MIN_NEG_SMALL: SWord = isize::MIN / 16;

pub const MAX_XREGS: Word = 256;
pub const MAX_FPREGS: Word = 32;


/// For CP values the highest bit is set. CP values never appear on heap, or
/// in registers, only in code or stack.
pub const TAG_CP: Word = 1usize << (WORD_BITS-1);


//pub fn unsafe_sword_to_word(n: SWord) -> Word {
//  unsafe { transmute::<isize, usize> (n) }
//}

//pub fn unsafe_word_to_sword(n: Word) -> SWord {
//  unsafe { transmute::<usize, isize> (n) }
//}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum ExceptionType {
  Throw,
  Error,
  Exit,
}
