//! A heap object is a block of heap memory tagged with:
//!
//! 1. (p+0) A header tag with arity which delimits the object size.
//! 2. (p+1) A `HeapObjClass` pointer which is used to call methods on a heap object.
//! 3. (p+2...) The data
use defs::Word;

pub enum HeapObjType {
  /// Maps to `emulator::heap::ho_import::HOImport`
  Import,
}

/// Used to identify heap object type on heap
pub struct HeapObjClass {
  pub obj_type: HeapObjType,
  pub dtor: fn(this: *mut Word) -> (),
  pub fmt_str: unsafe fn(this: *const Word) -> String,
}
