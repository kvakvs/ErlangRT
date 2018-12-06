use emulator::heap::Heap;
use fail::RtResult;
use rt_defs::storage_bytes_to_words;
use term::boxed::{BoxHeader, BOXTYPETAG_BIGINTEGER};

use core::ptr;
use num::bigint::BigInt;


#[allow(dead_code)]
pub struct Bignum {
  header: BoxHeader,

  /// The actual value. NOTE: Here we trust the storage class (BigInt)
  /// to manage the memory for its digits on the general application heap.
  // TODO: Not nice! Manage own data for Bignum.
  pub value: BigInt,
}

impl Bignum {
  const fn storage_size() -> usize {
    storage_bytes_to_words(core::mem::size_of::<Bignum>())
  }


  fn new(n_words: usize, value: BigInt) -> Bignum {
    Bignum {
      header: BoxHeader::new(BOXTYPETAG_BIGINTEGER, n_words),
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
