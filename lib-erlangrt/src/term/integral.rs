//! Type-safe Erlang integer used during code loading time.
use crate::{
  big,
  defs::{SWord, WORD_BITS},
};

/// A type-safe way to represent an Erlang integer, which can be either small
/// enough to fit into a word, or a large one, stored as a `BigInt`. There is no
/// way (and was no need) to represent a small signed integer.
#[derive(Debug, Eq, PartialEq)]
pub enum Integral {
  Small(SWord),
  BigInt(big::Big),
}

impl Integral {
  pub fn from_big(big_val: big::Big) -> Integral {
    if big_val.get_size().bit_count < (WORD_BITS - 1) {
      return Integral::Small(big_val.to_isize().unwrap());
    }
    Integral::BigInt(big_val)
  }
}
