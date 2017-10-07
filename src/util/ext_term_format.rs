use defs::Word;
use emulator::atom;
use emulator::heap::Heap;
use fail::{Hopefully, Error};
use term::lterm::LTerm;
use util::bin_reader::BinaryReader;

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
#[inline(always)]
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
    x if x == Tag::AtomDeprecated as u8 => decode_atom_latin1(r),
    _ => {
      let msg = format!("{}Don't know how to decode ETF value tag {}",
                        module(), term_tag);
      fail(msg)
    }
  }
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

  unsafe {
    for i in 0..n_elem {
      cell.set_hd(decode_naked(r, heap).unwrap());

      if i < n_elem {
        let new_cell = heap.allocate_cons().unwrap();
        cell.set_tl(new_cell.make_cons());
        cell = new_cell;
      }
    }

    cell.set_tl(LTerm::nil());
  } // unsafe

  Ok(cell.make_cons())
}
