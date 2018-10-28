use emulator::function::{FunEntry, CallableLocation};
use emulator::heap::{Heap};
use emulator::mfa::{MFArity};
use fail::{Hopefully};
use rt_defs::{Word, Arity, storage_bytes_to_words};
use term::boxed::{BoxHeader, BoxTypeTag};
use term::lterm::*;

use std::mem::size_of;
use std::ptr;
use fail::Error;


const fn module() -> &'static str { "closure: " }


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
    storage_bytes_to_words(size_of::<Closure>()) + nfree
  }


  fn new(mfa: MFArity, nfree: u32) -> Closure {
    let arity = Closure::storage_size(nfree) - 1;
    Closure {
      header: BoxHeader::new(BoxTypeTag::Closure, arity),
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
    let this = hp.alloc_words::<Closure>(n_words, false)?;

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


  pub unsafe fn const_from_term(t: LTerm) -> Hopefully<*const Closure> {
    helper_get_const_from_boxed_term::<Closure>(
      t, BoxTypeTag::Closure, Error::BoxedIsNotAClosure)
  }


  pub unsafe fn mut_from_term(t: LTerm) -> Hopefully<*mut Closure> {
    helper_get_mut_from_boxed_term::<Closure>(
      t, BoxTypeTag::Closure, Error::BoxedIsNotAClosure)
  }

}
