use crate::{
  defs::{ByteSize, WordSize},
  emulator::heap::Heap,
  fail::RtResult,
  term::boxed::{BoxHeader, BOXTYPETAG_BIGINTEGER},
};
use core::{mem::size_of, ptr};
use num::bigint::BigInt;


#[allow(dead_code)]
pub struct Bignum {
  header: BoxHeader,

  /// The actual value. NOTE: Here we trust the storage class (BigInt)
  /// to manage the memory for its digits on the general application heap.
  /// This impl stores bignum in dynamic heap with the num library
  // TODO: Not nice! Manage own data for Bignum.
  pub value: BigInt,
}

impl Bignum {
  const fn storage_size() -> WordSize {
    // This impl stores bignum in dynamic heap with the num library
    ByteSize::new(size_of::<Bignum>()).words_rounded_up()
  }


  fn new(bignum_size: WordSize, value: BigInt) -> Bignum {
    Bignum {
      header: BoxHeader::new(BOXTYPETAG_BIGINTEGER, bignum_size.words()),
      value,
    }
  }


  pub unsafe fn create_into(hp: &mut Heap, value: BigInt) -> RtResult<*mut Bignum> {
    let n_words = Bignum::storage_size();
    let this = hp.alloc::<Bignum>(n_words, false)?;

    ptr::write(this, Bignum::new(n_words, value));

    Ok(this)
  }
}
