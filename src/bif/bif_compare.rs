use emulator::gen_atoms;
use bif::BifResult;
use emulator::process::Process;
use term::lterm::*;
use term::compare::cmp_terms;

use std::cmp::{Ordering};


fn module() -> &'static str { "bif_compare: " }


/// Compare 2 terms with '=='
pub fn ubif_seqeq_2_2(_cur_proc: &mut Process,
                      args: &[LTerm]) -> BifResult {
  assert_eq!(args.len(), 2, "{}ubif_seqeq_2_2 takes 2 args", module());
  let a: LTerm = args[0];
  let b: LTerm = args[1];

  match cmp_terms(a, b, false) {
    Ordering::Equal => BifResult::Value(gen_atoms::TRUE),
    _ => BifResult::Value(gen_atoms::FALSE),
  }
}
