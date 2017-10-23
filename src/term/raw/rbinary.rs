//! A mutable and const pointers to binary on heap.
//! At this moment only heapbins are present, stored entirely on the owning
//! process heap.
//!
use defs::{Word, WORD_BYTES};
use term::lterm::LTerm;
use term::primary::header;

use std::ptr;


pub enum BinaryPtrMut { Ptr(*mut Word) }

pub enum BinaryPtr { Ptr(*const Word) }


impl BinaryPtrMut {

  #[inline]
  pub fn from_pointer(p: *mut Word) -> BinaryPtrMut {
    BinaryPtrMut::Ptr(p)
  }


  /// Given allocated heap space, set up header words to look like a binary.
  pub unsafe fn create_at(p: *mut Word, nbytes: Word) -> BinaryPtrMut {
    let nwords = storage_size(nbytes);
    *p = header::make_heapbin_header_raw(nwords);
    *(p.offset(1)) = nbytes;
    BinaryPtrMut::from_pointer(p)
  }


  pub fn make_term(&self) -> LTerm {
    let BinaryPtrMut::Ptr(p) = *self;
    LTerm::make_box(p)
  }


  /// Given a byte array, copy it to binary memory.
  pub unsafe fn store(&self, data: &Vec<u8>) {
    let data_len = data.len();
    if data_len == 0 {
      return
    }

    let BinaryPtrMut::Ptr(p) = *self;

    let bin_bytes = *p.offset(1);
    assert!(data_len <= bin_bytes,
            "The data ({} b) won't fit into binary (size {} b)",
            data_len, bin_bytes);

    // skip header word and nbytes word
    let dst = p.offset(2) as *mut u8;
    ptr::copy_nonoverlapping(&data[0], dst, data_len);
  }

}


/// Return storage size in Words to store the requested amount of bytes.
#[inline]
pub fn storage_size(nbytes: Word) -> Word {
  // 1 extra word for byte count, and round up to the nearest word
  1 + (nbytes + WORD_BYTES - 1) / WORD_BYTES
}
