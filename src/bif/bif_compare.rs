use crate::{
  emulator::process::Process,
  fail::RtResult,
  term::{compare::cmp_terms, lterm::LTerm},
};

use std::cmp::Ordering;

fn module() -> &'static str {
  "bif_compare: "
}

/// Compare 2 terms with '==' (s eq eq)
pub fn ubif_seqeq_2_2(_cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  assert_eq!(args.len(), 2, "{}ubif_seqeq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Equal, false, false)
}

/// Compare 2 terms with '/=' (s not eq eq)
/// Expressed as NOT EQUAL
pub fn ubif_sneqeq_2_2(_cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  assert_eq!(args.len(), 2, "{}ubif_seqeq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Equal, true, false)
}

/// Compare 2 terms with '=:=' (s eq)
pub fn ubif_seq_2_2(_cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Equal, false, true)
}

/// Compare 2 terms with '=/=' (s not eq)
/// Expressed as NOT EQUAL (EXACT)
/// Sssssnek...
pub fn ubif_sneq_2_2(_cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Equal, true, true)
}

/// Compare 2 terms with '<' (s less-than)
pub fn ubif_slt_2_2(_cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Less, false, false)
}

/// Compare 2 terms with '=<' (s greater-than)
pub fn ubif_sgt_2_2(_cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Greater, false, false)
}

/// Compare 2 terms with '=<' (s less-equal)
/// Expressed as NOT GREATER
pub fn ubif_sle_2_2(_cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Greater, true, false)
}

/// Compare 2 terms with '>=' (s greater-equal)
/// Expressed as NOT LESS
pub fn ubif_sge_2_2(_cur_proc: &mut Process, args: &[LTerm]) -> RtResult<LTerm> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Less, true, false)
}

/// Shared compare routine which expects a specific `ordering` to return true.
///
/// * Arg: `invert`: The result can be inverted.
/// * Arg: `exact`: The comparison can be exact or with number coercion
#[inline]
fn shared_eq(
  args: &[LTerm],
  ordering: Ordering,
  invert: bool,
  exact: bool,
) -> RtResult<LTerm> {
  let cmp_result = cmp_terms(args[0], args[1], exact)? == ordering;
  Ok(LTerm::make_bool(cmp_result ^ invert))
}
