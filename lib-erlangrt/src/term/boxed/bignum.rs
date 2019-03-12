use crate::{
  big,
  defs::{BitSize, ByteSize, WordSize},
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
use core::mem::size_of;

#[allow(dead_code)]
pub struct Bignum {
  header: BoxHeader,

  /// Contains same data as `big::Big` but the digits are stored in process
  /// heap memory following the `Bignum` header, starting at `digits` field.
  pub size: BitSize,
  pub digits: usize,
}

impl TBoxed for Bignum {
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

  pub unsafe fn create_into(hp: &mut Heap, value: big::Big) -> RtResult<*mut Bignum> {
    unimplemented!("bignum::create_into")
    //    let n_words = Bignum::storage_size();
    //    let this = hp.alloc::<Bignum>(n_words, false)?;
    //
    //    ptr::write(this, Bignum::new(n_words, value));
    //
    //    Ok(this)
  }

  #[allow(dead_code)]
  pub unsafe fn const_from_term(t: Term) -> RtResult<*const Self> {
    helper_get_const_from_boxed_term::<Self>(
      t,
      BOXTYPETAG_BIGINTEGER,
      RtErr::BoxedIsNotABigint,
    )
  }

  #[allow(dead_code)]
  pub unsafe fn mut_from_term(t: Term) -> RtResult<*mut Self> {
    helper_get_mut_from_boxed_term::<Self>(
      t,
      BOXTYPETAG_BIGINTEGER,
      RtErr::BoxedIsNotABigint,
    )
  }

  pub fn is_negative(&self) -> bool {
    unimplemented!("is_negative")
  }
}
