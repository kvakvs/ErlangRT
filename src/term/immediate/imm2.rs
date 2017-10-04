use term::primary;
use defs;
use defs::Word;
use term::immediate::imm1::*;

use std::mem::transmute;

//--- Immediate 2 precomposed values ---

#[repr(u8)]
pub enum Immediate2 {
  Atom = 0,
  Catch = 1,
  /// Special includes unique values like NIL, NONVALUE
  Special = 2,
  /// not used
  Immed3 = 3,
}
/// Max value for the Immediate2 enum (for assertions).
pub const IMMEDIATE2_MAX: Word = 3;

#[repr(u8)]
pub enum Immediate2Special {
  Nil = 1,
  NonValue = 2,
}

pub const IMM2_SIZE: Word = 2;
/// A mask to apply to a value shifted right by IMM2_TAG_SHIFT to get imm2 tag
pub const IMM2_TAG_MASK: Word = (1 << IMM2_SIZE) - 1;
/// How much to shift tag to place it into Immediate2 tag bits
pub const IMM2_TAG_SHIFT: Word = IMM1_VALUE_SHIFT;
/// How much to shift a value to place it after Immediate2 tag
pub const IMM2_VALUE_SHIFT: Word = IMM1_TAG_SHIFT + IMM1_SIZE + IMM2_SIZE;
pub const IMM2_MASK: Word = (1 << IMM2_VALUE_SHIFT) - 1;

/// Cut away the value to be able to compare with raw prefixes
#[inline]
pub fn get_imm2_prefix(val: Word) -> Word {
  val & IMM2_MASK
}

/// Trim to have only immediate2 bits and return them as an convenient enum.
#[inline]
pub fn get_imm2_tag(val: Word) -> Immediate2 {
  let t: Word = (val >> IMM2_TAG_SHIFT) & IMM2_TAG_MASK;
  assert!(t <= IMMEDIATE2_MAX);
  unsafe { transmute(t as u8) }
}

/// Remove tag bits from imm2 value by shifting it right
#[inline]
pub fn imm2_value(val: Word) -> Word {
  assert!(is_immediate2(val));
  val >> IMM2_VALUE_SHIFT
}

/// Precomposed bits for immediate2 values
pub const IMM2_PREFIX: Word = IMM1_PREFIX
    | ((Immediate1::Immed2 as Word) << primary::PRIM_TAG_BITS);

/// Precomposed bits for atom imm2
pub const IMM2_ATOM_PREFIX: Word = IMM2_PREFIX
    | ((Immediate2::Atom as Word) << IMM1_VALUE_SHIFT);

//--- Imm2 values tagged special ---

/// Special Primary tag+Immed1+Immed2 bits precomposed
pub const IMM2_SPECIAL_PREFIX: Word = IMM1_PREFIX
    | ((Immediate2::Special as Word) << IMM2_TAG_SHIFT);

/// Precomposed bits for NIL constant
pub const IMM2_SPECIAL_NIL_RAW: Word = IMM2_SPECIAL_PREFIX
    | ((Immediate2Special::Nil as Word) << IMM2_VALUE_SHIFT);

/// Precomposed bits for NON_VALUE constant
pub const IMM2_SPECIAL_NONVALUE_RAW: Word = IMM2_SPECIAL_PREFIX
    | ((Immediate2Special::NonValue as Word) << IMM2_VALUE_SHIFT);



/// Given value (to be shifted) and RAW_* preset bits, compose them together for imm2
#[inline]
pub fn create_imm2(val: Word, raw_preset: Word) -> Word {
  (val << IMM2_VALUE_SHIFT) | raw_preset
}

pub fn is_immediate2(val: Word) -> bool { val & IMM2_MASK == IMM2_PREFIX }
