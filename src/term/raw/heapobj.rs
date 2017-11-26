//! A heap object is a block of heap memory tagged with:
//!
//! 1. (p+0) A header tag with arity which delimits the object size.
//! 2. (p+1) A `HeapObjClass` pointer which is used to call methods on a heap object.
//! 3. (p+2...) The data
use rt_defs::Word;
use term::primary::header;
use term::classify::TermClass;
use term::lterm::LTerm;
use term::lterm::aspect_boxed::{BoxedAspect};


pub enum HeapObjType {
  /// Type maps to `term::raw::ho_import::HOImport` heap obj class.
  Import,
  Export,
  Binary,
  Bignum,
  Closure,
}


/// Everything on heap has a header word, this is to simplify traversing the
/// heap by GC. Now more complex objects also have a class pointer which
/// allows accessing features for that object.
pub struct HeapObjHeader {
  pub header_word: Word,
  pub class_ptr: *const HeapObjClass,
}


impl HeapObjHeader {
  /// Initialise heap object fields
  pub fn new(n_words: Word, cls: *const HeapObjClass) -> HeapObjHeader {
    HeapObjHeader {
      header_word: header::make_heapobj_header_raw(n_words),
      class_ptr: cls,
    }
  }
}


/// Used to identify heap object type on heap
pub struct HeapObjClass {
  pub obj_type: HeapObjType,
  pub dtor: unsafe fn(this: *mut Word) -> (),
  pub fmt_str: unsafe fn(this: *const Word) -> String,
  /// For comparisons, a class for this term or `TermClass::Special_`
  pub term_class: TermClass,
}


#[inline]
pub unsafe fn heapobj_from_term<HOClassT>(t: LTerm, hoclass: *const HeapObjClass)
  -> Option<*const HOClassT>
{
  if !t.is_box() {
    return None;
  }

  // Check whether the object is HOClosure
  let boxp = t.box_ptr();
  let hdr = boxp as *const HeapObjHeader;
  if (*hdr).class_ptr != hoclass {
    return None;
  }

  Some(boxp as *const HOClassT)
}