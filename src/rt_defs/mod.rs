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

/// This bit is set on boxed values which are CP pointers
pub const HIGHEST_BIT_CP: Word = 1 << (WORD_BITS - 1);

pub const WORD_BYTES: Word = WORD_BITS / 8;

/// Max value for a positive small integer packed into immediate2 low level
/// Term. Assume word size minus 4 bits for imm1 tag and 1 for sign
pub const SMALLEST_SMALL: SWord = isize::MIN >> TERM_TAG_BITS;
pub const LARGEST_SMALL: SWord = isize::MAX >> TERM_TAG_BITS;

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
  Cons,
  // From here and below, values fit into a single word
  Small,
  Atom,
  LocalPid,
  LocalPort,
  Special,
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
  Const, // special constants such as NIL, empty tuple, binary etc
  RegX,
  RegY,
  RegFP,
  Opcode, // decorates opcodes for easier code walking
}

pub enum SpecialConst {
  Nil,
  EmptyTuple,
  EmptyList,
  EmptyBinary,
}


/// For n bytes calculate how many words are required to store this
pub fn storage_bytes_to_words(n: Word) -> Word {
  (n + WORD_BYTES - 1) / WORD_BYTES
}
