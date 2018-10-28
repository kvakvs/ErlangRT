use num::bigint::BigInt;

use emulator::heap::{Heap};
use fail::Hopefully;
use rt_defs::{storage_bytes_to_words};
//use rt_defs::{Word};
use term::boxed::BoxHeader;
use term::lterm::*;

use core::ptr;
use std::mem::size_of;


pub struct Bignum {
  header: BoxHeader,

  /// The actual value. NOTE: Here we trust the storage class (BigInt)
  /// to manage the memory for its digits on the general application heap.
  // TODO: Not nice! Manage own data for Bignum.
  pub value: BigInt,
}

impl Bignum {

  const fn storage_size() -> usize {
    storage_bytes_to_words(size_of::<Bignum>())
  }

  pub unsafe fn create_into(hp: &mut Heap,
                           value: BigInt) -> Hopefully<*mut Bignum>
  {
    let n_words = Bignum::storage_size();
    let this = hp.alloc_words::<Bignum>(n_words, false)?;

    ptr::write(this, Bignum::new(n_words, value));

    Ok(this)
  }


  pub fn make_term(this: *const Bignum) -> LTerm {
    LTerm::make_boxed(&this.header)
  }

}
