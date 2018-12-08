use crate::{
  emulator::{gen_atoms, process::Process},
  fail::RtResult,
  term::{compare::cmp_terms, lterm::LTerm},
};

use std::cmp::Ordering;


fn module() -> &'static str {
  "bif_compare: "
}


/// Compare 2 terms with '=='
pub fn ubif_seqeq_2_2(_cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  assert_eq!(args.len(), 2, "{}ubif_seqeq_2_2 takes 2 args", module());
  shared_eq(args, false)
}


/// Compare 2 terms with '=:='
pub fn ubif_seq_2_2(_cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, true)
}


#[inline]
fn shared_eq(args: &[LTerm], exact: bool) -> RtResult<LTerm> {
  let a: LTerm = args[0];
  let b: LTerm = args[1];

  match cmp_terms(a, b, exact)? {
    Ordering::Equal => Ok(gen_atoms::TRUE),
    _ => Ok(gen_atoms::FALSE),
  }
}
