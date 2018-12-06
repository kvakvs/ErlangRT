use emulator::gen_atoms;
use emulator::process::{Process};
use fail::{RtResult};
use rt_defs::{ExceptionType};
use term::lterm::*;
use fail::Error;


fn module() -> &'static str { "bif_compare: " }


/// Calculate length of a list by traversing it.
pub fn gcbif_length_1(_cur_proc: &mut Process,
                      args: &[LTerm]) -> RtResult<LTerm> {
  assert_eq!(args.len(), 1, "{}gcbif_length_1 takes 1 arg", module());

  let l0: LTerm = args[0];
  if l0 == LTerm::nil() {
    return Ok(LTerm::make_small_signed(0))
  }

  let mut lst = l0.get_cons_ptr();
  let mut count = 1;
  loop {
    let tl = unsafe { (*lst).tl() };

    if tl.is_cons() {
      count += 1;
      lst = tl.get_cons_ptr();
    } else {
      if tl != LTerm::nil() {
        return Err(Error::Exception(ExceptionType::Error,
                                    gen_atoms::BADARG))
      }
      return Ok(LTerm::make_small_signed(count))
    }
  }
}
