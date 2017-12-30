//! Heap object which stores a closure - a stateful function pointer (with some
//! frozen values captured at its creation).

use std::mem::size_of;
use std::ptr;

use emulator::function::FunEntry;
use emulator::heap::Heap;
use emulator::mfa::MFArity;
use fail::Hopefully;
use rt_defs::heap::IHeap;
use rt_defs::{WORD_BYTES, Word};
use term::classify::TermClass;
use term::lterm::*;
use term::raw::heapobj::*;
use emulator::function::CallableLocation;


fn module() -> &'static str { "ho_closure: " }


/// Heap object `HOClosure` is placed on heap.
#[allow(dead_code)]
pub struct HOClosure {
  pub hobj: HeapObjHeader,
  pub mfa: MFArity,
  pub dst: CallableLocation,
  pub nfree: u32,
  // frozen values follow here in memory after the main fields
}


#[allow(const_err)]
static HOCLASS_CLOSURE: HeapObjClass = HeapObjClass {
  obj_type: HeapObjType::Closure,
  dtor: HOClosure::dtor,
  fmt_str: HOClosure::fmt_str,
  term_class: TermClass::Special_,
};


impl HOClosure {
  /// Destructor.
  pub unsafe fn dtor(_this: *mut Word) {}


  pub unsafe fn fmt_str(this0: *const Word) -> String {
    let this = this0 as *mut HOClosure;
    format!("Closure({})", (*this).mfa)
  }


  const STRUCT_SIZE: usize = (size_of::<HOClosure>() + WORD_BYTES - 1) / WORD_BYTES;


  #[inline]
  fn storage_size(nfree: u32) -> usize {
    HOClosure::STRUCT_SIZE + (nfree as usize)
  }


  fn new(hobj: HeapObjHeader, mfa: MFArity, nfree: u32) -> HOClosure {
    HOClosure { hobj, mfa, dst: CallableLocation::NeedUpdate, nfree }
  }


  pub unsafe fn place_into(hp: &mut Heap,
                           fe: &FunEntry,
                           frozen: &[LTerm]) -> Hopefully<LTerm>
  {
    let n_words = HOClosure::storage_size(fe.nfree);
    let this = hp.heap_allocate(n_words, false)? as *mut HOClosure;

    assert_eq!(frozen.len(), fe.nfree as usize);
    println!("{}new closure: {} frozen={} nfree={}", module(), fe.mfa, frozen.len(), fe.nfree);

    ptr::write(this,
               HOClosure::new(
                 HeapObjHeader::new(n_words, &HOCLASS_CLOSURE),
                 fe.mfa,
                 fe.nfree
               ));

    assert_eq!(frozen.len(), fe.nfree as usize);
    // step 1 closure forward, which will point exactly at the frozen location
    let dst = this.offset(1);
    ptr::copy(frozen.as_ptr() as *const Word,
              dst as *mut Word,
              fe.nfree as usize);

    Ok(make_box(this as *const Word))
  }


  #[inline]
  pub unsafe fn from_term(t: LTerm) -> Hopefully<*const HOClosure> {
    heapobj_from_term::<HOClosure>(t, &HOCLASS_CLOSURE)
  }


  //  /// Create a boxed term. NOTE: There is no `self`, this is a raw pointer.
  //  #[inline]
  //  pub fn make_term(this: *const HOClosure) -> LTerm {
  //    make_box(this as *const Word)
  //  }
}
