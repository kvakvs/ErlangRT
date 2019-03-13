use crate::{emulator::heap::Heap, term::lterm::Term};
use crate::term::boxed;

/// Helper, creates a new big integer on heap with val.
pub fn from_isize(hp: &mut Heap, val: isize) -> Term {
  let p = boxed::Bignum::with_isize(hp, val);
  Term::make_boxed(p)
}
