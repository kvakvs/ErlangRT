use term::lterm::*;
use term::boxed::BoxHeader;
use emulator::export;
use fail::{Hopefully, Error};
use term::boxed::BoxTypeTag;
use rt_defs::{storage_bytes_to_words};
use emulator::mfa::MFArity;
use emulator::heap::Heap;

use std::mem::{size_of};
use core::ptr;

pub struct Export {
  header: BoxHeader,
  pub exp: export::Export
}

impl Export {
  #[inline]
  fn storage_size() -> usize {
    storage_bytes_to_words(size_of::<Export>())
  }


  fn new(n_words: usize, mfa: &MFArity) -> Export {
    Export {
      header: BoxHeader::new(BoxTypeTag::Export, n_words),
      exp: export::Export::new(*mfa),
    }
  }


  #[allow(dead_code)]
  pub unsafe fn place_into(hp: &mut Heap,
                           mfa: &MFArity) -> Hopefully<LTerm>
  {
    let n_words = Export::storage_size();
    let this = hp.heap_allocate(n_words, false)? as *mut Export;

    ptr::write(this, Export::new(n_words, mfa));
    Ok(LTerm::make_boxed(this))
  }


  pub unsafe fn const_from_term(t: LTerm) -> Hopefully<*const Export> {
    helper_get_const_from_boxed_term::<Export>(
      t, BoxTypeTag::Export, Error::BoxedIsNotAnExport)
  }


  pub unsafe fn mut_from_term(t: LTerm) -> Hopefully<*mut Export> {
    helper_get_mut_from_boxed_term::<Export>(
      t, BoxTypeTag::Export, Error::BoxedIsNotAnExport)
  }

}
