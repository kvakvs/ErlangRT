use bif::result::{BifResult};
use emulator::gen_atoms;
use emulator::process::{Process};
use term::compare::{cmp_terms};
use term::lterm::{LTerm};

use std::cmp::{Ordering};


fn module() -> &'static str { "bif_compare: " }


/// Compare 2 terms with '=='
pub fn ubif_seqeq_2_2(_cur_proc: &mut Process,
                      args: &[LTerm]) -> BifResult {
  assert_eq!(args.len(), 2, "{}ubif_seqeq_2_2 takes 2 args", module());
  shared_eq(args, false)
}


/// Compare 2 terms with '=:='
pub fn ubif_seq_2_2(_cur_proc: &mut Process,
                    args: &[LTerm]) -> BifResult {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, true)
}


#[inline]
fn shared_eq(args: &[LTerm], exact: bool) -> BifResult {
  let a: LTerm = args[0];
  let b: LTerm = args[1];

  match cmp_terms(a, b, exact) {
    Ordering::Equal => BifResult::Value(gen_atoms::TRUE),
    _ => BifResult::Value(gen_atoms::FALSE),
  }
}
