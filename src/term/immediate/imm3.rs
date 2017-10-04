use term::primary;
use defs;
use defs::Word;
use term::immediate::imm1::*;
use term::immediate::imm2::*;

use std::mem::transmute;

//--- Immediate 3 precomposed values ---
// Special smaller values used in code representation
// Bit composition is - .... .... ccbb aaPP

#[repr(u8)]
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
  let t: Word = (val >> IMM3_TAG_SHIFT) & IMM3_TAG_MASK;
  assert!(t <= IMMEDIATE3_MAX);
  unsafe { transmute(t as u8) }
}

/// Remove tag bits from imm3 value by shifting it right
#[inline]
pub fn imm3_value(val: Word) -> Word {
  assert!(is_immediate3(val));
  val >> IMM3_VALUE_SHIFT
}

pub const IMM3_SIZE: Word = 2;

/// A mask to apply to a value shifted right by IMM3_TAG_SHIFT to get imm3 tag
pub const IMM3_TAG_MASK: Word = (1 << IMM3_SIZE) - 1;

/// How much to shift a tag to place it into Immediate3 tag bits
pub const IMM3_TAG_SHIFT: Word = IMM2_VALUE_SHIFT;

/// How much to shift a value to place it after Immediate3 tag
pub const IMM3_VALUE_SHIFT: Word = IMM1_TAG_SHIFT + IMM1_SIZE + IMM2_SIZE + IMM3_SIZE;

pub const IMM3_MASK: Word = (1 << IMM3_VALUE_SHIFT) - 1;

pub const IMM3_PREFIX: Word = IMM2_PREFIX
    | ((Immediate2::Immed3 as Word) << IMM2_TAG_SHIFT);

/// Bit prefix for X register value
pub const IMM3_XREG_PREFIX: Word = IMM3_PREFIX
    | ((Immediate3::XReg as Word) << IMM3_VALUE_SHIFT);

pub const IMM3_YREG_PREFIX: Word = IMM3_PREFIX
    | ((Immediate3::YReg as Word) << IMM3_VALUE_SHIFT);

pub const IMM3_FPREG_PREFIX: Word = IMM3_PREFIX
    | ((Immediate3::FPReg as Word) << IMM3_VALUE_SHIFT);

pub const IMM3_LABEL_PREFIX: Word = IMM3_PREFIX
    | ((Immediate3::Label as Word) << IMM3_VALUE_SHIFT);


pub fn is_immediate3(val: Word) -> bool { val & IMM3_MASK == IMM3_PREFIX }


/// Given a value (to be shifted) and RAW_* preset bits, compose them together for imm1
#[inline]
pub fn create_imm3(val: Word, raw_preset: Word) -> Word {
  (val << IMM3_VALUE_SHIFT) | raw_preset
}
