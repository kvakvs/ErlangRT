//!
//! Defines header tag values used on heap to define the header type.
//!
use bit_field::BitField;

use defs;
use defs::Word;
use term::primary;
use term::primary::PRIM_VALUE_FIRST;


/// Bit position for the header tag following the primary tag.
pub const HEADER_TAG_FIRST: u8 = PRIM_VALUE_FIRST;
pub const HEADER_TAG_LAST: u8 = HEADER_TAG_FIRST + 4;


/// Bit position for the value after the primary and the header tag.
pub const HEADER_VALUE_FIRST: u8 = HEADER_TAG_LAST;
pub const HEADER_VALUE_LAST: u8 = defs::WORD_BITS as u8;


/// Marks the type, which follows the header word.
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
#[allow(dead_code)]
pub enum HeaderTag {
  Tuple = 0,
  BigNegative = 2,
  BigPositive = 3,
  Reference = 4,
  Fun = 5,
  Float = 6,
  Export = 7,
  RefcBinary = 8,
  HeapBinary = 9,
  SubBinary = 10,
  // match 11?
  ExternalPid = 12,
  ExternalPort = 13,
  ExternalRef = 14,
}

const HEADER_TAG_TUPLE_RAW: Word = (primary::Tag::Header as Word)
      | ((HeaderTag::Tuple as Word) << HEADER_TAG_FIRST);

const HEADER_TAG_BIGNEG_RAW: Word = (primary::Tag::Header as Word)
    | ((HeaderTag::BigNegative as Word) << HEADER_TAG_FIRST);

const HEADER_TAG_BIGPOS_RAW: Word = (primary::Tag::Header as Word)
    | ((HeaderTag::BigPositive as Word) << HEADER_TAG_FIRST);


#[inline]
fn make_header_raw(val: Word, premade: Word) -> Word {
  let mut premade1 = premade;
  *premade1.set_bits(HEADER_VALUE_FIRST..HEADER_VALUE_LAST, val)
}


#[inline]
pub fn make_tuple_header_raw(sz: Word) -> Word {
  make_header_raw(sz, HEADER_TAG_TUPLE_RAW)
}


#[inline]
pub fn make_bignum_neg_header_raw(sz: Word) -> Word {
  make_header_raw(sz, HEADER_TAG_BIGNEG_RAW)
}


#[inline]
pub fn make_bignum_pos_header_raw(sz: Word) -> Word {
  make_header_raw(sz, HEADER_TAG_BIGPOS_RAW)
}
