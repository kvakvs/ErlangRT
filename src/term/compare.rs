use emulator::atom;

use std::cmp::{Ordering};
use term::lterm::LTerm;

/// Compare two terms A and B, fail if types are different even if coercion is
/// otherwise possible.
pub fn compare_terms(a: LTerm, b: LTerm, exact: bool) -> Ordering {
  /// Compare type tags first
  if a.is_atom() && b.is_atom() {
    return compare_atoms(a, b)
  } else if LTerm::are_both_small(a, b) {
    let a_small = a.small_get_s();
    let b_small = b.small_get_s();
    return a_small.eq(b_small)
  } else if unsafe { a.is_float() } && unsafe { b.is_float() } {
    panic!("TODO: Compare 2 floats")
  }
  compare_compound(a, b, exact)
}


fn compare_atoms(a: LTerm, b: LTerm) -> Ordering {
  let atom_a = atom::lookup(a);
  let atom_b = atom::lookup(b);
}
