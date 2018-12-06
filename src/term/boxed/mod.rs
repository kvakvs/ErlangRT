//! Boxed package contains modules which represent different types of
//! terms in memory.

pub mod pid;
pub use self::pid::ExternalPid;

pub mod closure;
pub use self::closure::Closure;

pub mod bignum;
pub use self::bignum::Bignum;

pub mod import;
pub use self::import::Import;

pub mod export;
pub use self::export::Export;

pub mod binary;
pub use self::binary::Binary;

pub mod tuple;
pub use self::tuple::Tuple;

pub mod cons;
pub use self::cons::Cons;
use crate::rt_defs::*;
use crate::term::lterm::{TERMTAG_HEADER, TERM_TAG_BITS, TERM_TAG_MASK};

//
// Structure of a header word:
// [ Arity ... ] [ Header type: 3 bits ] [ Header tag: 3 bits ]
//

const HEADER_TAG_BITS: Word = 3;
#[allow(dead_code)]
const HEADER_TAG_MASK: Word = (1 << HEADER_TAG_BITS) - 1;

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
pub const BOXTYPETAG_BINARY: BoxTypeTag = BoxTypeTag(1);
pub const BOXTYPETAG_BIGINTEGER: BoxTypeTag = BoxTypeTag(2);
pub const BOXTYPETAG_EXTERNALPID: BoxTypeTag = BoxTypeTag(3);
pub const BOXTYPETAG_EXTERNALREF: BoxTypeTag = BoxTypeTag(4);
pub const BOXTYPETAG_EXTERNALPORT: BoxTypeTag = BoxTypeTag(5);

// A function object with frozen (captured) variable values
pub const BOXTYPETAG_CLOSURE: BoxTypeTag = BoxTypeTag(6);

pub const BOXTYPETAG_FLOAT: BoxTypeTag = BoxTypeTag(7);
pub const BOXTYPETAG_IMPORT: BoxTypeTag = BoxTypeTag(8);
pub const BOXTYPETAG_EXPORT: BoxTypeTag = BoxTypeTag(9);
pub const BOXTYPETAG_MAP: BoxTypeTag = BoxTypeTag(10);

/// Term header in memory, followed by corresponding data.
pub struct BoxHeader {
  header_word: Word,
}

impl BoxHeader {
  pub fn new(t: BoxTypeTag, arity: Word) -> BoxHeader {
    BoxHeader {
      header_word: (arity << HEADER_TAG_BITS | t.get()) << TERM_TAG_BITS | TERMTAG_HEADER.0,
    }
  }


  pub const fn storage_size() -> Word {
    1
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
  BoxTypeTag(w & TERM_TAG_MASK)
}
