use crate::{
  emulator::{gen_atoms, heap::Heap},
  fail::{Error, RtResult},
  defs::ExceptionType,
  term::{
    builders::{make_badfun, make_badmatch},
    lterm::LTerm,
  },
};


/// Enum is used by VM dispatch handlers for opcodes to indicate whether to
/// continue, yield (take next process in the queue) or interrupt process
/// on error (to return error use Hopefully's Error::Exception/2)
#[allow(dead_code)]
pub enum DispatchResult {
  Normal,
  Yield,
}

impl DispatchResult {
  pub fn badmatch_val(val: LTerm, hp: &mut Heap) -> RtResult<DispatchResult> {
    let badmatch_tuple = make_badmatch(val, hp)?;
    Err(Error::Exception(ExceptionType::Error, badmatch_tuple))
  }

  pub fn badarity() -> RtResult<DispatchResult> {
    Err(Error::Exception(ExceptionType::Error, gen_atoms::BADARITY))
  }

  pub fn undef() -> RtResult<DispatchResult> {
    Err(Error::Exception(ExceptionType::Error, gen_atoms::UNDEF))
  }

  pub fn badfun() -> RtResult<DispatchResult> {
    Err(Error::Exception(ExceptionType::Error, gen_atoms::BADFUN))
  }

  pub fn badfun_val(val: LTerm, hp: &mut Heap) -> RtResult<DispatchResult> {
    let badfun_tuple = make_badfun(val, hp)?;
    Err(Error::Exception(ExceptionType::Error, badfun_tuple))
  }
}
