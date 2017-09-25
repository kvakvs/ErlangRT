use types::Word;

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
