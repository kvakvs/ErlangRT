//! Type-safe Erlang integer used during code loading time.
use crate::{
  big,
  defs::{SWord, WORD_BITS},
  term::boxed,
};

/// This is only used during the load time.
/// A type-safe way to represent an Erlang integer, which can be either small
/// enough to fit into a word, or a large one, stored as a `Bignum` on a literal
/// heap. There is no way (and was no need) to represent a small signed integer.
#[derive(Debug, Eq, PartialEq)]
pub enum LtIntegral {
  Small(SWord),
  Big(*const boxed::Bignum),
}
