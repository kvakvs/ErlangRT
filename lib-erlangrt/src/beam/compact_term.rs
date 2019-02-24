//! Module implements decoder for compact term format used in BEAM files.
//! <http://beam-wisdoms.clau.se/en/latest/indepth-beam-file.html#beam-compact-term-encoding>

use crate::{
  defs::{SWord, Word},
  fail::{Error, RtResult},
  rt_util::bin_reader::BinaryReader,
  term::{fterm::FTerm, integral::Integral},
};
use num::{bigint, ToPrimitive};

#[repr(u8)]
enum CTETag {
  LiteralInt = 0b000,
  Integer = 0b001,
  Atom = 0b010,
  XReg = 0b011,
  YReg = 0b100,
  Label = 0b101,
  Character = 0b110,
  Extended = 0b111,
}

#[cfg(feature = "r19")]
#[repr(u8)]
enum CTEExtTag {
  Float = 0b0001_0111,
  List = 0b0010_0111,
  FloatReg = 0b0011_0111,
  AllocList = 0b0100_0111,
  Literal = 0b0101_0111,
}

// In OTP20 the Float Ext tag is gone and Lists are taking the first value
#[cfg(feature = "r21")]
#[repr(u8)]
enum CTEExtTag {
  List = 0b0001_0111,
  FloatReg = 0b0010_0111,
  AllocList = 0b0011_0111,
  Literal = 0b0100_0111,
}

/// Errors created when parsing compact term format. They are delivered to the
/// end caller wrapped in `fail::Error:CodeLoadingCompactTerm(x)`
#[derive(Debug)]
pub enum CTError {
  BadLiteralTag,
  BadAtomTag,
  BadXRegTag,
  BadYRegTag,
  BadLabelTag,
  BadCharacterTag,
  BadIntegerTag,
  BadExtendedTag(String),
}

fn module() -> &'static str {
  "compact_term reader: "
}

#[inline]
fn make_err(e: CTError) -> RtResult<FTerm> {
  Err(Error::CodeLoadingCompactTerm(e))
}

// fn word_to_u32(w: Word) -> u32 {
//  assert!(w < std::u32::MAX as usize);
//  w as u32
//}

pub fn read(r: &mut BinaryReader) -> RtResult<FTerm> {
  let b = r.read_u8();
  let tag = b & 0b111;
  // let err_msg: &'static str = "Failed to parse beam compact term";

  let bword = if tag < CTETag::Extended as u8 {
    read_word(b, r)
  } else {
    Integral::Small(0)
  };

  match tag {
    x if x == CTETag::LiteralInt as u8 => {
      if let Integral::Small(index) = bword {
        return Ok(FTerm::SmallInt(index));
      }
      make_err(CTError::BadLiteralTag)
    }
    x if x == CTETag::Atom as u8 => {
      if let Integral::Small(index) = bword {
        if index == 0 {
          return Ok(FTerm::Nil);
        }
        return Ok(FTerm::LoadtimeAtom(index as usize));
      }
      make_err(CTError::BadAtomTag)
    }
    x if x == CTETag::XReg as u8 => {
      if let Integral::Small(index) = bword {
        return Ok(FTerm::XRegister(index as Word));
      }
      make_err(CTError::BadXRegTag)
    }
    x if x == CTETag::YReg as u8 => {
      if let Integral::Small(index) = bword {
        return Ok(FTerm::YRegister(index as Word));
      }
      make_err(CTError::BadYRegTag)
    }
    x if x == CTETag::Label as u8 => {
      if let Integral::Small(index) = bword {
        return Ok(FTerm::LoadtimeLabel(index as Word));
      }
      make_err(CTError::BadLabelTag)
    }
    x if x == CTETag::Integer as u8 => {
      if let Integral::Small(s) = bword {
        return Ok(FTerm::from_word(s));
      }
      if cfg!(debug_assertions) {
        println!("bad integer tag when parsing compact term format: {:?}", bword);
      }
      make_err(CTError::BadIntegerTag)
    }
    x if x == CTETag::Character as u8 => {
      if let Integral::Small(s) = bword {
        return Ok(FTerm::from_word(s));
      }
      make_err(CTError::BadCharacterTag)
    }
    // Extended tag (lower 3 bits = 0b111)
    _ => parse_ext_tag(b, r),
  }
  // return make_err(CTError::BadFormat)
}

