use defs::Word;

use std::mem::transmute;


pub const PRIM_TAG_BITS: Word = 2;
pub const PRIM_TAG_MASK: Word = (1 << PRIM_TAG_BITS) - 1;


#[derive(Eq, PartialEq)]
// First two bits in any term define its major type
pub enum Tag {
  // points to something special on heap
  Header = 0,
  // points to a cons cell on heap
  Cons = 1,
  // points to something on heap
  Box = 2,
  // is some value which fits into a Word
  Immediate = 3,
}

pub fn from_word(w: Word) -> Tag {
  unsafe { transmute((w & PRIM_TAG_MASK) as u8) }
}

pub fn set_primary_immediate(w: Word) -> Word {
  (w & (!PRIM_TAG_MASK)) | (Tag::Immediate as Word)
}

pub fn is_primary_tag(val: Word, tag: Tag) -> bool {
  val & PRIM_TAG_MASK == tag as Word
}

/// Trim value to the primary tag bits and transmute into primary::Tag
pub fn primary_tag(val: Word) -> Tag {
  unsafe { transmute::<u8, Tag>((val & PRIM_TAG_MASK) as u8) }
}

/// Zero the primary tag bits and assume the rest is a valid pointer
pub fn pointer(val: Word) -> *const Word {
  (val & (!PRIM_TAG_MASK)) as *const Word
}
