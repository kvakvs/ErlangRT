use crate::{
  defs::{ByteSize, WordSize},
  emulator::{code::pointer::CodePtr, code_srv::CodeServer, heap::Heap, mfa::ModFunArity},
  fail::{RtErr, RtResult},
  native_fun::NativeFn,
  term::{
    boxed::{
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
      BoxHeader, BOXTYPETAG_IMPORT,
    },
    classify,
    lterm::*,
  },
};
use core::{mem::size_of, ptr};

#[allow(dead_code)]
pub struct Import {
  header: BoxHeader,
  pub mfarity: ModFunArity,
  /// Whether import points to a native fun or to BEAM fun, or we don't know yet
  is_bif: Option<bool>,
}

impl TBoxed for Import {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_FUN
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_IMPORT
  }
}

impl Import {
  const fn storage_size() -> WordSize {
    ByteSize::new(size_of::<Import>()).get_words_rounded_up()
  }

  pub unsafe fn create_into(hp: &mut Heap, mfarity: ModFunArity) -> RtResult<LTerm> {
    let n_words = Import::storage_size();
    let this = hp.alloc::<Import>(n_words, false)?;

    ptr::write(
      this,
      Import {
        header: BoxHeader::new::<Import>(n_words.words()),
        mfarity,
        is_bif: None, // we don't know yet
      },
    );
    Ok(LTerm::make_boxed(this))
  }

  pub fn get_is_bif(&mut self, code_srv: &CodeServer) -> bool {
    match self.is_bif {
      Some(t) => t,
      None => {
        let is_bif = code_srv.native_functions.mfa_exists(&self.mfarity);
        self.is_bif = Some(is_bif);
        is_bif
      }
    }
  }

  #[allow(dead_code)]
  pub unsafe fn const_from_term(t: LTerm) -> RtResult<*const Import> {
    helper_get_const_from_boxed_term::<Import>(
      t,
      BOXTYPETAG_IMPORT,
      RtErr::BoxedIsNotAnImport,
    )
  }

  #[allow(dead_code)]
  pub unsafe fn mut_from_term(t: LTerm) -> RtResult<*mut Import> {
    helper_get_mut_from_boxed_term::<Import>(
      t,
      BOXTYPETAG_IMPORT,
      RtErr::BoxedIsNotAnImport,
    )
  }

  /// Lookup a function, referred by this object and possibly attempt code
  /// loading if the module was missing. Return a code pointer.
  pub fn resolve(&self, code_server: &mut CodeServer) -> RtResult<CodePtr> {
    code_server.lookup_beam_code_and_load(&self.mfarity)
  }

  /// Assuming that this object refers to a native function, look it up and
  /// return the function pointer.
  pub fn get_native_fn_ptr(&self, code_srv: &CodeServer) -> Option<NativeFn> {
    code_srv.native_functions.find_mfa(&self.mfarity)
  }
}
