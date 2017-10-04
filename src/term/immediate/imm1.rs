use term::primary;
use defs;
use defs::Word;

use std::mem::transmute;

//
// Premade bit combinations and constants for Immediate1 values
//

pub const IMM1_SIZE: Word = 2;
/// A mask to apply to a value shifted right by IMM1_TAG_SHIFT to get imm1 tag
pub const IMM1_TAG_MASK: Word = (1 << IMM1_SIZE) - 1;
/// How much to shift tag to place it into Immediate1 tag bits
pub const IMM1_TAG_SHIFT: Word = primary::PRIM_TAG_BITS;
/// How much to shift a value to place it after Immediate2 tag
pub const IMM1_VALUE_SHIFT: Word = IMM1_TAG_SHIFT + IMM1_SIZE;
pub const IMM1_VALUE_MASK: Word = (1 << IMM1_VALUE_SHIFT) - 1;

#[repr(u8)]
pub enum Immediate1 {
  Pid = 0,
  Port = 1,
  Small = 2,
  Immed2 = 3,
}
/// Max value for the Immediate1 enum (for assertions).
pub const IMMEDIATE1_MAX: Word = 3;

/// Cut away the value to be able to compare with raw prefixes
#[inline]
pub fn get_imm1_prefix(val: Word) -> Word {
  val & IMM1_VALUE_MASK
}

/// Trim the immediate1 bits and return them as an convenient enum.
#[inline]
pub fn get_imm1_tag(val: Word) -> Immediate1 {
  let t: Word = (val >> IMM1_TAG_SHIFT) & IMM1_TAG_MASK;
  assert!(t <= IMMEDIATE1_MAX);
  unsafe { transmute(t as u8) }
}

/// Remove tag bits from imm1 value by shifting it right
#[inline]
pub fn imm1_value(val: Word) -> Word {
  assert!(is_immediate1(val));
  val >> IMM1_VALUE_SHIFT
}

/// Special Primary tag+Immed1 precomposed
pub const IMM1_PREFIX: Word = primary::Tag::Immediate as Word;

/// Precomposed bits for pid imm1
pub const IMM1_PID_PREFIX: Word = IMM1_PREFIX
    | ((Immediate1::Pid as Word) << primary::PRIM_TAG_BITS);

pub const IMM1_SMALL_PREFIX: Word = IMM1_PREFIX
    | ((Immediate1::Small as Word) << primary::PRIM_TAG_BITS);


pub fn is_immediate1(val: Word) -> bool { val & IMM1_TAG_MASK == IMM1_PREFIX }


/// Given a value (to be shifted) and RAW_* preset bits, compose them together for imm1
#[inline]
pub fn create_imm1(val: Word, raw_preset: Word) -> Word {
  (val << IMM1_VALUE_SHIFT) | raw_preset
}