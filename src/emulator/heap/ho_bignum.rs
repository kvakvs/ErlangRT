//! Heap object which stores a bignum.

use std::mem::{size_of, swap, forget};
//use std::ptr;
use num::bigint::BigInt;

use defs::{WORD_BYTES, Word};
use emulator::heap::Heap;
use emulator::heap::heapobj::*;
use fail::Hopefully;
use term::lterm::LTerm;
use term::primary::header;


/// Heap object `HOBignum` is placed on heap by the VM and contains a signed
/// big integer. Bignum digits are stored after the bignum record.
pub struct HOBignum {
  pub header_word: Word,
  pub class_ptr: *const HeapObjClass,

  /// The actual value. NOTE: Here we trust `Vec<BigDigit>` to manage the
  /// memory for its digits on the general application heap.
  // TODO: Not nice! Manage own data for HOBignum.
  pub value: BigInt,
}


#[allow(const_err)]
static HOCLASS_BIGNUM: HeapObjClass = HeapObjClass {
  obj_type: HeapObjType::Binary,
  dtor: HOBignum::dtor,
  fmt_str: HOBignum::fmt_str,
};


impl HOBignum {
  /// Destructor.
  pub unsafe fn dtor(_this0: *mut Word) {
    //let this = this0 as *mut HOBignum;
  }


  /// Format as a string.
  pub unsafe fn fmt_str(this0: *const Word) -> String {
    let this = this0 as *mut HOBignum;
    format!("Big({})", &(*this).value)
  }


  #[inline]
  fn storage_size() -> usize {
    (size_of::<HOBignum>() + WORD_BYTES - 1) / WORD_BYTES
  }


  /// Allocate space on heap for `n_bytes` and initialize the fields.
  /// A pointer to binary is returned which manages heap placement automatically
  /// (i.e. heapbin or procbin etc, are used automatically).
  pub unsafe fn place_into(hp: &mut Heap,
                           mut value: BigInt) -> Hopefully<*mut HOBignum>
  {
    let n_words = HOBignum::storage_size();
    let this = hp.allocate(n_words)? as *mut HOBignum;

    (*this).header_word = header::make_heapobj_header_raw(n_words);
    (*this).class_ptr = &HOCLASS_BIGNUM;
//    (*this).value = value;
    swap(&mut (*this).value, &mut value);
    forget(value);

    // Take next word after the record end, that'll be first data word.
//    let bin_bytes = this.offset(1) as *mut u8;
//    ptr::copy_nonoverlapping(&data[0], bin_bytes, data_len);

    return Ok(this)
  }


  /// Given a term, unbox it and convert to a `HOBignum` const pointer.
//  #[inline]
//  pub fn from_term(t: LTerm) -> *const HOBignum {
//    let p = t.box_ptr();
//    p as *const HOBignum
//  }


  /// Given a term, unbox it and convert to a `HOBignum` mut pointer.
//  #[inline]
//  pub fn from_term_mut(t: LTerm) -> *const HOBignum {
//    let p = t.box_ptr_mut();
//    p as *mut HOBignum
//  }


  #[inline]
  pub fn make_term(this: *const HOBignum) -> LTerm {
    LTerm::make_box(this as *const Word)
  }

}
