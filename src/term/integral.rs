//! Type-safe Erlang integer used during code loading time.
//!
use defs::{SWord, WORD_BITS};

use num;
use num::ToPrimitive;


/// A type-safe way to represent an Erlang integer, which can be either small
/// enough to fit into a word, or a large one, stored as a BigInt. There is no
/// way (and was no need) to represent a small signed integer.
#[derive(Debug, Eq, PartialEq)]
pub enum Integral {
  Small(SWord),
  BigInt(num::BigInt),
}

impl Integral {

  pub fn from_big(big: num::BigInt) -> Integral {
    if big.bits() < WORD_BITS {
      return Integral::Small(big.to_isize().unwrap());
    }
    Integral::BigInt(big)
  }

}
