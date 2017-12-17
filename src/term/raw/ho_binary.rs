//! Heap object which stores a binary or a reference to a refcounted binary.

use std::mem::size_of;
use std::ptr;

use rt_defs::heap::{IHeap};
use rt_defs::{WORD_BYTES, Word};
use emulator::heap::Heap;
use term::raw::heapobj::*;
use fail::Hopefully;
use term::classify::TermClass;
use term::lterm::*;


pub enum HOBinaryType {
  Heap,
  //Refcounted,
  //Subbinary,
}


/// Heap object `HOBinary` is placed on heap by the VM and might transform 
/// itself to contain binary either locally or refer to it
#[allow(dead_code)]
pub struct HOBinary {
  hobj: HeapObjHeader,
  pub n_bytes: Word,
  pub flavour: HOBinaryType,
}


#[allow(const_err)]
static HOCLASS_BINARY: HeapObjClass = HeapObjClass {
  obj_type: HeapObjType::Binary,
  dtor: HOBinary::dtor,
  fmt_str: HOBinary::fmt_str,
  term_class: TermClass::Binary,
};


impl HOBinary {
  /// Destructor.
  pub unsafe fn dtor(this0: *mut Word) {
    let this = this0 as *mut HOBinary;
    match (*this).flavour {
      HOBinaryType::Heap => {},
    }
  }


  /// Format as a string.
  pub unsafe fn fmt_str(this0: *const Word) -> String {
    let this = this0 as *mut HOBinary;
    match (*this).flavour {
      HOBinaryType::Heap => {
        format!("HeapBin({} bytes)", (*this).n_bytes)
      }
    }
  }


  #[inline]
  fn storage_size(n_bytes: Word) -> usize {
    // Add 1 for header word
    (size_of::<HOBinary>() + n_bytes + WORD_BYTES - 1) / WORD_BYTES
  }


  /// Allocate space on heap for `n_bytes` and initialize the fields.
  /// A pointer to binary is returned which manages heap placement automatically
  /// (i.e. heapbin or procbin etc, are used automatically).
  pub unsafe fn place_into(hp: &mut Heap,
                           n_bytes: Word) -> Hopefully<*mut HOBinary>
  {
    let n_words = HOBinary::storage_size(n_bytes);
    let this = hp.heap_allocate(n_words, false)? as *mut HOBinary;

    ptr::write(this,
               HOBinary {
                 hobj: HeapObjHeader::new(n_words, &HOCLASS_BINARY),
                 n_bytes,
                 flavour: HOBinaryType::Heap,
               });

    Ok(this)
  }


  /// Given a term, unbox it and convert to a `HOBinary` const pointer.
  /// Returns None if not a binary.
  #[inline]
  pub unsafe fn from_term(t: LTerm) -> Hopefully<*const HOBinary> {
    heapobj_from_term::<HOBinary>(t, &HOCLASS_BINARY)
  }


  /// Given a term, unbox it and convert to a `HOBinary` mut pointer.
//  #[inline]
//  pub fn from_term_mut(t: LTerm) -> *const HOBinary {
//    let p = t.box_ptr_mut();
//    p as *mut HOBinary
//  }


  /// Create a boxed term. NOTE: There is no `self`, this is a raw pointer.
  #[inline]
  pub fn make_term(this: *const HOBinary) -> LTerm {
    make_box(this as *const Word)
  }


  /// Given a byte array, copy it to the binary's memory (depending on
  /// the binary type).
  pub unsafe fn store(this: *mut HOBinary, data: &[u8]) {
    let data_len = data.len();
    if data_len == 0 {
      return
    }

    assert!(data_len <= (*this).n_bytes,
            "The data ({} b) won't fit into binary (size {} b)",
            data_len, (*this).n_bytes);

    // Take next word after the record end, that'll be first data word.
    let bin_bytes = this.offset(1) as *mut u8;
    ptr::copy_nonoverlapping(&data[0], bin_bytes, data_len);
  }

}
