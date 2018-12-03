use emulator::gen_atoms;
use emulator::heap::Heap;
use rt_defs::ExceptionType;
use term::builders::{make_badmatch, make_badfun};
use term::lterm::LTerm;
use fail::{Error, Hopefully};


/// Enum is used by VM dispatch handlers for opcodes to indicate whether to
/// continue, yield (take next process in the queue) or interrupt process
/// on error (to return error use Hopefully's Error::Exception/2)
#[allow(dead_code)]
pub enum DispatchResult {
  Normal,
  Yield,
}

impl DispatchResult {
  pub fn badmatch_val(val: LTerm, hp: &mut Heap) -> Hopefully<DispatchResult> {
    let badmatch_tuple = make_badmatch(val, hp)?;
    Err(Error::Exception(ExceptionType::Error,
                         badmatch_tuple))
  }

  pub fn badarity() -> Hopefully<DispatchResult> {
    Err(Error::Exception(ExceptionType::Error,
                         gen_atoms::BADARITY))
  }

  pub fn undef() -> Hopefully<DispatchResult> {
    Err(Error::Exception(ExceptionType::Error,
                         gen_atoms::UNDEF))
  }

  pub fn badfun() -> Hopefully<DispatchResult> {
    Err(Error::Exception(ExceptionType::Error,
                         gen_atoms::BADFUN))
  }

  pub fn badfun_val(val: LTerm, hp: &mut Heap) -> Hopefully<DispatchResult> {
    let badfun_tuple = make_badfun(val, hp)?;
    Err(Error::Exception(ExceptionType::Error,
                         badfun_tuple))
  }
}