#[cfg(feature = "r19")]
fn parse_ext_tag(b: u8, r: &mut BinaryReader) -> RtResult<FTerm> {
  match b {
    x if x == CTEExtTag::Float as u8 => parse_ext_float(r),
    x if x == CTEExtTag::List as u8 => parse_ext_list(r),
    x if x == CTEExtTag::FloatReg as u8 => parse_ext_fpreg(r),
    x if x == CTEExtTag::Literal as u8 => parse_ext_literal(r),
    x if x == CTEExtTag::AllocList as u8 => {
      panic!("Don't know how to decode an alloclist")
    }
    other => {
      let msg = format!("Ext tag {} unknown", other);
      make_err(CTError::BadExtendedTag(msg))
    }
  }
}

#[cfg(feature = "r21")]
fn parse_ext_tag(b: u8, r: &mut BinaryReader) -> RtResult<FTerm> {
  match b {
    x if x == CTEExtTag::List as u8 => parse_ext_list(r),
    x if x == CTEExtTag::AllocList as u8 => {
      panic!("Don't know how to decode an alloclist");
      // Ok(FTerm::AllocList_)
    }
    x if x == CTEExtTag::FloatReg as u8 => parse_ext_fpreg(r),
    x if x == CTEExtTag::Literal as u8 => parse_ext_literal(r),
    other => {
      let msg = format!("Ext tag {} unknown", other);
      make_err(CTError::BadExtendedTag(msg))
    }
  }
}

#[cfg(feature = "r19")]
fn parse_ext_float(r: &mut BinaryReader) -> RtResult<FTerm> {
  // floats are always stored as f64
  let fp_bytes = r.read_u64be();
  let fp: f64 = unsafe { std::mem::transmute::<u64, f64>(fp_bytes) };
  Ok(FTerm::Float(fp as defs::Float))
}

fn parse_ext_fpreg(r: &mut BinaryReader) -> RtResult<FTerm> {
  let b = r.read_u8();
  if let Integral::Small(reg) = read_word(b, r) {
    return Ok(FTerm::FloatRegister(reg as Word));
  }
  let msg = "Ext tag FPReg value too big".to_string();
  make_err(CTError::BadExtendedTag(msg))
}

fn parse_ext_literal(r: &mut BinaryReader) -> RtResult<FTerm> {
  let b = r.read_u8();
  if let Integral::Small(reg) = read_word(b, r) {
    return Ok(FTerm::LoadtimeLit(reg as Word));
  }
  let msg = "toExt tag Literal value too big".to_string();
  make_err(CTError::BadExtendedTag(msg))
}

fn parse_ext_list(r: &mut BinaryReader) -> RtResult<FTerm> {
  // The stream now contains a smallint size, then size/2 pairs of values
  let n_elts = read_int(r);
  let mut el: Vec<FTerm> = Vec::new();
  el.reserve(n_elts as usize);

  for _i in 0..n_elts {
    let value = read(r)?;
    el.push(value);
  }

  Ok(FTerm::LoadtimeExtlist(el))
}

/// Assume that the stream contains a tagged small integer (check the tag!)
/// read it and return the unwrapped value as word.
fn read_int(r: &mut BinaryReader) -> SWord {
  let b = r.read_u8();
  assert_eq!(b & 0b111, CTETag::LiteralInt as u8);
  match read_word(b, r) {
    Integral::Small(w) => w,
    Integral::BigInt(big) => big.to_isize().unwrap(),
  }
}

