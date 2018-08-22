//! Heap object which stores an export - a stateless pointer to some function
//! by its M:F/Arity and created with a `fun module:name/0` expression.
//! Do not import this file directly, use `use term::raw::*;` instead.

use std::mem::size_of;
use std::ptr;

use emulator::export::Export;
use emulator::heap::Heap;
use emulator::mfa::MFArity;
use fail::Hopefully;
use rt_defs::heap::IHeap;
use rt_defs::{WORD_BYTES, Word};
use term::classify::TermClass;
use term::lterm::*;
use term::raw::heapobj::*;


/// Heap object `HOExport` is placed on heap.
#[allow(dead_code)]
pub struct HOExport {
  pub hobj: HeapObjHeader,
  pub exp: Export,
}


#[allow(const_err)]
static HOCLASS_EXPORT: HeapObjClass = HeapObjClass {
  obj_type: HeapObjType::Export,
  dtor: HOExport::dtor,
  fmt_str: HOExport::fmt_str,
  term_class: TermClass::Special_,
};


impl HOExport {
  /// Destructor.
  pub unsafe fn dtor(_this: *mut Word) {}


  pub unsafe fn fmt_str(this0: *const Word) -> String {
    let this = this0 as *mut HOExport;
    format!("Export({})", (*this).exp.mfa)
  }


  const STRUCT_SIZE: usize = (size_of::<HOExport>() + WORD_BYTES - 1) / WORD_BYTES;


  #[inline]
  fn storage_size() -> usize { HOExport::STRUCT_SIZE }


  fn new(n_words: usize, mfa: &MFArity) -> HOExport {
    HOExport {
      hobj: HeapObjHeader::new(n_words, &HOCLASS_EXPORT),
      exp: Export::new(*mfa),
    }
  }


  #[allow(dead_code)]
  pub unsafe fn place_into(hp: &mut Heap,
                           mfa: &MFArity) -> Hopefully<LTerm>
  {
    let n_words = HOExport::storage_size();
    let this = hp.heap_allocate(n_words, false)? as *mut HOExport;

    ptr::write(this, HOExport::new(n_words, mfa));
    Ok(make_box(this as *const Word))
  }


  #[inline]
  pub unsafe fn from_term(t: LTerm) -> Hopefully<*const HOExport> {
    heapobj_from_term::<HOExport>(t, &HOCLASS_EXPORT)
  }


//  /// Create a boxed term. NOTE: There is no `self`, this is a raw pointer.
//  #[inline]
//  pub fn make_term(this: *const HOExport) -> LTerm {
//    make_box(this as *const Word)
//  }
}
