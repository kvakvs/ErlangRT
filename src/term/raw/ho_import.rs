//! Heap object which stores an import - Mod, Fun, Arity and a bif flag.
//! Do not import this file directly, use `use term::raw::*;` instead.

use std::mem::size_of;
use std::ptr;

use bif::{BifFn, find_bif};
use emulator::code::CodePtr;
use emulator::code_srv::CodeServer;
use emulator::heap::Heap;
use emulator::mfa::MFArity;
use fail::Hopefully;
use emulator::heap::IHeap;
use rt_defs::{WORD_BYTES, Word};
use term::classify::TermClass;
use term::lterm::*;
use term::raw::heapobj::*;


/// Heap object `HOImport` is placed on lit heap by the BEAM loader, VM would
/// deref it using boxed term pointer and feed to `code_srv` for resolution.
#[allow(dead_code)]
pub struct HOImport {
  hobj: HeapObjHeader,
  pub mfarity: MFArity,
  pub is_bif: bool,
}


#[allow(const_err)]
static HOCLASS_IMPORT: HeapObjClass = HeapObjClass {
  obj_type: HeapObjType::Import,
  dtor: HOImport::dtor,
  fmt_str: HOImport::fmt_str,
  term_class: TermClass::Special_,
};


impl HOImport {

  /// Destructor.
  pub unsafe fn dtor(_this: *mut Word) {
  }


  pub unsafe fn fmt_str(this0: *const Word) -> String {
    let this = this0 as *mut HOImport;
    let m = (*this).mfarity.m;
    let f = (*this).mfarity.f;
    let arity = (*this).mfarity.arity;
    format!("Import({}:{}/{})@{:p}", m, f, arity, this0)
  }


  #[inline]
  fn storage_size() -> usize {
    (size_of::<HOImport>() + WORD_BYTES - 1) / WORD_BYTES
  }

  pub unsafe fn place_into(hp: &mut Heap,
                           mfarity: MFArity,
                           is_bif: bool) -> Hopefully<LTerm>
  {
    let n_words = HOImport::storage_size();
    let this = hp.heap_allocate(n_words, false)? as *mut HOImport;

    ptr::write(this,
               HOImport {
                 hobj: HeapObjHeader::new(n_words, &HOCLASS_IMPORT),
                 mfarity,
                 is_bif,
               });
    Ok(make_box(this as *const Word))
  }


  #[inline]
  pub unsafe fn from_term(t: LTerm) -> Hopefully<*const HOImport> {
    heapobj_from_term::<HOImport>(t, &HOCLASS_IMPORT)
  }


  /// Lookup a function, referred by this object and possibly attempt code
  /// loading if the module was missing. Return a code pointer.
  pub fn resolve(&self, code_server: &mut CodeServer) -> Hopefully<CodePtr> {
    code_server.lookup_and_load(&self.mfarity)
  }


  /// Assuming that this object refers to a BIF function, perform a BIF lookup.
  pub fn resolve_bif(&self) -> Hopefully<BifFn> {
    find_bif(&self.mfarity)
  }
}
