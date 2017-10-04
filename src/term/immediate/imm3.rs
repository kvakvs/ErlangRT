//!
//! Special smallest values used in code representation
//! Bit composition is - `.... .... ccbb aaPP`, where `PP` is primary tag,
//! `aa` is imm1 tag, `bb` is imm2 tag, and `cc` is imm3 tag.
//!
//! Max value for imm3 is 64-8=56, or 32-8=24 bits.
//!
use defs::Word;
use defs;
use term::immediate::imm1::*;
use term::immediate::imm2::*;
use term::primary;

use std::mem::transmute;
use bit_field::BitField;

/// Bit position for imm1 tag
pub const IMM3_TAG_FIRST: u8 = 6;
pub const IMM3_TAG_LAST: u8 = 8;

/// Bit position for the value after imm1 tag
pub const IMM3_VALUE_FIRST: u8 = IMM3_TAG_LAST;
pub const IMM3_VALUE_LAST: u8 = defs::WORD_BITS as u8;

#[repr(usize)]
#[derive(Debug, Eq, PartialEq)]
pub enum Immediate3 {
  XReg = 0,
  YReg = 1,
  FPReg = 2,
  Label = 3,
}

/// Max value for the Immediate3 enum (for assertions).
pub const IMMEDIATE3_MAX: Word = 3;

/// Trim to have only immediate3 bits and return them as an convenient enum.
#[inline]
pub fn get_imm3_tag(val: Word) -> Immediate3 {
  let t: Word = val.get_bits(IMM3_TAG_FIRST..IMM3_TAG_LAST);
  assert!(t <= IMMEDIATE3_MAX);
  unsafe { transmute::<Word, Immediate3>(t) }
}

/// Remove tag bits from imm3 value by shifting it right
#[inline]
pub fn imm3_value(val: Word) -> Word {
  assert!(is_immediate3(val));
  val.get_bits(IMM3_VALUE_FIRST..IMM3_VALUE_LAST)
}

/// Precomposed bits to use with immediate3 tags and values
pub const IMM3_PREFIX: Word = IMM2_PREFIX
    | ((Immediate2::Immed3 as Word) << IMM2_TAG_FIRST);

/// Bit prefix for X register value
pub const IMM3_XREG_PREFIX: Word = IMM3_PREFIX
    | ((Immediate3::XReg as Word) << IMM3_TAG_FIRST);

pub const IMM3_YREG_PREFIX: Word = IMM3_PREFIX
    | ((Immediate3::YReg as Word) << IMM3_TAG_FIRST);

pub const IMM3_FPREG_PREFIX: Word = IMM3_PREFIX
    | ((Immediate3::FPReg as Word) << IMM3_TAG_FIRST);

pub const IMM3_LABEL_PREFIX: Word = IMM3_PREFIX
    | ((Immediate3::Label as Word) << IMM3_TAG_FIRST);

/// Get prefix bits BEFORE imm3 tag
#[inline]
pub fn get_imm3_prefix(val: Word) -> Word {
  val.get_bits(0..IMM3_TAG_FIRST)
}

#[inline]
pub fn is_immediate3(val: Word) -> bool {
  get_imm3_prefix(val) == IMM3_PREFIX
}

/// Given a value raw preset bits, compose them together and form an imm3 LTerm
#[inline]
pub fn create_imm3(val: Word, prefix0: Word) -> Word {
  let mut prefix = prefix0;
  assert!(prefix < (1 << IMM3_VALUE_FIRST));
  assert!(val < (1 << (IMM3_VALUE_LAST - IMM3_VALUE_FIRST)));
  *prefix.set_bits(IMM3_VALUE_FIRST..IMM3_VALUE_LAST, val)
}
