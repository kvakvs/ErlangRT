use fail::{Hopefully, Error};
use term::lterm::LTerm;
use util::bin_reader::BinaryReader;
use emulator::heap::Heap;

#[repr(u8)]
enum Tag {
  ETF = 131
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
    _ => {
      let msg = format!("{}Unknown ETF value tag {}", module(), term_tag);
      fail(msg)
    }
  }
}
