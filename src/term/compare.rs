use emulator::atom;

use std::cmp::{Ordering};
use term::lterm::LTerm;
use term::classify;
use term::primary;


/// When comparing nested terms they might turn out to be equal. `CompareOp`
/// is stored in `stack` in `eq_terms()` function and tells where to resume
/// comparing the previous term.
enum ContinueCompare {
  // This begins the compare while not knowing types for `a` or `b`.
  AnyType(LTerm, LTerm),
  // Resume comparing Cons cells, we just reenter `eq_terms_cons`.
  Cons(LTerm, LTerm),
}


enum EqResult {
  /// Equality result is concluded to be the `bool` value.
  Concluded(Ordering),
  /// Equality is not concluded, but comparing these two terms will give the
  /// result (equivalent to `goto tailrecur_ne` in Erlang/OTP). This happens
  /// when a beginning elements of a nested structure compares equal but some
  /// members have to be checked recursively.
  CompareNested(LTerm, LTerm, ContinueCompare),
}


/// Compare two terms for equality, fail if types are different even if
/// coercion is otherwise possible.
pub fn cmp_terms(a: LTerm, b: LTerm, exact: bool) -> Ordering {
  // Comparison might want to recurse, to avoid stack growth, do a switch here
  // and continue comparing. We grow `stack` instead of a CPU stack.
  let mut stack = Vec::<ContinueCompare>::new();
  let mut op = ContinueCompare::AnyType(a, b);

  loop {
    let eq_result = match op {
      ContinueCompare::AnyType(a1, b1) => {
        cmp_terms_any_type(a1, b1, exact)
      }
      ContinueCompare::Cons(a2, b2) => unsafe {
        cmp_terms_cons(a2, b2)
      },
    };

    match eq_result {
      EqResult::Concluded(result) => {
        if stack.is_empty() {
          //println!("eq {} {} concluded {}", a, b, result);
          return result
        } else {
          op = stack.pop().unwrap();
          continue
        } // stack not empty
      },

      // Nested terms may accidentally compare equal, to be able to return and
      // continue comparing upper level term, we store a `continue_op` on
      // `stack`.
      EqResult::CompareNested(a3, b3, continue_op) => {
        stack.push(continue_op);
        op = ContinueCompare::AnyType(a3, b3);
        continue
      },
    }
  }
}


fn cmp_terms_any_type(a: LTerm, b: LTerm, exact: bool) -> EqResult {
  // Compare type tags first
  if a.is_atom() && b.is_atom() {
    return EqResult::Concluded(cmp_atoms(a, b));
  }

  let a_is_small = a.is_small();
  let b_is_small = b.is_small();
  if a_is_small && b_is_small {
    let a_small = a.small_get_s();
    let b_small = b.small_get_s();
    return EqResult::Concluded(a_small.cmp(&b_small));
  }

  let a_is_float = unsafe { a.is_float() };
  let b_is_float = unsafe { b.is_float() };
  // If not exact then allow comparing float to int/bigint
  if !exact && (a_is_float || a_is_small) && (b_is_float || b_is_small) {
    return EqResult::Concluded(cmp_numbers_not_exact(a, b));
  } else if a_is_float && b_is_float {
    return EqResult::Concluded(cmp_floats(a, b))
  }

  // If types don't compare equal, we can stop comparing here?
  // TODO: Except when we compare numbers
//  let order = cmp_type_order(a, b);
//  if order != Ordering::Equal {
//    return EqResult::Concluded(order);
//  }

  cmp_terms_primary(a, b)
}


fn cmp_floats(_a: LTerm, _b: LTerm) -> Ordering {
  panic!("TODO: eq_floats")
}


fn cmp_numbers_not_exact(_a: LTerm, _b: LTerm) -> Ordering {
  panic!("TODO: eq_numbers_not_exact")
}


/// Compare two atoms for equality.
fn cmp_atoms(a: LTerm, b: LTerm) -> Ordering {
  let atom_a = atom::lookup(a);
  debug_assert!(atom_a.is_null() == false);

  let atom_b = atom::lookup(b);
  debug_assert!(atom_b.is_null() == false);

  // This should really be safe, as pointers to Atom exist statically forever
  unsafe {
    let a_len = (*atom_a).len;
    let b_len = (*atom_b).len;
    if a_len == b_len {
      return (*atom_a).name.cmp(&(*atom_b).name);
    }
    a_len.cmp(&b_len)
  }
}


/// Compare order of two types without looking into their value.
fn cmp_type_order(a: LTerm, b: LTerm) -> Ordering {
  let aclass = classify::classify_term(a);
  let bclass = classify::classify_term(b);
  aclass.cmp(&bclass)
}


/// Switch between comparisons for equality by primary tag (immediate or boxes
/// or fail immediately for different primary tags).
fn cmp_terms_primary(a: LTerm, b: LTerm) -> EqResult {
  let a_val = a.raw();
  let a_prim_tag = primary::get_tag(a_val);

  let b_val = b.raw();
  let b_prim_tag = primary::get_tag(b_val);
  if b_prim_tag != a_prim_tag {
    // different primary types, compare their classes
    // This can be optimized a little but is there any value in optimization?
    return EqResult::Concluded(cmp_type_order(a, b));
  }

  match a_prim_tag {
    primary::TAG_IMMED => {
      return EqResult::Concluded(cmp_terms_immed(a, b))
    },
    primary::TAG_CONS => unsafe {
      return cmp_terms_cons(a, b)
    },
    primary::TAG_BOX => {
      return cmp_terms_box(a, b)
    },
    _ => panic!("Primary tag {} eq_terms unsupported", a_prim_tag)
  }
}


fn cmp_terms_immed(_a: LTerm, _b: LTerm) -> Ordering {
  panic!("TODO: eq_terms_immed")
}


/// Compare two boxed or immediate terms. In case when nested terms need to be
/// recursively compared, we return `EqResult::CompareNested` to change the
/// values `a` and `b` and perform another comparison without growing the stack.
unsafe fn cmp_terms_cons(a: LTerm, b: LTerm) -> EqResult {
  let mut aa = a.cons_get_ptr();
  let mut bb = b.cons_get_ptr();
  loop {
    // Check the heads
    let ahd = aa.hd();
    let bhd = bb.hd();
    if ahd.raw() != bhd.raw() {
      // Recurse into a.hd and b.hd, but push a.tl and b.tl to continue
      let continue_op = ContinueCompare::Cons(aa.tl(), bb.tl());
      return EqResult::CompareNested(ahd, bhd, continue_op)
    }

    // See the tails
    let atl = aa.tl();
    let btl = bb.tl();
    if atl.raw() == btl.raw() {
      return EqResult::Concluded(Ordering::Equal)
    }
    if !atl.is_list() || !btl.is_list() {
      // Just do a regular compare of `a.tl` vs `b.tl`
      let continue_op = ContinueCompare::AnyType(atl, btl);
      return EqResult::CompareNested(atl, btl, continue_op)
    }

    // Take the next linked cons cell and continue comparing
    aa = atl.cons_get_ptr();
    bb = btl.cons_get_ptr();
  }
}


fn cmp_terms_box(_a: LTerm, _b: LTerm) -> EqResult {
  panic!("TODO: eq_terms_box")
}
