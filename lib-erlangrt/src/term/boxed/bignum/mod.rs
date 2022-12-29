use core::{mem::size_of, ptr};

use crate::{
  big,
  defs::{self, SizeBytes, SizeWords},
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
      BoxHeader, BOXTYPETAG_BIGINTEGER,
    },
    classify,
    *,
  },
};

use self::sign::*;
use crate::{
  emulator::heap::{AllocInit, THeap},
  term::boxed::{bignum, endianness::Endianness},
};

pub mod endianness;
pub mod sign;

pub const BIG_DIGIT_SIZE: usize = defs::WORD_BYTES;
pub type Digit = usize;

#[allow(dead_code)]
pub struct Bignum {
  header: BoxHeader,

  /// Negative size points that the number is negative.
  pub size: isize,
  /// First limb of digits is here, remaining digits follow in the memory after
  pub digits: Digit,
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
  const fn storage_size() -> SizeWords {
    // This impl stores bignum in dynamic heap with the num library
    SizeBytes::new(size_of::<Bignum>()).get_words_rounded_up()
  }

  /// Create bignum for one isize
  pub fn with_isize(hp: &mut dyn THeap, val: isize) -> RtResult<*mut Bignum> {
    let (sign, positive_val) = Sign::split(val);

    // Create slice of one limb
    let limbs = unsafe {
      let data = &positive_val as *const usize;
      core::slice::from_raw_parts(data, 1)
    };
    unsafe { Self::create_into(hp, sign, limbs) }
  }

  /// Given an array of bytes with little-endian order, create a bignum on the
  /// provided heap, or fail.
  pub unsafe fn create_le(
    hp: &mut dyn THeap,
    sign: bignum::sign::Sign,
    digits: Vec<u8>,
  ) -> RtResult<Term> {
    let limbs = big::make_limbs_from_bytes(Endianness::Little, digits);
    let p = Self::create_into(hp, sign, &limbs)?;
    Ok(Term::make_boxed(p))
  }

  /// Consume bytes as either big- or little-endian stream, and build a big
  /// integer on heap.
  pub unsafe fn create_into(
    hp: &mut dyn THeap,
    sign: Sign,
    limbs: &[Digit],
  ) -> RtResult<*mut Self> {
    let n_words = Self::storage_size();
    let this = hp.alloc(n_words, AllocInit::Uninitialized)? as *mut Self;

    this.write(Self {
      header: BoxHeader::new::<Self>(n_words),
      size: if sign == Sign::Negative {
        -(limbs.len() as isize)
      } else {
        limbs.len() as isize
      },
      digits: 0,
    });
    ptr::copy_nonoverlapping(
      limbs.as_ptr(),
      &mut (*this).digits as *mut Digit,
      limbs.len(),
    );

    Ok(this)
  }

  pub fn get_digits(&self) -> &[Digit] {
    unsafe { core::slice::from_raw_parts(&self.digits as *const Digit, self.get_size()) }
  }

  pub fn is_negative(&self) -> bool {
    self.size < 0
  }

  /// Return how many digits are stored (abs value of self.size)
  #[inline]
  pub fn get_size(&self) -> usize {
    if self.size >= 0 {
      self.size as usize
    } else {
      -self.size as usize
    }
  }

  /// Return how many bytes are used to store the digits. Multiple of word size.
  #[inline]
  pub fn get_byte_size(&self) -> SizeBytes {
    SizeBytes::new(self.get_size() * BIG_DIGIT_SIZE)
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
}
