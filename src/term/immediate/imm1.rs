//!
//! Immediate1 values used to represent longer special term types.
//! Bit composition is - `.... .... .... aaPP`, where `PP` is primary tag, and
//! `aa` is imm1 tag
//!
//! Max value for imm1 is 64-4=60, or 32-4=28 bits.
//!
use term::primary;
use defs;
use defs::Word;
use term::immediate::primary::PRIM_TAG_LAST;

use std::mem::transmute;
use bit_field::BitField;

/// Bit position for imm1 tag
pub const IMM1_TAG_FIRST: u8 = 2;
pub const IMM1_TAG_LAST: u8 = 4;

/// Bit position for the value after imm1 tag
pub const IMM1_VALUE_FIRST: u8 = IMM1_TAG_LAST;
pub const IMM1_VALUE_LAST: u8 = defs::WORD_BITS as u8;

#[allow(dead_code)]
pub const SMALL_BITS: Word = (IMM1_VALUE_LAST - IMM1_VALUE_FIRST) as Word;

#[repr(usize)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Immediate1 {
  Pid = 0,
  Port = 1,
  Immed2 = 2,
  Small = 3,
}

/// Max value for the Immediate1 enum (for assertions).
pub const IMMEDIATE1_MAX: Word = 3;

/// Special tag {primary=Immediate} precomposed
pub const IMM1_PREFIX: Word = primary::TAG_IMMED;

/// Precomposed bits for pid imm1
pub const IMM1_PID_PREFIX: Word = IMM1_PREFIX
    | ((Immediate1::Pid as Word) << IMM1_TAG_FIRST);

pub const IMM1_SMALL_PREFIX: Word = IMM1_PREFIX
    | ((Immediate1::Small as Word) << IMM1_TAG_FIRST);

#[inline(always)]
pub fn is_immediate1(val: Word) -> bool {
  get_imm1_prefix(val) == IMM1_PREFIX
}

/// Get prefix bits BEFORE imm1 tag
#[inline(always)]
pub fn get_imm1_prefix(val: Word) -> Word {
  val.get_bits(0..PRIM_TAG_LAST)
}

/// Trim the immediate1 bits and return them as an convenient enum.
#[inline]
pub fn get_imm1_tag(val: Word) -> Immediate1 {
  let t: Word = val.get_bits(IMM1_TAG_FIRST..IMM1_TAG_LAST);
  assert!(t <= IMMEDIATE1_MAX);
  unsafe { transmute::<Word, Immediate1>(t) }
}

/// Remove tag bits from imm1 value by shifting it right
#[inline]
pub fn imm1_value(val: Word) -> Word {
  assert!(is_immediate1(val));
  val.get_bits(IMM1_VALUE_FIRST..IMM1_VALUE_LAST)
}

/// Given a value raw preset bits, compose them together and form an imm1 LTerm
#[inline]
pub fn combine_imm1_prefix_and_val(val: Word, prefix0: Word) -> Word {
  let mut prefix = prefix0;
  assert!(prefix < (1 << IMM1_VALUE_FIRST));
  assert!(val < (1 << (IMM1_VALUE_LAST - IMM1_VALUE_FIRST)));
  *prefix.set_bits(IMM1_VALUE_FIRST..IMM1_VALUE_LAST, val)
}
