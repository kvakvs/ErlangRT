//!
//! Low level term representation for compact heap storage
//!
//! Term bits are: `.... .... ..bb aaPP`
//!
//! Here "PP" are the primary tag, one of `primary_tag::Tag::Immediate`
//! And "aa" with size 2 bits, uses `Immediate1` bits.
//!
//! To use `Immediate2` bits set "aa" to `Immediate1::Immed2` and set "bb" to the
//!    desired value from `Immediate2` enum.
//!
use term::primary_tag;
use defs;
use defs::Word;

use std::mem::transmute;

//
// Premade bit combinations and constants for Immediate1 values
//

const IMM1_SIZE: Word = 2;
/// A mask to apply to a value shifted right by IMM1_TAG_SHIFT to get imm1 tag
const IMM1_TAG_MASK: Word = (1 << IMM1_SIZE) - 1;
/// How much to shift tag to place it into Immediate1 tag bits
const IMM1_TAG_SHIFT: Word = primary_tag::SIZE;
/// How much to shift a value to place it after Immediate2 tag
const IMM1_VALUE_SHIFT: Word = IMM1_TAG_SHIFT + IMM1_SIZE;
const IMM1_MASK: Word = (1 << IMM1_VALUE_SHIFT) - 1;

enum Immediate1 {
  Pid = 0,
  Port = 1,
  Small = 2,
  Immed2 = 3,
}

/// Cut away the value to be able to compare with raw prefixes
#[inline]
fn get_imm1_prefix(val: Word) -> Word {
  val & IMM1_MASK
}

//#[inline]
//fn get_imm1_tag(val: Word) -> Immediate1 {
//  let t: Word = (val >> IMM1_TAG_SHIFT) & IMM1_TAG_MASK;
//  unsafe { transmute::<Word, Immediate1>(t) }
//}

/// Special Primary tag+Immed1 precomposed
const IMM1_PREFIX: Word = primary_tag::Tag::Immediate as Word;

/// Precomposed bits for pid imm1
const IMM1_PID_PREFIX: Word = IMM1_PREFIX
    | ((Immediate1::Pid as Word) << primary_tag::SIZE);

const IMM1_SMALL_PREFIX: Word = IMM1_PREFIX
    | ((Immediate1::Small as Word) << primary_tag::SIZE);

//--- Immediate 2 precomposed values ---

enum Immediate2 {
  Atom = 0,
  Catch = 1,
  /// Special includes unique values like NIL, NONVALUE
  Special = 2,
  /// not used
  Immed3 = 3,
}

enum Immediate2Special {
  Nil = 1,
  NonValue = 2,
}

const IMM2_SIZE: Word = 2;
/// A mask to apply to a value shifted right by IMM2_TAG_SHIFT to get imm2 tag
const IMM2_TAG_MASK: Word = (1 << IMM2_SIZE) - 1;
/// How much to shift tag to place it into Immediate2 tag bits
const IMM2_TAG_SHIFT: Word = IMM1_VALUE_SHIFT;
/// How much to shift a value to place it after Immediate2 tag
const IMM2_VALUE_SHIFT: Word = IMM1_TAG_SHIFT + IMM1_SIZE + IMM2_SIZE;
const IMM2_MASK: Word = (1 << IMM2_VALUE_SHIFT) - 1;

/// Cut away the value to be able to compare with raw prefixes
#[inline]
fn get_imm2_prefix(val: Word) -> Word {
  val & IMM2_MASK
}

/// Precomposed bits for immediate2 values
const IMM2_PREFIX: Word = IMM1_PREFIX
    | ((Immediate1::Immed2 as Word) << primary_tag::SIZE);

/// Precomposed bits for atom imm2
pub const IMM2_ATOM_PREFIX: Word = IMM2_PREFIX
    | ((Immediate2::Atom as Word) << IMM1_VALUE_SHIFT);

//--- Imm2 values tagged special ---

/// Special Primary tag+Immed1+Immed2 bits precomposed
const IMM2_SPECIAL_PREFIX: Word = IMM1_PREFIX
    | ((Immediate2::Special as Word) << IMM2_TAG_SHIFT);

