//!
//! All low level (LTerm) values have a primary tag to define basic type.
//! Bit composition is - `.... .... .... ..PP`, where `PP` is the primary tag.
//!
//! Max value for such term is 64-2=62, or 32-2=30 bits. This value is large
//! enough to hold a platform pointer, using the fact that lowest 2 bits of a
//! word aligned pointer are always `00`. On some platforms extra data may be
//! stored in pointer high bits, but this would be strongly system specific.
//!
use defs;
use defs::Word;

use std::mem::transmute;
use bit_field::BitField;

/// Bit position for the primary tag
pub const PRIM_TAG_FIRST: u8 = 0;
pub const PRIM_TAG_LAST: u8 = 2;

/// Bit position for the value after the primary tag
pub const PRIM_VALUE_FIRST: u8 = PRIM_TAG_LAST;
pub const PRIM_VALUE_LAST: u8 = defs::WORD_BITS as u8;

#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
// First two bits in any term define its major type
pub enum Tag {
  // points to something special on heap
  Header = 0,
  // points to a cons cell on heap
  Cons = 1,
  // is some value which fits into a Word
  Immediate = 2,
  // points to something on heap
  Box = 3,
}

/// Get the primary tag bits and transmute into primary::Tag
pub fn get(val: Word) -> Tag {
  let tag_bits = val.get_bits(PRIM_TAG_FIRST..PRIM_TAG_LAST);
  unsafe { transmute::<usize, Tag>(tag_bits) }
}

/// Zero the primary tag bits and assume the rest is a valid pointer
pub fn pointer(val0: Word) -> *const Word {
  let mut val = val0;
  let untagged = val.set_bits(PRIM_TAG_FIRST..PRIM_TAG_LAST, 0);
  untagged as *const Word
}
