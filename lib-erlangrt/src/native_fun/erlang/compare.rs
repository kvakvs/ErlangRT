use crate::{
  emulator::{process::Process, vm::VM},
  fail::RtResult,
  term::{compare::cmp_terms, Term},
};
use core::cmp::Ordering;

fn module() -> &'static str {
  "bif_compare: "
}

/// Compare 2 terms with '=='
pub fn nativefun_equalequal_2(
  _vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_eq!(args.len(), 2, "{}ubif_seqeq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Equal, false, false)
}

/// Compare 2 terms with '/='
/// Expressed as NOT EQUAL
pub fn nativefun_notequal_2(
  _vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_eq!(args.len(), 2, "{}ubif_seqeq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Equal, true, false)
}

/// Compare 2 terms with '=:='
pub fn nativefun_equal_exact_2(
  _vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Equal, false, true)
}

/// Compare 2 terms with '=/=' (s not eq)
/// Expressed as NOT EQUAL (EXACT)
/// Sssssnek...
pub fn nativefun_notequal_exact_2(
  _vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Equal, true, true)
}

/// Compare 2 terms with '<' (s less-than)
pub fn nativefun_lessthan_2(
  _vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Less, false, false)
}

/// Compare 2 terms with '=<' (s greater-than)
pub fn nativefun_greaterthan_2(
  _vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Greater, false, false)
}

/// Compare 2 terms with '=<' (s less-equal)
/// Expressed as NOT GREATER
pub fn nativefun_lessequal_2(
  _vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Greater, true, false)
}

/// Compare 2 terms with '>=' (s greater-equal)
/// Expressed as NOT LESS
pub fn nativefun_greaterequal_2(
  _vm: &mut VM,
  _cur_proc: &mut Process,
  args: &[Term],
) -> RtResult<Term> {
  assert_eq!(args.len(), 2, "{}ubif_seq_2_2 takes 2 args", module());
  shared_eq(args, Ordering::Less, true, false)
}

/// Shared compare routine which expects a specific `ordering` to return true.
///
/// * Arg: `invert`: The result can be inverted.
/// * Arg: `exact`: The comparison can be exact or with number coercion
#[inline]
fn shared_eq(
  args: &[Term],
  ordering: Ordering,
  invert: bool,
  exact: bool,
) -> RtResult<Term> {
  let cmp_result = cmp_terms(args[0], args[1], exact)? == ordering;
  Ok(Term::make_bool(cmp_result ^ invert))
}
