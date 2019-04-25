use crate::{
  defs::{ByteSize, WordSize},
  emulator::{export, heap::heap_trait::THeap, mfa::ModFunArity},
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
      BoxHeader, BOXTYPETAG_EXPORT,
    },
    classify,
    value::*,
  },
};
use core::{mem::size_of, ptr};

#[allow(dead_code)]
pub struct Export {
  header: BoxHeader,
  pub exp: export::Export,
}

impl TBoxed for Export {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_FUN
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_EXPORT
  }
}

impl Export {
  #[inline]
  fn storage_size() -> WordSize {
    ByteSize::new(size_of::<Self>()).get_words_rounded_up()
  }

  fn new(mfa: &ModFunArity) -> Self {
    let storage_size = Export::storage_size();
    Self {
      header: BoxHeader::new::<Self>(storage_size),
      exp: export::Export::new(*mfa),
    }
  }

  #[allow(dead_code)]
  pub unsafe fn create_into(hp: &mut THeap, mfa: &ModFunArity) -> RtResult<Term> {
    let n_words = Self::storage_size();
    let this = hp.alloc::<Self>(n_words, false)?;

    ptr::write(this, Self::new(mfa));
    Ok(Term::make_boxed(this))
  }

  #[allow(dead_code)]
  pub unsafe fn const_from_term(t: Term) -> RtResult<*const Self> {
    helper_get_const_from_boxed_term::<Self>(
      t,
      BOXTYPETAG_EXPORT,
      RtErr::BoxedIsNotAnExport,
    )
  }

  #[allow(dead_code)]
  pub unsafe fn mut_from_term(t: Term) -> RtResult<*mut Self> {
    helper_get_mut_from_boxed_term::<Self>(
      t,
      BOXTYPETAG_EXPORT,
      RtErr::BoxedIsNotAnExport,
    )
  }
}
