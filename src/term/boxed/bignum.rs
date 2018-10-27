use std::mem::size_of;
use std::ptr;
use num::bigint::BigInt;

use emulator::heap::{IHeap, Heap};
use fail::Hopefully;
use rt_defs::{WORD_BYTES, Word};
use term::classify::TermClass;
use term::lterm::*;

pub struct Bignum {
  header: BoxHeader,

  /// The actual value. NOTE: Here we trust `Vec<BigDigit>` to manage the
  /// memory for its digits on the general application heap.
  // TODO: Not nice! Manage own data for HOBignum.
  pub value: BigInt,
}

impl Bignum {
  pub fn make_term(this: *const Bignum) -> LTerm {
    LTerm::make_boxed(&this.header)
  }

}
