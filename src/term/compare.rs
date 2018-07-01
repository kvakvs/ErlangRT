use emulator::atom;

use std::cmp::{Ordering};
use term::lterm::*;
use term::classify;
use term::primary;
use term::immediate;


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
      ContinueCompare::AnyType(a1, b1) |
      ContinueCompare::Cons(a1, b1) => {
        cmp_terms_any_type(a1, b1, exact)
      },
    };

    match eq_result {
      EqResult::Concluded(result) if result == Ordering::Equal => {
        if stack.is_empty() {
          //println!("comparison {} {} concluded {:?}", a, b, result);
          return result
        } else {
          //println!("comparison {} {} got intermediate result {:?}", a, b, result);
          op = stack.pop().unwrap();
          continue
        } // stack not empty
      },

      EqResult::Concluded(result) => return result,

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
  //println!("cmp any type {} {}", a, b);

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

  cmp_terms_primary(a, b, exact)
}


fn cmp_floats(_a: LTerm, _b: LTerm) -> Ordering {
  panic!("TODO: eq_floats")
}


fn cmp_numbers_not_exact(_a: LTerm, _b: LTerm) -> Ordering {
  panic!("TODO: eq_numbers_not_exact")
}


/// Compare two atoms for equality.
fn cmp_atoms(a: LTerm, b: LTerm) -> Ordering {
  assert!(!a.is_nil());
  let atomp_a = atom::lookup(a);
  debug_assert!(!atomp_a.is_null(),
                "cmp_atoms: atom lookup {} failed", a);

  assert!(!b.is_nil());
  let atomp_b = atom::lookup(b);
  debug_assert!(!atomp_b.is_null(),
                "cmp_atoms: atom lookup {} failed", b);

  // This should really be safe, as pointers to Atom exist statically forever
  unsafe {
    let a_len = (*atomp_a).len;
    let b_len = (*atomp_b).len;
    if a_len == b_len {
      return (*atomp_a).name.cmp(&(*atomp_b).name);
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
fn cmp_terms_primary(a: LTerm, b: LTerm, exact: bool) -> EqResult {
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
      EqResult::Concluded(cmp_terms_immed(a, b, exact))
    },
    primary::TAG_CONS => unsafe {
      cmp_cons(a, b)
    },
    primary::TAG_BOX => {
      cmp_terms_box(a, b)
    },
    _ => panic!("Primary tag {} eq_terms unsupported", a_prim_tag)
  }
}


fn cmp_terms_immed(a: LTerm, b: LTerm, _exact: bool) -> Ordering {
  let av = a.raw();
  let bv = b.raw();

  if (a.is_nil() || a.is_empty_tuple() || a.is_empty_binary())
      && (a.raw() == b.raw()) {
    return Ordering::Equal;
  }

  if a.is_immediate1() {
    if a.is_local_port() {
      if b.is_local_port() {
        panic!("TODO: cmp local vs local port")
      } else if b.is_external_port() {
        panic!("TODO: cmp local vs ext port")
      } else {
        return cmp_mixed_types(a, b)
      }
    }

    if a.is_local_pid() {
      if b.is_local_pid() {
        panic!("TODO: cmp local vs local pid")
      } else if b.is_external_pid() {
        panic!("TODO: cmp local vs ext pid")
      } else {
        return cmp_mixed_types(a, b)
      }
    }
  }

  if a.is_cons() {
    if !b.is_cons() {
      return cmp_mixed_types(a, b)
    }

    panic!("TODO: invoke cmp_cons correctly from here")
    //return cmp_cons(a, b)
  }

  if a.is_box() {
    if unsafe { a.is_tuple() } {
      if unsafe { b.is_tuple() } {
        panic!("TODO: cmp tuple vs tuple")
      } else {
        return cmp_mixed_types(a, b)
      }
    } else if a.is_map() {
      if a.is_flat_map() {
        if !b.is_flat_map() {
          if b.is_hash_map() {
            let b_size = b.map_size();
            return a.map_size().cmp(&b_size)
          }
        } else {
          // Compare two flat maps
          panic!("TODO: cmp flatmap vs flatmap (+exact)")
        }
      } else {
        if !b.is_hash_map() {
          if b.is_flat_map() {
            let b_size = b.map_size();
            return a.map_size().cmp(&b_size)
          }
        } else {
          // Compare two hash maps
          panic!("TODO: cmp flatmap vs flatmap (+exact)")
        }
      }
      //Hashmap compare strategy:
      //Phase 1. While keys are identical
      //    Do synchronous stepping through leafs of both trees in hash
      //    order. Maintain value compare result of minimal key.
      //
      //Phase 2. If key diff was found in phase 1
      //    Ignore values from now on.
      //    Continue iterate trees by always advancing the one
      //    lagging behind hash-wise. Identical keys are skipped.
      //    A minimal key can only be candidate as tie-breaker if we
      //    have passed that hash value in the other tree (which means
      //    the key did not exist in the other tree).
    } else if unsafe { a.is_float() } {
      if !unsafe { b.is_float() } {
        // TODO: If b is integer and we don't do exact comparison?
        return cmp_mixed_types(a, b)
      } else {
        let af = unsafe { a.float_get() };
        let bf = unsafe { b.float_get() };
        return af.partial_cmp(&bf).unwrap()
      }
    } else if a.is_bignum() {
      if !b.is_bignum() {
        return cmp_mixed_types(a, b)
      }
    } else if a.is_export() {
      if !b.is_export() {
        return cmp_mixed_types(a, b)
      }
      // Compare two exports: from utils.c line ~2918
      // cmp atoms a.module and b.module
      // cmp atoms a.fn and b.fn
      // cmp arity
      panic!("TODO compare 2 exports")
    } else if a.is_header() {
      if !a.is_fun() {
        return cmp_mixed_types(a, b)
      }
      // Compare 2 function objects: from utils.c line ~2937
      // compare a.module, b.module
      // compare old_index
      // compare old_uniq
      // compare num_Free
      panic!("TODO compare 2 fun objects")
    } else if a.is_external_pid() {
      if b.is_local_pid() {
        panic!("TODO compare ext vs local pid")
      } else if b.is_external_pid() {
        panic!("TODO compare ext vs ext pid")
      } else {
        return cmp_mixed_types(a, b)
      }
    } else if a.is_external_port() {
      if b.is_local_port() {
        panic!("TODO compare ext vs local port")
      } else if b.is_external_port() {
        panic!("TODO compare ext vs ext port")
      } else {
        return cmp_mixed_types(a, b)
      }
    } else if a.is_local_ref() {
      if b.is_local_ref() {
        panic!("TODO compare local vs local ref")
      } else if b.is_external_ref() {
        panic!("TODO compare local vs ext ref")
      } else {
        return cmp_mixed_types(a, b)
      }
    } else if a.is_external_ref() {
      if b.is_local_ref() {
        panic!("TODO compare ext vs local ref")
      } else if b.is_external_ref() {
        panic!("TODO compare ext vs ext ref")
      } else {
        return cmp_mixed_types(a, b)
      }
    } else {
      // must be a binary
      assert!(unsafe { a.is_binary() });
      if !unsafe { b.is_binary() } {
        return cmp_mixed_types(a, b)
      }
      panic!("TODO cmp binaries")
    }
  }

  // if both are immediate3, it is hard to distunguish because all imm3 values
  // are special
  if a.is_immediate3() && b.is_immediate3() {
    let a3t = immediate::get_imm3_tag(av);
    let b3t = immediate::get_imm3_tag(bv);
    // If imm3 tags are same, we can conclude with comparing their values
    if a3t == b3t {
      let a3v = immediate::get_imm3_value(av);
      let b3v = immediate::get_imm3_value(bv);
      return a3v.cmp(&b3v);
    }
    return a3t.cmp(&b3t);
  }

  panic!("TODO: eq_terms_immed {} {}", a, b)
}


/// Deeper comparison of two values with different types
fn cmp_mixed_types(_a: LTerm, _b: LTerm) -> Ordering {
  panic!("TODO: cmp_mixed_types(a, b)")
}


/// Compare two cons (list) cells. In case when first elements are equal and
/// a deeper comparison is required, we will return `EqResult::CompareNested`.
/// This will be pushed to a helper stack by the caller (`cmp_terms()`).
unsafe fn cmp_cons(a: LTerm, b: LTerm) -> EqResult {
  let mut aa = a.cons_get_ptr();
  let mut bb = b.cons_get_ptr();

  loop {
    // Check the heads
    let ahd = aa.hd();
    let bhd = bb.hd();

    if !LTerm::is_same(ahd, bhd) {
      //println!("cmp_cons ahd {} bhd {}", ahd, bhd);
      // Recurse into a.hd and b.hd, but push a.tl and b.tl to continue
      let continue_op = ContinueCompare::Cons(aa.tl(), bb.tl());
      return EqResult::CompareNested(ahd, bhd, continue_op)
    }

    // See the tails
    let atl = aa.tl();
    let btl = bb.tl();

    if LTerm::is_same(atl, btl) {
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
