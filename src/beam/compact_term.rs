use types::{Word, Integral};
use types;
use util::reader::BinaryReader;
use rterror;

use std;
use std::mem;
use num;
use num::bigint;

/// Enum type represents a compacted simplified term format used in BEAM files,
/// must be converted to real term during the module loading phase
#[derive(Debug, PartialEq)]
pub enum CompactTerm {
  Literal(Word),
  Integer(Integral),
  Atom(Word),
  XReg(Word),
  YReg(Word),
  Label(Word),
  Character(Word),
  Float(types::Float),
  List,
  FPReg(Word),
  AllocList,
  ExtLiteral,
}

enum CTETag {
  Literal = 0,
  Integer = 1,
  Atom = 2,
  XReg = 3,
  YReg = 4,
  Label = 5,
  Character = 6,
  Extended = 7,
}

enum CTEExtTag {
  Float = 0b00010111,
  List = 0b00100111,
  FloatReg = 0b00110111,
  AllocList = 0b01000111,
  Literal = 0b01010111,
}

/// Errors created when parsing compact term format. They are delivered to the
/// end caller wrapped in `rterror::Error:CodeLoadingCompactTerm(x)`
#[derive(Debug)]
pub enum CTError {
  BadLiteralTag,
  BadAtomTag,
  BadXRegTag,
  BadYRegTag,
  BadLabelTag,
  BadCharacterTag,
  BadExtendedTag,
  BadFormat,
}

fn module() -> &'static str { "compact_term reader: " }

fn make_err(e: CTError) -> Result<CompactTerm, rterror::Error> {
  Err(rterror::Error::CodeLoadingCompactTerm(e))
}

fn word_to_u32(w: Word) -> u32 {
  assert!(w < std::u32::MAX as usize);
  w as u32
}

pub fn read(r: &mut BinaryReader) -> Result<CompactTerm, rterror::Error> {
  let b = r.read_u8();
  let tag = b & 0b111;
  let err_msg: &'static str = "Failed to parse beam compact term";

  let mut bword = Integral::Word(0);
  if tag < CTETag::Extended as u8 {
    bword = read_word(b, r);
  }

  match tag {
    x if x == CTETag::Literal as u8 => {
      if let Integral::Word(index) = bword {
        return Ok(CompactTerm::Literal(index))
      }
      return make_err(CTError::BadLiteralTag)
    },
    x if x == CTETag::Atom as u8 => {
      if let Integral::Word(index) = bword {
        return Ok(CompactTerm::Atom(index))
      }
      return make_err(CTError::BadAtomTag)
    },
    x if x == CTETag::XReg as u8 => {
      if let Integral::Word(index) = bword {
        return Ok(CompactTerm::XReg(index))
      }
      return make_err(CTError::BadXRegTag)
    },
    x if x == CTETag::YReg as u8 => {
      if let Integral::Word(index) = bword {
        return Ok(CompactTerm::YReg(index))
      }
      return make_err(CTError::BadYRegTag)
    },
    x if x == CTETag::Label as u8 => {
      if let Integral::Word(index) = bword {
        return Ok(CompactTerm::Label(index))
      }
      return make_err(CTError::BadLabelTag)
    },
    x if x == CTETag::Integer as u8 => {
      return Ok(CompactTerm::Integer(bword))
    },
    x if x == CTETag::Character as u8 => {
      if let Integral::Word(index) = bword {
        return Ok(CompactTerm::Character(index))
      }
      return make_err(CTError::BadCharacterTag)
    }
    // Extended tag (lower 3 bits = 0b111)
    _ => {
      match b {
        x if x == CTEExtTag::Float as u8 => {
          // floats are always stored as f64
          let fp_bytes = r.read_u64be();
          let fp: f64 = unsafe {
            std::mem::transmute::<u64, f64>(fp_bytes)
          };
          return Ok(CompactTerm::Float(fp as types::Float))
        },
        _ => {
          return make_err(CTError::BadExtendedTag)
        }
      }
    }
  }

  return make_err(CTError::BadFormat)
}

/// Given the first byte, parse an integer encoded after the 3-bit tag,
/// read more bytes from stream if needed.
fn read_word(b: u8, r: &mut BinaryReader) -> Integral {
  if 0 == (b & 0b1000) {
    // Bit 3 is 0 marks that 4 following bits contain the value
    return Integral::Word((b as Word) >> 4);
  }
  // Bit 3 is 1, but...
  if 0 == (b & 0b10000) {
    // Bit 4 is 0, marks that the following 3 bits (most significant) and
    // the following byte (least significant) will contain the 11-bit value
    let r = ((b as Word) & 0b11100000) << 3 | (r.read_u8() as Word);
    return Integral::Word(r);
  } else {
    // Bit 4 is 1 means that bits 5-6-7 contain amount of bytes+2 to store
    // the value
    let mut n_bytes = (((b as Word) & 0b11100000) >> 5) + 2;
    if n_bytes == 9 {
      // bytes=9 means upper 5 bits were set to 1, special case 0b11111xxx
      // which means that following nested tagged value encodes size,
      // followed by the bytes (Size+9)
      let bnext = r.read_u8();
      if let Integral::Word(tmp) = read_word(bnext, r) {
        n_bytes = tmp + 9;
      } else {
        panic!("{}read word encountered a wrong byte length", module())
      }
    }
    // Read the remaining big endian bytes and convert to int
    let long_bytes = r.read_bytes(n_bytes).unwrap();
    let r = num::BigInt::from_bytes_be(bigint::Sign::NoSign,
                                       long_bytes.as_slice());
    Integral::BigInt(r)
  } // if larger than 11 bits
}

//
// Testing section
//
#[cfg(test)]
mod tests {
  use super::*;

  fn try_parse(inp: Vec<u8>, expect: CompactTerm) {
    let mut r = BinaryReader::from_bytes(inp);
    match read(&mut r) {
      Ok(ref e) if e == &expect => {},
      other => {
        println!("Test got {:?}, expected {:?}", other, expect);
        assert!(false)
      }
    }
  }

  #[test]
  fn test_lit() {
    try_parse(vec![0u8], CompactTerm::Literal(0));
  }

  #[test]
  fn test_int() {
    try_parse(vec![0b1u8], CompactTerm::Integer(Integral::Word(0)));
  }

  #[test]
  fn test_float() {
    try_parse(vec![0b00010111u8, 63, 243, 192, 193, 252, 143, 50, 56],
              CompactTerm::Float(1.23456));
  }

  // TODO: test reading longer and very long words with read_word
  // TODO: test extended
}
