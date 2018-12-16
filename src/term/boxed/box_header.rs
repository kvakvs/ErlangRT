use crate::{
  defs::*,
  term::lterm::{TERMTAG_HEADER, TERM_TAG_BITS},
};

//
// Structure of a header word:
// [ Arity ... ] [ Header type: 3 bits ] [ Header tag: 3 bits ]
//

#[derive(Debug, Eq, PartialEq)]
pub struct BoxTypeTag(Word);

impl BoxTypeTag {
  #[inline]
  pub fn get(self) -> Word {
    let BoxTypeTag(t) = self;
    t
  }
}

pub const BOXTYPETAG_TUPLE: BoxTypeTag = BoxTypeTag(0);
pub const BOXTYPETAG_BIGINTEGER: BoxTypeTag = BoxTypeTag(1);
pub const BOXTYPETAG_EXTERNALPID: BoxTypeTag = BoxTypeTag(2);
pub const BOXTYPETAG_EXTERNALREF: BoxTypeTag = BoxTypeTag(3);
pub const BOXTYPETAG_EXTERNALPORT: BoxTypeTag = BoxTypeTag(4);

// A function object with frozen (captured) variable values
pub const BOXTYPETAG_CLOSURE: BoxTypeTag = BoxTypeTag(5);

pub const BOXTYPETAG_FLOAT: BoxTypeTag = BoxTypeTag(6);
pub const BOXTYPETAG_IMPORT: BoxTypeTag = BoxTypeTag(7);
pub const BOXTYPETAG_EXPORT: BoxTypeTag = BoxTypeTag(8);
pub const BOXTYPETAG_MAP: BoxTypeTag = BoxTypeTag(9);

pub const BOXTYPETAG_BINARY: BoxTypeTag = BoxTypeTag(10);
// max 15 (1 << HEADER_TAG_BITS)

const BOXTYPE_TAG_BITS: Word = 4;
#[allow(dead_code)]
const BOXTYPE_TAG_MASK: Word = (1 << BOXTYPE_TAG_BITS) - 1;


/// Term header in memory, followed by corresponding data.
pub struct BoxHeader {
  header_word: Word,
}

impl BoxHeader {
  pub fn new(t: BoxTypeTag, arity: Word) -> BoxHeader {
    let mut header_word = arity;
    header_word <<= BOXTYPE_TAG_BITS;
    header_word |= t.get();
    header_word <<= TERM_TAG_BITS;
    header_word |= TERMTAG_HEADER.get();
    BoxHeader { header_word }
  }


  pub const fn storage_size() -> WordSize {
    WordSize::new(1)
  }


  pub fn get_tag(&self) -> BoxTypeTag {
    headerword_to_boxtype(self.header_word)
  }


  pub fn get_arity(&self) -> Word {
    headerword_to_arity(self.header_word)
  }
}


/// For a header word value, extract bits with arity
pub fn headerword_to_arity(w: Word) -> Word {
  w >> TERM_TAG_BITS
}


pub const fn headerword_to_boxtype(w: Word) -> BoxTypeTag {
  BoxTypeTag((w >> TERM_TAG_BITS) & BOXTYPE_TAG_MASK)
}
