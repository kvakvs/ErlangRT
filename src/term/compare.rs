use emulator::atom;

//use std::cmp::{Ordering};
use term::lterm::LTerm;
use term::primary;

/// Compare two terms for equality, fail if types are different even if
/// coercion is otherwise possible.
pub fn eq_terms(a: LTerm, b: LTerm, exact: bool) -> bool {
  // Compare type tags first
  if a.is_atom() && b.is_atom() {
    return eq_atoms(a, b);
  }

  let a_is_small = a.is_small();
  let b_is_small = b.is_small();
  if a_is_small && b_is_small {
    let a_small = a.small_get_s();
    let b_small = b.small_get_s();
    return a_small.eq(&b_small);
  }

  let a_is_float = unsafe { a.is_float() };
  let b_is_float = unsafe { b.is_float() };
  // If not exact then allow comparing float to int/bigint
  if !exact && (a_is_float || a_is_small) && (b_is_float || b_is_small) {
    return eq_numbers_not_exact(a, b);
  } else if a_is_float && b_is_float {
    return eq_floats(a, b)
  }

  eq_terms_primary(a, b)
}


fn eq_floats(a: LTerm, b: LTerm) -> bool {
  panic!("TODO: eq_floats")
}


fn eq_numbers_not_exact(a: LTerm, b: LTerm) -> bool {
  panic!("TODO: eq_numbers_not_exact")
}


/// Compare two atoms for equality.
fn eq_atoms(a: LTerm, b: LTerm) -> bool {
  let atom_a = atom::lookup(a);
  debug_assert!(atom_a.is_null() == false);

  let atom_b = atom::lookup(b);
  debug_assert!(atom_b.is_null() == false);

  // This should really be safe, as pointers to Atom exist statically forever
  unsafe {
    if (*atom_a).len == (*atom_b).len {
      return (*atom_a).name == (*atom_b).name;
    }
  }

  false
}


/// Switch between comparisons for equality by primary tag (immediate or boxes
/// or fail immediately for different primary tags).
fn eq_terms_primary(a: LTerm, b: LTerm) -> bool {
  let a_val = a.raw();
  let a_prim_tag = primary::get_tag(a_val);

  let b_val = b.raw();
  let b_prim_tag = primary::get_tag(b_val);
  if b_prim_tag != a_prim_tag {
    return false; // different primary types - not equal
  }

  match a_prim_tag {
    primary::TAG_IMMED => {
      return eq_terms_immed(a, b)
    },
    primary::TAG_CONS => {
      return eq_terms_cons(a, b)
    },
    primary::TAG_BOX => {
      return eq_terms_box(a, b)
    },
    _ => panic!("Primary tag {} eq_terms unsupported", a_prim_tag)
  }
}


fn eq_terms_immed(a: LTerm, b: LTerm) -> bool {
  panic!("TODO: eq_terms_immed")
}

fn eq_terms_cons(a: LTerm, b: LTerm) -> bool {
  panic!("TODO: eq_terms_cons")
}

fn eq_terms_box(a: LTerm, b: LTerm) -> bool {
  panic!("TODO: eq_terms_box")
}
