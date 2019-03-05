use crate::{
  defs::{ByteSize, WordSize},
  emulator::heap::Heap,
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
      BoxHeader, BOXTYPETAG_BIGINTEGER,
    },
    classify,
    lterm::*,
  },
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

impl TBoxed for BigInt {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_NUMBER
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_BIGINTEGER
  }
}

impl Bignum {
  const fn storage_size() -> WordSize {
    // This impl stores bignum in dynamic heap with the num library
    ByteSize::new(size_of::<Bignum>()).get_words_rounded_up()
  }

  fn new(bignum_size: WordSize, value: BigInt) -> Bignum {
    Bignum {
      header: BoxHeader::new::<BigInt>(bignum_size.words()),
      value,
    }
  }

  pub unsafe fn create_into(hp: &mut Heap, value: BigInt) -> RtResult<*mut Bignum> {
    let n_words = Bignum::storage_size();
    let this = hp.alloc::<Bignum>(n_words, false)?;

    ptr::write(this, Bignum::new(n_words, value));

    Ok(this)
  }

  #[allow(dead_code)]
  pub unsafe fn const_from_term(t: LTerm) -> RtResult<*const Self> {
    helper_get_const_from_boxed_term::<Self>(
      t,
      BOXTYPETAG_BIGINTEGER,
      RtErr::BoxedIsNotABigint,
    )
  }

  #[allow(dead_code)]
  pub unsafe fn mut_from_term(t: LTerm) -> RtResult<*mut Self> {
    helper_get_mut_from_boxed_term::<Self>(
      t,
      BOXTYPETAG_BIGINTEGER,
      RtErr::BoxedIsNotABigint,
    )
  }
}
