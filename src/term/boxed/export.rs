use crate::emulator::export;
use crate::emulator::heap::Heap;
use crate::emulator::mfa::MFArity;
use crate::fail::{Error, RtResult};
use crate::rt_defs::storage_bytes_to_words;
use crate::term::boxed::BoxHeader;
use crate::term::boxed::BOXTYPETAG_EXPORT;
use crate::term::lterm::*;

use core::ptr;

#[allow(dead_code)]
pub struct Export {
  header: BoxHeader,
  pub exp: export::Export,
}

impl Export {
  #[inline]
  fn storage_size() -> usize {
    storage_bytes_to_words(core::mem::size_of::<Export>())
  }


  fn new(n_words: usize, mfa: &MFArity) -> Export {
    Export {
      header: BoxHeader::new(BOXTYPETAG_EXPORT, n_words),
      exp: export::Export::new(*mfa),
    }
  }


  #[allow(dead_code)]
  pub unsafe fn create_into(hp: &mut Heap, mfa: &MFArity) -> RtResult<LTerm> {
    let n_words = Export::storage_size();
    let this = hp.alloc::<Export>(n_words, false)?;

    ptr::write(this, Export::new(n_words, mfa));
    Ok(LTerm::make_boxed(this))
  }


  pub unsafe fn const_from_term(t: LTerm) -> RtResult<*const Export> {
    helper_get_const_from_boxed_term::<Export>(t, BOXTYPETAG_EXPORT, Error::BoxedIsNotAnExport)
  }


  #[allow(dead_code)]
  pub unsafe fn mut_from_term(t: LTerm) -> RtResult<*mut Export> {
    helper_get_mut_from_boxed_term::<Export>(t, BOXTYPETAG_EXPORT, Error::BoxedIsNotAnExport)
  }
}
