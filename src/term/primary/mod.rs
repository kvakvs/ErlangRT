//!
//! All low level (LTerm) values have a primary tag to define basic type.
//! Bit composition is - `.... .... .... ..PP`, where `PP` is the primary tag.
//!
//! Max value for such term is 64-2=62, or 32-2=30 bits. This value is large
//! enough to hold a platform pointer, using the fact that lowest 2 bits of a
//! word aligned pointer are always `00`. On some platforms extra data may be
//! stored in pointer high bits, but this would be strongly system specific.
//!
pub mod header;

use defs;
use defs::Word;

use std::mem::transmute;
use bit_field::BitField;

/// Bit position for the primary tag.
pub const PRIM_TAG_FIRST: u8 = 0;
pub const PRIM_TAG_LAST: u8 = 2;

/// Bit position for the value after the primary tag.
pub const PRIM_VALUE_FIRST: u8 = PRIM_TAG_LAST;
pub const PRIM_VALUE_LAST: u8 = defs::WORD_BITS as u8;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
#[allow(dead_code)]
/// First two bits in any term define its major type
pub enum Tag {
  // Marks something special on heap, never appears as a LTerm value in
  // registers, is always on heap.
  Header = 0,
  // points to a cons cell on heap
  Cons = 1,
  // is some value which fits into a Word
  Immediate = 2,
  // points to something on heap
  Box = 3,
}


/// Get the primary tag bits and transmute into primary::Tag
#[inline]
pub fn get_tag(val: Word) -> Tag {
  let tag_bits = val.get_bits(PRIM_TAG_FIRST..PRIM_TAG_LAST);
  unsafe { transmute::<usize, Tag>(tag_bits) }
}


#[inline]
pub fn get_value(val: Word) -> Word {
  val.get_bits(PRIM_VALUE_FIRST..PRIM_VALUE_LAST)
}


/// Zero the primary tag bits and assume the rest is a valid const pointer.
#[inline]
pub fn pointer(val0: Word) -> *const Word {
  let mut val = val0;
  let untagged = val.set_bits(PRIM_TAG_FIRST..PRIM_TAG_LAST, 0);
  untagged as *const Word
}


/// Zero the primary tag bits and assume the rest is a valid mutable pointer.
#[inline]
pub fn pointer_mut(val0: Word) -> *mut Word {
  let mut val = val0;
  let untagged = val.set_bits(PRIM_TAG_FIRST..PRIM_TAG_LAST, 0);
  untagged as *mut Word
}


#[inline]
pub fn make_box_raw(ptr: *const Word) -> Word {
  let i = ptr as Word;
  debug_assert!(i.get_bits(PRIM_TAG_FIRST..PRIM_TAG_LAST) == 0);
  i | (Tag::Box as Word)
}


#[inline]
pub fn make_cons_raw(ptr: *const Word) -> Word {
  let i = ptr as Word;
  debug_assert!(i.get_bits(PRIM_TAG_FIRST..PRIM_TAG_LAST) == 0);
  i | (Tag::Cons as Word)
}
