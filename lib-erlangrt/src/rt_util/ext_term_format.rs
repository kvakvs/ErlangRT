use super::bin_reader::BinaryReader;
use crate::{
  defs::{SWord, Word},
  fail::{RtErr, RtResult},
  term::{boxed::bignum::sign::Sign, lterm::Term, term_builder::TermBuilder},
};

///// Errors indicating a problem with External Term Format parser.
//#[derive(Debug)]
// pub enum ETFError {
//  ParseError(String),
//  ReadError(ReadError),
//}

#[repr(u8)]
#[allow(dead_code)]
enum Tag {
  ETF = 131,
  NewFloat = 70,
  BitBinary = 77,
  AtomCacheRef_ = 82,
  SmallInteger = 97,
  Integer = 98,
  Float = 99,
  AtomDeprecated = 100,
  Reference = 101,
  Port = 102,
  Pid = 103,
  SmallTuple = 104,
  LargeTuple = 105,
  Nil = 106,
  String = 107,
  List = 108,
  Binary = 109,
  SmallBig = 110,
  LargeBig = 111,
  NewFun = 112,
  Export = 113,
  NewReference = 114,
  SmallAtomDeprecated = 115,
  Map = 116,
  Fun = 117,
  AtomUtf8 = 118,
  SmallAtomUtf8 = 119,
}

fn module() -> &'static str {
  "external_term_format: "
}

fn fail<TermType: Copy>(msg: String) -> RtResult<TermType> {
  Err(RtErr::ETFParseError(msg))
}

/// Given a binary reader `r` parse term and return it, `heap` is used to
/// allocate space for larger boxed terms.
pub fn decode(r: &mut BinaryReader, tb: &mut TermBuilder) -> RtResult<Term> {
  let etf_tag = r.read_u8();
  if etf_tag != Tag::ETF as u8 {
    let msg = format!("{}Expected ETF tag byte 131, got {}", module(), etf_tag);
    return fail(msg);
  }
  decode_naked(r, tb)
}

/// Given an encoded term without ETF tag (131u8), read the term from `r` and
/// place boxed term parts on heap `heap`.
pub fn decode_naked(r: &mut BinaryReader, tb: &mut TermBuilder) -> RtResult<Term> {
  let term_tag = r.read_u8();
  match term_tag {
    x if x == Tag::List as u8 => decode_list(r, tb),

    x if x == Tag::String as u8 => decode_string(r, tb),

    x if x == Tag::AtomDeprecated as u8 => decode_atom_latin1(r, tb),

    x if x == Tag::SmallInteger as u8 => decode_u8(r, tb),

    x if x == Tag::Integer as u8 => decode_s32(r, tb),

    x if x == Tag::Nil as u8 => Ok(Term::nil()),

    x if x == Tag::LargeTuple as u8 => {
      let size = r.read_u32be() as Word;
      decode_tuple(r, size, tb)
    }

    x if x == Tag::SmallTuple as u8 => {
      let size = r.read_u8() as Word;
      decode_tuple(r, size, tb)
    }

    x if x == Tag::LargeBig as u8 => {
      let size = r.read_u32be() as Word;
      decode_big(r, size, tb)
    }

    x if x == Tag::SmallBig as u8 => {
      let size = r.read_u8() as Word;
      decode_big(r, size, tb)
    }

    x if x == Tag::Binary as u8 => decode_binary(r, tb),

    x if x == Tag::Map as u8 => {
      let size = r.read_u32be() as Word;
      decode_map(r, size, tb)
    }

    _ => {
      let msg = format!(
        "Don't know how to decode ETF value tag 0x{:x} ({})",
        term_tag, term_tag
      );
      fail(msg)
    }
  }
}

/// Given `size`, read digits for a bigint.
fn decode_big(r: &mut BinaryReader, size: Word, tb: &mut TermBuilder) -> RtResult<Term> {
  let sign = if r.read_u8() == 0 {
    Sign::Positive
  } else {
    Sign::Negative
  };
  let digits = r.read_bytes(size)?;
  // Determine storage size in words
  unsafe { Ok(tb.create_bignum_le(sign, digits)?) }
}

fn decode_binary(r: &mut BinaryReader, tb: &mut TermBuilder) -> RtResult<Term> {
  let n_bytes = r.read_u32be() as usize;
  if n_bytes == 0 {
    return Ok(Term::empty_binary());
  }

  let data = r.read_bytes(n_bytes)?;
  Ok(unsafe { tb.create_binary(&data)? })
}

/// Given arity, allocate a tuple and read its elements sequentially.
fn decode_tuple(
  r: &mut BinaryReader,
  size: usize,
  tb: &mut TermBuilder,
) -> RtResult<Term> {
  let tuple_builder = tb.create_tuple_builder(size)?;
  for i in 0..size {
    let elem = decode_naked(r, tb)?;
    unsafe { tuple_builder.set_element(i, elem) }
  }
  Ok(tuple_builder.make_term())
}

/// Given size, create a map of given size and read `size` pairs.
fn decode_map(r: &mut BinaryReader, size: usize, tb: &mut TermBuilder) -> RtResult<Term> {
  let mut mapb = tb.create_map_builder(size)?;
  for _i in 0..size {
    let key = decode_naked(r, tb)?;
    let val = decode_naked(r, tb)?;
    unsafe { mapb.add(key, val)? }
  }
  Ok(mapb.make_term())
}

fn decode_u8(r: &mut BinaryReader, tb: &mut TermBuilder) -> RtResult<Term> {
  let val = r.read_u8();
  Ok(tb.create_small_s(val as SWord))
}

fn decode_s32(r: &mut BinaryReader, tb: &mut TermBuilder) -> RtResult<Term> {
  let val = r.read_u32be() as i32;
  Ok(tb.create_small_s(val as SWord))
}

fn decode_atom_latin1(r: &mut BinaryReader, tb: &mut TermBuilder) -> RtResult<Term> {
  let sz = r.read_u16be();
  let val = r.read_str_latin1(sz as Word).unwrap();
  Ok(tb.create_atom_str(&val))
}

fn decode_list(r: &mut BinaryReader, tb: &mut TermBuilder) -> RtResult<Term> {
  let n_elem = r.read_u32be();
  if n_elem == 0 {
    return Ok(Term::nil());
  }

  let mut lb = tb.create_list_builder()?;
  for _i in 0..n_elem {
    let another = decode_naked(r, tb)?;
    unsafe {
      lb.append(another)?;
    }
  }

  // Decode tail, possibly a nil
  let tl = decode_naked(r, tb)?;
  unsafe { Ok(lb.make_term_with_tail(tl)) }
}

/// A string of bytes encoded as tag 107 (String) with 16-bit length.
fn decode_string(r: &mut BinaryReader, tb: &mut TermBuilder) -> RtResult<Term> {
  let n_elem = r.read_u16be();
  if n_elem == 0 {
    return Ok(Term::nil());
  }

  // Using mutability build list forward creating many cells and linking them
  let mut list_builder = tb.create_list_builder()?;

  for _i in 0..n_elem {
    let elem = r.read_u8();
    unsafe {
      let another = tb.create_small_s(elem as SWord);
      list_builder.append(another)?;
    }
  }

  Ok(list_builder.make_term())
}
