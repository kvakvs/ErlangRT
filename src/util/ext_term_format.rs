use defs::Word;
use defs;
use emulator::atom;
use emulator::heap::Heap;
use fail::{Hopefully, Error};
use term::lterm::LTerm;
//use term::raw::RawBignum;
use util::bin_reader::BinaryReader;

use num;
use num::ToPrimitive;

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
  AtomDeprecated = 100, // deprecated?
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
  SmallAtomDeprecated = 115, // deprecated?
  Map = 116,
  Fun = 117,
  AtomUtf8 = 118,
  SmallAtomUtf8 = 119,
}

fn module() -> &'static str { "external_term_format: " }


fn fail(msg: String) -> Hopefully<LTerm> {
  Err(Error::ReadExternalTerm(msg))
}


/// Given a binary reader `r` parse term and return it, `heap` is used to
/// allocate space for larger boxed terms.
#[inline]
pub fn decode(r: &mut BinaryReader, heap: &mut Heap) -> Hopefully<LTerm> {
  let etf_tag = r.read_u8();
  if etf_tag != Tag::ETF as u8 {
    let msg = format!("{}Expected ETF tag byte 131, got {}", module(), etf_tag);
    return fail(msg)
  }
  decode_naked(r, heap)
}


/// Given an encoded term without ETF tag (131u8), read the term from `r` and
/// place boxed term parts on heap `heap`.
pub fn decode_naked(r: &mut BinaryReader, heap: &mut Heap) -> Hopefully<LTerm> {
  let term_tag = r.read_u8();
  match term_tag {
    x if x == Tag::List as u8 => decode_list(r, heap),

    x if x == Tag::String as u8 => decode_string(r, heap),

    x if x == Tag::AtomDeprecated as u8 => decode_atom_latin1(r),

    x if x == Tag::SmallInteger as u8 => decode_u8(r),

    x if x == Tag::Nil as u8 => Ok(LTerm::nil()),

    x if x == Tag::LargeTuple as u8 => {
      let size = r.read_u32be() as Word;
      decode_tuple(r, heap, size)
    },

    x if x == Tag::SmallTuple as u8 => {
      let size = r.read_u8() as Word;
      decode_tuple(r, heap, size)
    },

    x if x == Tag::LargeBig as u8 => {
      let size = r.read_u32be() as Word;
      decode_big(r, heap, size)
    },

    x if x == Tag::SmallBig as u8 => {
      let size = r.read_u8() as Word;
      decode_big(r, heap, size)
    },

    _ => {
      let msg = format!("{}Don't know how to decode ETF value tag {}",
                        module(), term_tag);
      fail(msg)
    }
  }
}


/// Given `size`, read digits for a bigint.
fn decode_big(r: &mut BinaryReader, heap: &mut Heap,
                size: Word) -> Hopefully<LTerm> {
  let sign = if r.read_u8() == 0 { num::bigint::Sign::Plus }
      else { num::bigint::Sign::Minus };
  let digits = r.read_bytes(size).unwrap();
  let big = num::BigInt::from_bytes_le(sign, &digits);

  // Assert that the number fits into small
  if big.bits() < defs::WORD_BITS - 4 {
    return Ok(LTerm::make_small_s(big.to_isize().unwrap()))
  }

  // Determine storage size in words
  let rbig = heap.allocate_big(&big).unwrap();
  Ok(rbig.make_bignum())
}


/// Given arity, allocate a tuple and read its elements sequentially.
fn decode_tuple(r: &mut BinaryReader, heap: &mut Heap,
                size: Word) -> Hopefully<LTerm> {
  let rtuple = heap.allocate_tuple(size).unwrap();
  for i in 0..size {
    let elem = decode_naked(r, heap).unwrap();
    unsafe { rtuple.set_element_base0(i, elem) }
  }
  Ok(rtuple.make_tuple())
}


fn decode_u8(r: &mut BinaryReader) -> Hopefully<LTerm> {
  let val = r.read_u8();
  Ok(LTerm::make_small_u(val as Word))
}


fn decode_atom_latin1(r: &mut BinaryReader) -> Hopefully<LTerm> {
  let sz = r.read_u16be();
  let val = r.read_str_latin1(sz as Word).unwrap();
  Ok(atom::from_str(&val))
}


fn decode_list(r: &mut BinaryReader, heap: &mut Heap) -> Hopefully<LTerm> {
  let n_elem = r.read_u32be();
  if n_elem == 0 {
    return Ok(LTerm::nil())
  }

  // Using mutability build list forward creating many cells and linking them
  let mut cell = heap.allocate_cons().unwrap();
  let cell0 = cell.clone();

  unsafe {
    for i in 0..n_elem {
      let elem = decode_naked(r, heap).unwrap();
      cell.set_hd(elem);

      if i + 1 < n_elem {
        let new_cell = heap.allocate_cons().unwrap();
        cell.set_tl(new_cell.make_cons());
        cell = new_cell;
      }
    }

    cell.set_tl(LTerm::nil());
  } // unsafe

  Ok(cell0.make_cons())
}


/// A string of bytes encoded as tag 107 (String) with 16-bit length.
fn decode_string(r: &mut BinaryReader, heap: &mut Heap) -> Hopefully<LTerm> {
  let n_elem = r.read_u16be();
  if n_elem == 0 {
    return Ok(LTerm::nil())
  }

  // Using mutability build list forward creating many cells and linking them
  let mut cell = heap.allocate_cons().unwrap();
  let cell0 = cell.clone();

  unsafe {
    for i in 0..n_elem {
      let elem = r.read_u8();
      cell.set_hd(LTerm::make_small_u(elem as usize));

      if i + 1 < n_elem {
        let new_cell = heap.allocate_cons().unwrap();
        cell.set_tl(new_cell.make_cons());
        cell = new_cell;
      }
    }

    cell.set_tl(LTerm::nil());
  } // unsafe

  Ok(cell0.make_cons())
}
