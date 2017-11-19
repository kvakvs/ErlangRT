use bif::BifResult;
use emulator::process::Process;
use term::lterm::*;
use term::lterm::aspect_smallint::{make_small_s};
use term::lterm::aspect_list::ListAspect;
use term::raw::rcons::ConsPtr;


fn module() -> &'static str { "bif_compare: " }


/// Compare 2 terms with '=='
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
    if tl.is_list() && !tl.is_nil() {
      count += 1;
      lst = tl.cons_get_ptr();
    } else {
      return BifResult::Value(make_small_s(count))
    }
  }
}
