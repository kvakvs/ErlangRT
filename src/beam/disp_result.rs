use emulator::gen_atoms;
use emulator::heap::{Heap};
use rt_defs::ExceptionType;
use term::builders::{make_badmatch, make_badfun};
use term::lterm::LTerm;


/// Enum is used by VM dispatch handlers for opcodes to indicate whether to
/// continue, yield (take next process in the queue) or interrupt process
/// on error.
#[allow(dead_code)]
pub enum DispatchResult {
  Normal,
  Yield,
  Error(ExceptionType, LTerm),
}

impl DispatchResult {
//  pub fn badmatch() -> DispatchResult {
//    DispatchResult::Error(ExceptionType::Error, gen_atoms::BADMATCH)
//  }

  pub fn badmatch_val(val: LTerm, hp: &mut Heap) -> DispatchResult {
    let badmatch_tuple = make_badmatch(val, hp);
    DispatchResult::Error(ExceptionType::Error, badmatch_tuple)
  }

  pub fn badarity() -> DispatchResult {
    DispatchResult::Error(ExceptionType::Error, gen_atoms::BADARITY)
  }

  pub fn undef() -> DispatchResult {
    DispatchResult::Error(ExceptionType::Error, gen_atoms::UNDEF)
  }

  pub fn badfun() -> DispatchResult {
    DispatchResult::Error(ExceptionType::Error, gen_atoms::BADFUN)
  }

  pub fn badfun_val(val: LTerm, hp: &mut Heap) -> DispatchResult {
    let badfun_tuple = make_badfun(val, hp);
    DispatchResult::Error(ExceptionType::Error, badfun_tuple)
  }
}