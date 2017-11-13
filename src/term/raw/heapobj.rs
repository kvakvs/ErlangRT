//! A heap object is a block of heap memory tagged with:
//!
//! 1. (p+0) A header tag with arity which delimits the object size.
//! 2. (p+1) A `HeapObjClass` pointer which is used to call methods on a heap object.
//! 3. (p+2...) The data
use defs::Word;
use term::primary::header;
use term::classify::TermClass;


pub enum HeapObjType {
  /// Maps to `emulator::heap::ho_import::HOImport`
  Import,
  Binary,
  //Bignum,
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
