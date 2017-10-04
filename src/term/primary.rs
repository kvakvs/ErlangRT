use defs::Word;

use std::mem::transmute;


pub const SIZE: Word = 2;
pub const MASK: Word = (1 << SIZE) - 1;


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
  unsafe { transmute((w & MASK) as u8) }
}

pub fn set_primary_immediate(w: Word) -> Word {
  (w & (!MASK)) | (Tag::Immediate as Word)
}

pub fn is_primary_tag(val: Word, tag: Tag) -> bool {
  val & MASK == tag as Word
}

/// Trim value to the primary tag bits and transmute into primary::Tag
pub fn primary_tag(val: Word) -> Tag {
  unsafe { transmute::<u8, Tag>((val & MASK) as u8) }
}

/// Zero the primary tag bits and assume the rest is a valid pointer
pub fn pointer(val: Word) -> *const Word {
  (val & (!MASK)) as *const Word
}
