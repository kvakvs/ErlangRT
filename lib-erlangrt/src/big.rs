use crate::{
  emulator::heap::Heap,
  fail::RtResult,
  term::{boxed, lterm::Term},
};

/// Helper, creates a new big integer on heap with val.
pub fn from_isize(hp: &mut Heap, val: isize) -> RtResult<Term> {
  let p = boxed::Bignum::with_isize(hp, val)?;
  Ok(Term::make_boxed(p))
}

/// Multiply two big ints
pub fn mul(_hp: &mut Heap, a: Term, b: Term) -> RtResult<Term> {
  debug_assert!(a.is_big_int());
  debug_assert!(b.is_big_int());
  unimplemented!("mul big a, b")
}

pub fn isize_from(val: Term) -> Option<isize> {
  if val.is_small() {
    return Some(val.get_small_signed());
  }
  unimplemented!("isize from big")
}
