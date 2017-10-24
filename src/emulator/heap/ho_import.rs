//! Heap object which stores an import - Mod, Fun, Arity and a bif flag.

use std::mem::size_of;

use fail::Hopefully;
use defs::{WORD_BYTES, Word};
use emulator::code::CodePtr;
use emulator::code_srv;
use emulator::heap::Heap;
use emulator::heap::heapobj::*;
use emulator::mfa::MFArity;
use term::lterm::LTerm;
use term::primary::header;


/// Heap object `HOImport` is placed on lit heap by the BEAM loader, VM would
/// deref it using boxed term pointer and feed to `code_srv` for resolution.
pub struct HOImport {
  pub class_ptr: *const HeapObjClass,
  pub mfarity: MFArity,
  pub is_bif: bool,
}


#[allow(const_err)]
static HOCLASS_IMPORT: HeapObjClass = HeapObjClass {
  obj_type: HeapObjType::Import,
  dtor: |_ptr: *mut Word| {},
  fmt_str: HOImport::fmt_str
};


impl HOImport {
  pub unsafe fn fmt_str(this0: *const Word) -> String {
    let this = this0 as *mut HOImport;
    let m = (*this).mfarity.m;
    let f = (*this).mfarity.f;
    let arity = (*this).mfarity.arity;
    format!("Import({}:{}/{})@{:p}", m, f, arity, this0)
  }


  #[inline]
  fn storage_size() -> usize {
    // Add 1 for header word
    1 + (size_of::<HOImport>() + WORD_BYTES - 1) / WORD_BYTES
  }

  pub fn place_into(hp: &mut Heap, mfarity: MFArity, is_bif: bool) -> LTerm {
    let nwords = HOImport::storage_size();
    let p = hp.allocate(nwords).unwrap();
    unsafe {
      *p = header::make_heapobj_header_raw(nwords);
      let inplace = p.offset(1) as *mut HOImport;
      (*inplace).class_ptr = &HOCLASS_IMPORT;
      (*inplace).mfarity = mfarity;
      (*inplace).is_bif = is_bif;
    }
    LTerm::make_box(p)
  }


  pub fn from_term(t: LTerm) -> *const HOImport {
    let p = t.box_ptr();
    unsafe { p.offset(1) as *const HOImport }
  }


  pub fn resolve(&self) -> Hopefully<CodePtr> {
    code_srv::lookup_and_load(&self.mfarity)
  }
}
