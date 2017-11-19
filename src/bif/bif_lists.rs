use rt_defs::ExceptionType;
use bif::BifResult;
use emulator::process::Process;
use emulator::gen_atoms;
use term::lterm::*;
use term::lterm::aspect_smallint::{make_small_s};
use term::lterm::aspect_list::ListAspect;
use term::raw::rcons::ConsPtr;


fn module() -> &'static str { "bif_compare: " }


/// Calculate length of a list by traversing it.
pub fn gcbif_length_1(_cur_proc: &mut Process,
                      args: &[LTerm]) -> BifResult {
  assert_eq!(args.len(), 1, "{}gcbif_length_1 takes 1 arg", module());

  let l0: LTerm = args[0];
  if l0.is_nil() {
    return BifResult::Value(make_small_s(0));
  }

  let mut lst = l0.cons_get_ptr();
  let mut count = 0;
  loop {
    let tl = unsafe { lst.tl() };

    if tl.is_cons() {
      count += 1;
      lst = tl.cons_get_ptr();
    } else {
      if !tl.is_nil() {
        return BifResult::Exception(ExceptionType::Error, gen_atoms::BADARG);
      }
      return BifResult::Value(make_small_s(count))
    }
  }
}
