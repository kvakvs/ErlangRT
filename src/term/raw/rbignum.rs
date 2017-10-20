use defs;
use defs::Word;
use term::lterm::LTerm;
use term::primary;

use num;

/// Represents raw layout of bignum digits in memory.
/// TODO: Operate without copying.
pub struct BignumPtr {
  p: *mut Word,
}


fn module() -> &'static str { "raw::rbignum: " }


impl BignumPtr {
  /// How many words of heap we need to store the bignum.
  #[inline]
  pub fn storage_size(big: &num::BigInt) -> Word {
    (big.bits() + (defs::WORD_BITS - 1) / defs::WORD_BITS)
  }

  /// Given a pointer initialize a boxed value header here, hence unsafe.
  /// Return a `RawBignum` wrapper.
  pub unsafe fn create_at(p: *mut Word, big: &num::BigInt) -> BignumPtr {
    let storage_sz = BignumPtr::storage_size(big);
    match big.sign() {
      num::bigint::Sign::Minus =>
        *p = primary::header::make_bignum_neg_header_raw(storage_sz),
      num::bigint::Sign::Plus =>
        *p = primary::header::make_bignum_pos_header_raw(storage_sz),
      _ => panic!("{}create_at - no sign", module())
    }
    BignumPtr { p }
  }

  /// Given a pointer to an already initialized tuple, just return a wrapper.
//  pub fn from_pointer(p: *mut Word, arity: Word) -> RawBignum {
//    RawBignum { p }
//  }

//  pub unsafe fn arity(&self) -> Word {
//    primary::get_value(*self.p)
//  }

//  /// Zero-based set element function
//  pub unsafe fn set_element_base0(&self, i: Word, val: LTerm) {
//    assert!(i < self.arity());
//    *self.p.offset(i as isize + 1) = val.raw()
//  }

//  pub unsafe fn get_element(&self, i: Word) -> LTerm {
//    LTerm::from_raw(*self.p.offset(i as isize + 1))
//  }

  /// Box the `self.p` pointer into `LTerm`.
  pub fn make_bignum(&self) -> LTerm { LTerm::make_box(self.p) }
}
