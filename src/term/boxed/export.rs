use crate::defs::ByteSize;
use crate::{
  defs::WordSize,
  emulator::{export, heap::Heap, mfa::MFArity},
  fail::{Error, RtResult},
  term::{
    boxed::{BoxHeader, BOXTYPETAG_EXPORT},
    lterm::*,
  },
};
use core::{mem::size_of, ptr};

#[allow(dead_code)]
pub struct Export {
  header: BoxHeader,
  pub exp: export::Export,
}

impl Export {
  #[inline]
  fn storage_size() -> WordSize {
    ByteSize::new(size_of::<Export>()).words_rounded_up()
  }

  fn new(mfa: &MFArity) -> Export {
    let n_words = Export::storage_size();
    Export {
      header: BoxHeader::new(BOXTYPETAG_EXPORT, n_words.words()),
      exp: export::Export::new(*mfa),
    }
  }

  #[allow(dead_code)]
  pub unsafe fn create_into(hp: &mut Heap, mfa: &MFArity) -> RtResult<LTerm> {
    let n_words = Export::storage_size();
    let this = hp.alloc::<Export>(n_words, false)?;

    ptr::write(this, Export::new(mfa));
    Ok(LTerm::make_boxed(this))
  }

  pub unsafe fn const_from_term(t: LTerm) -> RtResult<*const Export> {
    helper_get_const_from_boxed_term::<Export>(
      t,
      BOXTYPETAG_EXPORT,
      Error::BoxedIsNotAnExport,
    )
  }

  #[allow(dead_code)]
  pub unsafe fn mut_from_term(t: LTerm) -> RtResult<*mut Export> {
    helper_get_mut_from_boxed_term::<Export>(
      t,
      BOXTYPETAG_EXPORT,
      Error::BoxedIsNotAnExport,
    )
  }
}
