use bif::result::{BifResult};
use emulator::gen_atoms;
use emulator::process::Process;
use rt_defs::ExceptionType;
use term::lterm::*;


fn module() -> &'static str { "bif_compare: " }


/// Calculate length of a list by traversing it.
pub fn gcbif_length_1(_cur_proc: &mut Process,
                      args: &[LTerm]) -> BifResult {
  assert_eq!(args.len(), 1, "{}gcbif_length_1 takes 1 arg", module());

  let l0: LTerm = args[0];
  if l0.is_nil() {
    return BifResult::Value(LTerm::make_small_signed(0));
  }

  let mut lst = l0.cons_get_ptr();
  let mut count = 1;
  loop {
    let tl = unsafe { lst.tl() };

    if tl.is_cons() {
      count += 1;
      lst = tl.cons_get_ptr();
    } else {
      if !tl.is_nil() {
        return BifResult::Exception(ExceptionType::Error, gen_atoms::BADARG);
      }
      return BifResult::Value(LTerm::make_small_signed(count))
    }
  }
}
