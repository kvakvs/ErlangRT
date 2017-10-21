//! Heap object which stores an import - Mod, Fun, Arity and a bif flag.

use std::mem::size_of;

use defs::{Arity, WORD_BYTES, Word};
use emulator::heap::Heap;
use term::lterm::LTerm;
use term::primary::header;
use emulator::heap::heapobj::*;

// This should pack into 1+3 words if Arity has size less than 1 word
pub struct HOImport {
  class_ptr: *const HeapObjClass,
  m: LTerm,
  f: LTerm,
  arity: Arity,
  is_bif: bool,
}


static HOCLASS_IMPORT: HeapObjClass = HeapObjClass {
  obj_type: HeapObjType::Import,
  dtor: |p: *mut Word| {}
};


impl HOImport {
  #[inline]
  fn storage_size() -> usize {
    // Add 1 for header word
    1 + (size_of::<HOImport>() + WORD_BYTES - 1) / WORD_BYTES
  }

  pub fn place_into(hp: &mut Heap, m: LTerm, f: LTerm,
                    arity: Arity, is_bif: bool) -> LTerm {
    let nwords = HOImport::storage_size();
    let p = hp.allocate(nwords).unwrap();
    unsafe {
      *p = header::make_heapobj_header_raw(nwords);
      let inplace = p.offset(1) as *mut HOImport;
      (*inplace).class_ptr = &HOCLASS_IMPORT;
      (*inplace).m = m;
      (*inplace).f = f;
      (*inplace).arity = arity;
      (*inplace).is_bif = is_bif;
    }
    LTerm::make_box(p)
  }
}
