//! Module implements decoder for compact term format used in BEAM files.
//! <http://beam-wisdoms.clau.se/en/latest/indepth-beam-file.html#beam-compact-term-encoding>

use crate::{
  beam::loader::CompactTermError,
  defs::{SWord, Word},
  emulator::heap::Heap,
  fail::{RtErr, RtResult},
  rt_util::bin_reader::BinaryReader,
  term::{
    boxed::{self, bignum, endianness::Endianness},
    lterm::{Term, SPECIAL_LT_LITERAL},
  },
};

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
#[cfg(not(feature = "r19"))]
#[repr(u8)]
enum CTEExtTag {
  List = 0b0001_0111,
  FloatReg = 0b0010_0111,
  AllocList = 0b0011_0111,
  Literal = 0b0100_0111,
}

fn module() -> &'static str {
  "compact_term: "
}

#[inline]
fn make_err<T>(e: CompactTermError) -> RtResult<T> {
  Err(RtErr::CodeLoadingCompactTerm(e))
}

pub fn read(r: &mut BinaryReader) -> RtResult<Term> {
  let b = r.read_u8();
  let tag = b & 0b111;

  let bword = if tag < CTETag::Extended as u8 {
    read_word(b, r)
  } else {
    Term::make_small_unsigned(0)
  };

  match tag {
    x if x == CTETag::LiteralInt as u8 => {
      if bword.is_small() {
        return Ok(bword);
      }
      make_err(CompactTermError::BadLiteralTag)
    }
    x if x == CTETag::Atom as u8 => {
      if bword.is_small() {
        let index = bword.get_small_unsigned();
        if index == 0 {
          return Ok(Term::nil());
        }
        return Ok(Term::make_ltatom(index));
      }
      make_err(CompactTermError::BadAtomTag)
    }
    x if x == CTETag::XReg as u8 => {
      if bword.is_small() {
        return Ok(Term::make_regx(bword.get_small_unsigned()));
      }
      make_err(CompactTermError::BadXRegTag)
    }
    x if x == CTETag::YReg as u8 => {
      if bword.is_small() {
        return Ok(Term::make_regy(bword.get_small_unsigned()));
      }
      make_err(CompactTermError::BadYRegTag)
    }
    x if x == CTETag::Label as u8 => {
      if bword.is_small() {
        return Ok(Term::make_ltlabel(bword.get_small_unsigned()));
      }
      make_err(CompactTermError::BadLabelTag)
    }
    x if x == CTETag::Integer as u8 => {
      if bword.is_small() {
        return Ok(bword);
      }
      if cfg!(debug_assertions) {
        println!(
          "bad integer tag when parsing compact term format: {}",
          bword
        );
      }
      make_err(CompactTermError::BadIntegerTag)
    }
    x if x == CTETag::Character as u8 => {
      if bword.is_small() {
        return Ok(bword);
      }
      make_err(CompactTermError::BadCharacterTag)
    }
    // Extended tag (lower 3 bits = 0b111)
    _ => parse_ext_tag(b, r),
  }
  // return make_err(CTError::BadFormat)
}

#[cfg(feature = "r19")]
fn parse_ext_tag(hp: &mut Heap, b: u8, r: &mut BinaryReader) -> RtResult<LtTerm> {
  match b {
    x if x == CTEExtTag::List as u8 => parse_ext_list(hp, r),
    x if x == CTEExtTag::Float as u8 => parse_ext_float(hp, r),
    x if x == CTEExtTag::FloatReg as u8 => parse_ext_fpreg(r),
    x if x == CTEExtTag::Literal as u8 => parse_ext_literal(r),
    x if x == CTEExtTag::AllocList as u8 => {
      panic!("Don't know how to decode an alloclist")
    }
    other => {
      let msg = format!("Ext tag {} unknown", other);
      make_err(CompactTermError::BadExtendedTag(msg))
    }
  }
}

#[cfg(not(feature = "r19"))]
fn parse_ext_tag(hp: &mut Heap, b: u8, r: &mut BinaryReader) -> RtResult<Term> {
  match b {
    x if x == CTEExtTag::List as u8 => parse_ext_list(hp, r),
    x if x == CTEExtTag::Float as u8 => parse_ext_float(hp, r),
    x if x == CTEExtTag::AllocList as u8 => {
      panic!("Don't know how to decode an alloclist");
      // Ok(FTerm::AllocList_)
    }
    x if x == CTEExtTag::FloatReg as u8 => parse_ext_fpreg(r),
    x if x == CTEExtTag::Literal as u8 => parse_ext_literal(r),
    other => {
      let msg = format!("Ext tag {} unknown", other);
      make_err(CompactTermError::BadExtendedTag(msg))
    }
  }
}

fn parse_ext_float(hp: &mut Heap, r: &mut BinaryReader) -> RtResult<Term> {
  // floats are always stored as f64
  let fp_bytes = r.read_u64be();
  let fp: f64 = unsafe { std::mem::transmute::<u64, f64>(fp_bytes) };
  Term::make_float(hp, fp)
}

fn parse_ext_fpreg(r: &mut BinaryReader) -> RtResult<Term> {
  let b = r.read_u8();
  let reg = read_word(b, r);
  if reg.is_small() {
    return Ok(Term::make_regfp(reg.get_small_unsigned()));
  }
  let msg = "Ext tag FPReg value too big".to_string();
  make_err(CompactTermError::BadExtendedTag(msg))
}

fn parse_ext_literal(r: &mut BinaryReader) -> RtResult<Term> {
  let b = r.read_u8();
  let reg = read_word(b, r);
  if reg.is_small() {
    return Ok(Term::make_loadtime(
      SPECIAL_LT_LITERAL,
      reg.get_small_unsigned(),
    ));
  }
  let msg = "compact_term: loadtime Literal index too big".to_string();
  make_err(CompactTermError::BadExtendedTag(msg))
}