/// Given the first byte, parse an integer encoded after the 3-bit tag,
/// read more bytes from stream if needed.
fn read_word(b: u8, r: &mut BinaryReader) -> Integral {
  if 0 == (b & 0b1000) {
    // Bit 3 is 0 marks that 4 following bits contain the value
    return Integral::Small((b as SWord) >> 4);
  }
  // Bit 3 is 1, but...
  if 0 == (b & 0b1_0000) {
    // Bit 4 is 0, marks that the following 3 bits (most significant) and
    // the following byte (least significant) will contain the 11-bit value
    let r = ((b as Word) & 0b1110_0000) << 3 | (r.read_u8() as Word);
    Integral::Small(r as SWord)
  } else {
    // Bit 4 is 1 means that bits 5-6-7 contain amount of bytes+2 to store
    // the value
    let mut n_bytes = (b >> 5) as Word + 2;
    if n_bytes == 9 {
      // bytes=9 means upper 5 bits were set to 1, special case 0b11111xxx
      // which means that following nested tagged value encodes size,
      // followed by the bytes (Size+9)
      let bnext = r.read_u8();
      if let Integral::Small(tmp) = read_word(bnext, r) {
        n_bytes = tmp as Word + 9;
      } else {
        panic!("{}read word encountered a wrong byte length", module())
      }
    }

    // Read the remaining big endian bytes and convert to int
    let long_bytes = r.read_bytes(n_bytes).unwrap();
    let sign = if long_bytes[0] & 0x80 == 0x80 {
      bigint::Sign::Minus
    } else {
      bigint::Sign::Plus
    };
    let r = bigint::BigInt::from_bytes_be(sign, &long_bytes);
    Integral::from_big(r)
  } // if larger than 11 bits
}

// Testing section
//
#[cfg(test)]
mod tests {
  use super::*;

  fn try_parse(inp: Vec<u8>, expect: FTerm) {
    let mut r = BinaryReader::from_bytes(inp);
    match read(&mut r) {
      Ok(ref e) if e == &expect => {}
      other => {
        println!("Test got {:?}, expected {:?}", other, expect);
        assert!(false)
      }
    }
  }

  #[test]
  fn test_bigint_create() {
    let inp = vec![255u8, 1];
    let r = bigint::BigUint::from_bytes_be(inp.as_slice());
    assert_eq!(r.to_usize().unwrap(), (255 * 256 + 1) as usize);
  }

  #[test]
  fn test_lit() {
    try_parse(vec![0u8], FTerm::SmallInt(0));
  }

  #[test]
  fn test_int() {
    try_parse(vec![0b1u8], FTerm::SmallInt(0));
  }

  // This test is not applicable to R20+ where the Float ext tag is removed
  #[test]
  #[cfg(feature = "r19")]
  fn test_float() {
    try_parse(
      vec![0b00010111u8, 63, 243, 192, 193, 252, 143, 50, 56],
      FTerm::Float(1.23456),
    );
  }

  // TODO: test extended

  /// Given the vec<u8> input, we read the word encoded in it and compare with
  /// the word or bigint expected
  fn try_read_word(inp: Vec<u8>, expect: Integral) {
    let mut r = BinaryReader::from_bytes(inp);
    let b0 = r.read_u8();
    assert_eq!(read_word(b0, &mut r), expect)
  }

  #[test]
  fn test_read_word_4bit() {
    try_read_word(vec![0b10010000u8], Integral::Small(9));
    try_read_word(vec![0b11110000u8], Integral::Small(15));
  }

  #[test]
  fn test_read_word_11bit() {
    try_read_word(vec![0b1000u8, 127], Integral::Small(127));
    try_read_word(vec![0b10101000u8, 255], Integral::Small(0b101 * 256 + 255));
    try_read_word(
      vec![0b11101000u8, 0b00001111],
      Integral::Small(0b111 * 256 + 0b00001111),
    );
  }

  #[test]
  fn test_read_word_16to64bit() {
    try_read_word(vec![0b00011000u8, 127, 1], Integral::Small(127 * 256 + 1));
  }
}
