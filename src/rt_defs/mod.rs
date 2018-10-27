//!
//! Helper module defines types used everywhere in the VM runtime
//!
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


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum ExceptionType {
  Throw,
  Error,
  Exit,
}

//
// Structure of term:
// [ Value or a pointer ] [ TAG_* value 3 bits ]
//

pub const TERM_TAG_BITS: Word = 3;
pub const TERM_TAG_MASK: Word = (1 << TERM_TAG_BITS) - 1;

pub enum TermTag {
  Boxed,
  Header,
  Small,
  Atom,
  LocalPid,
  LocalPort,
  CP,
  Special
}

//pub const TAG_BOXED: Word = 0; // contains pointer to a value on heap
//pub const TAG_HEADER: Word = 1; // Marks a stored boxed value on heap
//
//pub const TAG_SMALL: Word = 2; // contains a small integer
//pub const TAG_ATOM: Word = 3; // contains atom index
//pub const TAG_LOCAL_PID: Word = 4; // contains process index
//pub const TAG_CP: Word = 5; // contains a pointer to code
//// Special contains extra values like empty binary, NIL etc
//pub const TAG_SPECIAL: Word = 7; // contains something else (below)

//
// Structure of SPECIAL values,
// they are plethora of term types requiring fewer bits or useful in other ways
// [ special value ] [ VAL_SPECIAL_... 3 bits ] [ TAG_SPECIAL 3 bits ]
//
pub const TERM_SPECIAL_TAG_BITS: Word = 3;
pub const TERM_SPECIAL_TAG_MASK: Word = (1 << TERM_SPECIAL_TAG_BITS) - 1;

pub enum SpecialTag {
  EmptyList,
  EmptyTuple,
  EmptyBinary,
  RegX,
  RegY,
  RegFP,
}

//pub const VAL_SPECIAL_EMPTY_LIST: Word = 1;
//pub const VAL_SPECIAL_EMPTY_TUPLE: Word = 2;
//pub const VAL_SPECIAL_EMPTY_BINARY: Word = 3;
//pub const VAL_SPECIAL_REGX: Word = 4;
//pub const VAL_SPECIAL_REGY: Word = 5;
//pub const VAL_SPECIAL_REGFP: Word = 6;