/// Parses a list, places on the provided heap
fn parse_ext_list(hp: &mut Heap, r: &mut BinaryReader) -> RtResult<Term> {
  // The stream now contains a smallint size, then size/2 pairs of values
  let n_pairs = read_int(r) / 2;
  let mut jt = boxed::JumpTable::create_into(hp, n_pairs)?;

  for i in 0..n_pairs {
    let value = read(r)?;
    let loc = read(r)?;
    unsafe {
      jt.set_pair(i, value, loc);
    }
  }

  Ok(Term::make_boxed(jt))
}

/// Assume that the stream contains a tagged small integer (check the tag!)
/// read it and return the unwrapped value as word.
fn read_int(r: &mut BinaryReader) -> SWord {
  let b = r.read_u8();
  assert_eq!(b & 0b111, CTETag::LiteralInt as u8);
  let val = read_word(b, r);
  if val.is_small() {
    return val.get_small_signed();
  }
  unimplemented!("unwrap bigint from term or return an error")
}

/// Given the first byte, parse an integer encoded after the 3-bit tag,
/// read more bytes from stream if needed.
fn read_word(hp: &mut Heap, b: u8, r: &mut BinaryReader) -> Term {
  if 0 == (b & 0b1000) {
    // Bit 3 is 0 marks that 4 following bits contain the value
    return Term::make_small_signed((b as SWord) >> 4);
  }
  // Bit 3 is 1, but...
  if 0 == (b & 0b1_0000) {
    // Bit 4 is 0, marks that the following 3 bits (most significant) and
    // the following byte (least significant) will contain the 11-bit value
    let r = ((b as Word) & 0b1110_0000) << 3 | (r.read_u8() as Word);
    Term::make_small_signed(r as SWord)
  } else {
    // Bit 4 is 1 means that bits 5-6-7 contain amount of bytes+2 to store
    // the value
    let mut n_bytes = (b >> 5) as Word + 2;
    if n_bytes == 9 {
      // bytes=9 means upper 5 bits were set to 1, special case 0b11111xxx
      // which means that following nested tagged value encodes size,
      // followed by the bytes (Size+9)
      let bnext = r.read_u8();
      let tmp = read_word(bnext, r);
      if tmp.is_small() {
        n_bytes = tmp.get_small_unsigned() + 9;
      } else {
        panic!("{}read word encountered a wrong byte length", module())
      }
    }

    // Read the remaining big endian bytes and convert to int
    let long_bytes = r.read_bytes(n_bytes).unwrap();
    let sign = if long_bytes[0] & 0x80 == 0x80 {
      bignum::sign::Sign::Negative
    } else {
      bignum::sign::Sign::Positive
    };

    let r =
      unsafe { boxed::Bignum::create_into(hp, sign, Endianness::Big, &long_bytes)? };
    Term::make_boxed(r)
  } // if larger than 11 bits
}

//// Testing section
////
//#[cfg(test)]
// mod tests {
//  use super::*;
//
//  fn try_parse(inp: Vec<u8>, expect: LtTerm) {
//    let mut r = BinaryReader::from_bytes(inp);
//    match read(&mut r) {
//      Ok(ref e) if e == &expect => {}
//      other => {
//        println!("Test got {:?}, expected {:?}", other, expect);
//        assert!(false)
//      }
//    }
//  }
//
//  #[test]
//  fn test_bigint_create() {
//    let inp = vec![255u8, 1];
//    let r = big::Big::from_bytes_be(big::Sign::Positive, inp.as_slice());
//    assert_eq!(r.to_usize().unwrap(), (255 * 256 + 1) as usize);
//  }
//
//  #[test]
//  fn test_lit() {
//    try_parse(vec![0u8], LtTerm::SmallInt(0));
//  }
//
//  #[test]
//  fn test_int() {
//    try_parse(vec![0b1u8], LtTerm::SmallInt(0));
//  }
//
//  // This test is not applicable to R20+ where the Float ext tag is removed
//  #[test]
//  #[cfg(feature = "r19")]
//  fn test_float() {
//    try_parse(
//      vec![0b00010111u8, 63, 243, 192, 193, 252, 143, 50, 56],
//      LtTerm::Float(1.23456),
//    );
//  }
//
//  // TODO: test extended
//
//  /// Given the vec<u8> input, we read the word encoded in it and compare with
//  /// the word or bigint expected
//  fn try_read_word(inp: Vec<u8>, expect: LtIntegral) {
//    let mut r = BinaryReader::from_bytes(inp);
//    let b0 = r.read_u8();
//    assert_eq!(read_word(b0, &mut r), expect)
//  }
//
//  #[test]
//  fn test_read_word_4bit() {
//    try_read_word(vec![0b10010000u8], LtIntegral::Small(9));
//    try_read_word(vec![0b11110000u8], LtIntegral::Small(15));
//  }
//
//  #[test]
//  fn test_read_word_11bit() {
//    try_read_word(vec![0b1000u8, 127], LtIntegral::Small(127));
//    try_read_word(
//      vec![0b10101000u8, 255],
//      LtIntegral::Small(0b101 * 256 + 255),
//    );
//    try_read_word(
//      vec![0b11101000u8, 0b00001111],
//      LtIntegral::Small(0b111 * 256 + 0b00001111),
//    );
//  }
//
//  #[test]
//  fn test_read_word_16to64bit() {
//    try_read_word(vec![0b00011000u8, 127, 1], LtIntegral::Small(127 * 256 + 1));
//  }
//}