/// Precomposed bits for NIL constant
pub const IMM2_SPECIAL_NIL_PREFIX: Word = IMM2_SPECIAL_PREFIX
    | ((Immediate2Special::Nil as Word) << IMM2_VALUE_SHIFT);

/// Precomposed bits for NON_VALUE constant
pub const IMM2_SPECIAL_NONVALUE_PREFIX: Word = IMM2_SPECIAL_PREFIX
    | ((Immediate2Special::NonValue as Word) << IMM2_VALUE_SHIFT);

//--- Immediate 3 precomposed values ---
// Bit composition is - .... .... ccbb aaPP

enum Immediate3 {
  XReg = 0,
  YReg = 1,
  FPReg = 3,
}

const IMM3_SIZE: Word = 2;
/// A mask to apply to a value shifted right by IMM3_TAG_SHIFT to get imm3 tag
const IMM3_TAG_MASK: Word = (1 << IMM3_SIZE) - 1;
/// How much to shift a tag to place it into Immediate3 tag bits
const IMM3_TAG_SHIFT: Word = IMM2_VALUE_SHIFT;
/// How much to shift a value to place it after Immediate3 tag
const IMM3_VALUE_SHIFT: Word = IMM1_TAG_SHIFT + IMM1_SIZE + IMM2_SIZE + IMM3_SIZE;
const IMM3_MASK: Word = (1 << IMM3_VALUE_SHIFT) - 1;

pub const IMM3_PREFIX: Word = IMM2_PREFIX
    | ((Immediate2::Immed3 as Word) << IMM2_TAG_SHIFT);

/// Bit prefix for X register value
pub const IMM3_XREG_PREFIX: Word = IMM3_PREFIX
    | ((Immediate3::XReg as Word) << IMM3_VALUE_SHIFT);

//
// Construction
//

/// Given value (to be shifted) and RAW_* preset bits, compose them together for imm2
#[inline]
fn create_imm2(val: Word, raw_preset: Word) -> Word {
  (val << IMM2_VALUE_SHIFT) | raw_preset
}

pub fn is_immediate1(val: Word) -> bool { val & IMM1_MASK == IMM1_PREFIX }
pub fn is_immediate2(val: Word) -> bool { val & IMM2_MASK == IMM2_PREFIX }
pub fn is_immediate3(val: Word) -> bool { val & IMM3_MASK == IMM3_PREFIX }

/// Given a value (to be shifted) and RAW_* preset bits, compose them together for imm1
#[inline]
fn create_imm1(val: Word, raw_preset: Word) -> Word {
  (val << IMM1_VALUE_SHIFT) | raw_preset
}

/// Remove tag bits from imm2 value by shifting it right
#[inline]
pub fn imm2_value(val: Word) -> Word {
  assert!(primary_tag::is_primary_tag(val, primary_tag::Tag::Immediate));
  assert!(is_immediate2(val));
  val >> IMM2_VALUE_SHIFT
}

/// Given a value (to be shifted) and RAW_* preset bits, compose them together for imm1
#[inline]
fn create_imm3(val: Word, raw_preset: Word) -> Word {
  (val << IMM3_VALUE_SHIFT) | raw_preset
}

//
// Construction
//

/// Create a raw value for a term from atom index
#[inline]
pub fn make_atom_raw(val: Word) -> Word {
  create_imm2(val, IMM2_ATOM_PREFIX)
}

/// Create a raw value for a pid term from process index
#[inline]
pub fn make_pid_raw(pindex: Word) -> Word {
  create_imm1(pindex, IMM1_PID_PREFIX)
}

/// Create a raw smallint value for a term from atom index
#[inline]
pub fn make_small_raw(val: Word) -> Word {
  create_imm1(val, IMM1_SMALL_PREFIX)
}

#[inline]
pub fn make_xreg_raw(x: Word) -> Word {
  assert!(x < defs::MAX_XREGS);
  create_imm3(x, IMM3_XREG_PREFIX)
}

//
// Checks
//

#[inline]
pub fn is_pid_raw(val: Word) -> bool {
  get_imm1_prefix(val) == IMM1_PID_PREFIX
}

#[inline]
pub fn is_atom_raw(val: Word) -> bool {
  get_imm2_prefix(val) == IMM2_ATOM_PREFIX
}
