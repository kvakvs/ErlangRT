//!
//! Immediate2 values used to represent smaller special term types.
//! Bit composition is - `.... .... ..bb aaPP`, where `PP` is primary tag,
//! `aa` is imm1 tag, and `bb` is imm2 tag
//!
//! Max value for imm2 is 64-6=58, or 32-6=26 bits.
//!
use defs;
use defs::Word;
use term::immediate::imm1::*;

use std::mem::transmute;
use bit_field::BitField;

/// Bit position for imm1 tag
pub const IMM2_TAG_FIRST: u8 = 4;
pub const IMM2_TAG_LAST: u8 = 6;

/// Bit position for the value after imm1 tag
pub const IMM2_VALUE_FIRST: u8 = IMM2_TAG_LAST;
pub const IMM2_VALUE_LAST: u8 = defs::WORD_BITS as u8;

#[repr(usize)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Immediate2 {
  Atom = 0,
  Catch = 1,
  Immed3 = 2,
  /// Special includes unique values like NIL, NONVALUE
  Special = 3,
}
/// Max value for the Immediate2 enum (for assertions).
pub const IMMEDIATE2_MAX: Word = 3;

#[repr(usize)]
pub enum Immediate2Special {
  Nil = 1,
  NonValue = 2,
}

/// Trim to have only immediate2 bits and return them as an convenient enum.
#[inline]
pub fn get_imm2_tag(val: Word) -> Immediate2 {
  let t: Word = val.get_bits(IMM2_TAG_FIRST..IMM2_TAG_LAST);
  assert!(t <= IMMEDIATE2_MAX);
  unsafe { transmute::<Word, Immediate2>(t) }
}

/// Remove tag bits from imm2 value by shifting it right
#[inline]
pub fn imm2_value(val: Word) -> Word {
  assert!(is_immediate2(val));
  val.get_bits(IMM2_VALUE_FIRST..IMM2_VALUE_LAST)
}

/// Precomposed bits for immediate2 values
pub const IMM2_PREFIX: Word = IMM1_PREFIX
    | ((Immediate1::Immed2 as Word) << IMM1_TAG_FIRST);

/// Precomposed bits for atom imm2
pub const IMM2_ATOM_PREFIX: Word = IMM2_PREFIX
    | ((Immediate2::Atom as Word) << IMM2_TAG_FIRST);

//--- Imm2 values tagged special ---

/// Special Primary tag+Immed1+Immed2 bits precomposed
pub const IMM2_SPECIAL_PREFIX: Word = IMM1_PREFIX
    | ((Immediate2::Special as Word) << IMM2_TAG_FIRST);

/// Precomposed bits for NIL constant
pub const IMM2_SPECIAL_NIL_RAW: Word = IMM2_SPECIAL_PREFIX
    | ((Immediate2Special::Nil as Word) << IMM2_VALUE_FIRST);

/// Precomposed bits for NON_VALUE constant
pub const IMM2_SPECIAL_NONVALUE_RAW: Word = IMM2_SPECIAL_PREFIX
    | ((Immediate2Special::NonValue as Word) << IMM2_VALUE_FIRST);

/// Get prefix bits BEFORE imm2 tag
#[inline]
pub fn get_imm2_prefix(val: Word) -> Word {
  val.get_bits(0..IMM2_TAG_FIRST)
}

#[inline]
pub fn is_immediate2(val: Word) -> bool {
  get_imm2_prefix(val) == IMM2_PREFIX
}

/// Given a value raw preset bits, compose them together and form an imm2 LTerm
#[inline]
pub fn combine_imm2_prefix_and_val(val: Word, prefix0: Word) -> Word {
  let mut prefix = prefix0;
  assert!(prefix < (1 << IMM2_VALUE_FIRST));
  assert!(val < (1 << (IMM2_VALUE_LAST - IMM2_VALUE_FIRST)));
  *prefix.set_bits(IMM2_VALUE_FIRST..IMM2_VALUE_LAST, val)
}

pub const fn combine_imm2_prefix_and_val_const(val: Word, prefix0: Word) -> Word {
  prefix0 | (val << IMM2_VALUE_FIRST)
}
