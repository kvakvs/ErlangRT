use emulator::function::CallableLocation;
use emulator::function::FunEntry;
use emulator::heap::Heap;
use emulator::heap::IHeap;
use emulator::mfa::MFArity;
use fail::Hopefully;
use term::boxed::{BoxHeader, BoxTypeTag};
use term::lterm::LTerm;

use std::mem::size_of;
use std::ptr;
use fail::Error;
use core::cmp;


/// Boxed `Closure` is placed on heap and referred via LTerm::p
#[allow(dead_code)]
pub struct Closure {
  pub header: BoxHeader,

  pub mfa: MFArity,
  pub dst: CallableLocation,
  pub nfree: usize, // must be word size to avoid alignment of the following data
  // frozen values follow here in memory after the main fields,
  // first is a field, rest will be allocated as extra bytes after
  pub frozen: LTerm
}

impl Closure {
  #[inline]
  const fn storage_size(nfree: Word) -> Word {
    size_of::<Closure>() + nfree
  }


  fn new(mfa: MFArity, nfree: u32) -> HOClosure {
    Closure {
      header: BoxHeader::new(BoxTypeTag::Closure),
      mfa,
      dst: CallableLocation::NeedUpdate,
      nfree: nfree as Arity,
      frozen: LTerm::non_value()
    }
  }


  pub unsafe fn place_into(hp: &mut Heap,
                           fe: &FunEntry,
                           frozen: &[LTerm]) -> Hopefully<LTerm>
  {
    let n_words = Closure::storage_size(fe.nfree);
    let this = hp.heap_allocate(n_words, false)? as *mut Closure;

    assert_eq!(frozen.len(), fe.nfree as usize);
    println!("{}new closure: {} frozen={} nfree={}", module(),
             fe.mfa, frozen.len(), fe.nfree);

    ptr::write(this, Closure::new(fe.mfa, fe.nfree));

    assert_eq!(frozen.len(), fe.nfree as usize);
    // step 1 closure forward, which will point exactly at the frozen location
    let dst = this.offset(1);
    ptr::copy(frozen.as_ptr() as *const Word,
              dst as *mut Word,
              fe.nfree as usize);

    Ok(LTerm::make_box(this as *const Word))
  }

  #[inline]
  pub unsafe fn const_from_term(t: LTerm) -> Hopefully<*const HOClosure> {
    if !t.is_boxed() { return Err(Error::TermIsNotABoxed) }
    let cptr = t.get_box_ptr() as *const Closure;
    if unsafe { cptr.header.t != BoxTypeTag::Closure } {
      return Err(Error::BoxedIsNotAClosure)
    }
    cptr
  }


  #[inline]
  pub unsafe fn mut_from_term(t: LTerm) -> *mut HOClosure {
    debug_assert!(t.is_boxed());
    let cptr = t.get_box_ptr() as *mut Closure;
    unsafe { debug_assert!(cptr.header.t == BoxTypeTag::Closure) }
    cptr
  }

}
