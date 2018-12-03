//! Boxed package contains modules which represent different types of
//! terms in memory.

pub mod pid;
pub use self::pid::{RemotePid};

pub mod closure;
pub use self::closure::{Closure};

pub mod bignum;
pub use self::bignum::{Bignum};

pub mod import;
pub use self::import::{Import};

pub mod export;
pub use self::export::{Export};

pub mod binary;
pub use self::binary::{Binary};

pub mod tuple;
pub use self::tuple::{Tuple};

pub mod cons;
pub use self::cons::{Cons};
use rt_defs::*;

//
// Structure of a header word:
// [ Arity ... ] [ Header type: 3 bits ] [ Header tag: 3 bits ]
//

const HEADER_TAG_BITS: Word = 3;
const HEADER_TAG_MASK: Word = (1 << HEADER_TAG_BITS) - 1;

#[derive(Debug, Eq, PartialEq)]
pub enum BoxTypeTag {
  Tuple,
  Binary,
  BigInteger,
  ExternalPid,
  ExternalRef,
  ExternalPort,
  Closure,
  // A function object with frozen (captured) variable values
  Float,
  Import,
  Export,
}

/// Term header in memory, followed by corresponding data.
pub struct BoxHeader {
  header_word: Word
}

impl BoxHeader {
  pub fn new(t: BoxTypeTag, arity: Word) -> BoxHeader {
    BoxHeader {
      header_word: (arity << HEADER_TAG_BITS | t) << TERM_TAG_BITS | TermTag::Header
    }
  }


  pub const fn storage_size() -> Word { 1 }


  pub fn get_tag(self) -> BoxTypeTag {
    headerword_to_boxtype(self.header_word)
  }


  pub fn get_arity(self) -> Word {
    headerword_to_arity(self.header_word)
  }
}


/// For a header word value, extract bits with arity
pub fn headerword_to_arity(w: Word) -> Word {
  w >> TERM_TAG_BITS
}


pub fn headerword_to_boxtype(w: Word) -> BoxTypeTag {
  (w & TERM_TAG_MASK) as BoxTypeTag
}