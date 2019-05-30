use core::cmp::Ordering;

use crate::{
  defs::TDataReader,
  emulator::atom,
  fail::RtResult,
  term::{
    boxed::{self, binary::TBinary},
    classify,
    compare::EqResult::Concluded,
    *,
  },
};

/// When comparing nested terms they might turn out to be equal. `CompareOp`
/// is stored in `stack` in `eq_terms()` function and tells where to resume
/// comparing the previous term.
#[allow(dead_code)]
enum ContinueCompare {
  // This begins the compare while not knowing types for `a` or `b`.
  AnyType { a: Term, b: Term },
  // Resume comparing Cons cells, we just reenter `eq_terms_cons`.
  Cons { a: Term, b: Term },
}

#[allow(dead_code)]
enum EqResult {
  /// Equality result is concluded to be the `bool` value.
  Concluded(Ordering),
  /// Equality is not concluded, but comparing these two terms will give the
  /// result (equivalent to `goto tailrecur_ne` in Erlang/OTP). This happens
  /// when a first element of a nested structure compares equal but some
  /// members remain to be checked recursively.
  CompareNested {
    a: Term,
    b: Term,
    state: ContinueCompare,
  },
}

/// Compare two terms for equality, fail if types are different even if
/// coercion is otherwise possible.
pub fn cmp_terms(a: Term, b: Term, exact: bool) -> RtResult<Ordering> {
  if a == b {
    return Ok(Ordering::Equal);
  }
  cmp_terms_1(a, b, exact)
}

#[inline]
fn cmp_terms_1(a: Term, b: Term, exact: bool) -> RtResult<Ordering> {
  // Comparison might want to recurse.
  // To avoid stack growth, do a switch here and continue comparing in a loop.
  // We grow `stack` vector instead of a CPU stack.
  const DEFAULT_CAPACITY: usize = 8;
  let mut stack = Vec::<ContinueCompare>::with_capacity(DEFAULT_CAPACITY);
  let mut op = ContinueCompare::AnyType { a, b };

  // The main comparison loop which is able to step deeper into recursive structures
  loop {
    let eq_result = match op {
      ContinueCompare::AnyType { a: a1, b: b1 }
      | ContinueCompare::Cons { a: a1, b: b1 } => cmp_terms_any_type(a1, b1, exact)?,
    };

    match eq_result {
      EqResult::Concluded(result) if result == Ordering::Equal => {
        if stack.is_empty() {
          return Ok(result);
        } else {
          op = stack.pop().unwrap();
          continue;
        } // stack not empty
      }

      EqResult::Concluded(result) => return Ok(result),

      // Nested terms may accidentally compare equal, to be able to return and
      // continue comparing upper level term, we store a `continue_op` on
      // `stack`.
      EqResult::CompareNested {
        a: a3,
        b: b3,
        state: continue_op,
      } => {
        stack.push(continue_op);
        op = ContinueCompare::AnyType { a: a3, b: b3 };
        continue;
      }
    }
  }
}

/// Given a and b, terms, branch on their type and try do draw some conclusions.
fn cmp_terms_any_type(a: Term, b: Term, exact: bool) -> RtResult<EqResult> {
  debug_assert!(a.is_value(), "compare_any_type, a #Nonvalue<> and a {}", b);
  debug_assert!(b.is_value(), "compare_any_type, a {} and a #Nonvalue<>", a);

  // Compare type tags first
  if a.is_atom() && b.is_atom() {
    return Ok(EqResult::Concluded(cmp_atoms(a, b)));
  }

  // Maybe both a and b are small integers
  let a_is_small = a.is_small();
  let b_is_small = b.is_small();
  if a_is_small && b_is_small {
    let a_small = a.get_small_signed();
    let b_small = b.get_small_signed();
    return Ok(EqResult::Concluded(a_small.cmp(&b_small)));
  }

  // Maybe some of a and b are floats
  let a_is_float = a.is_float();
  let b_is_float = b.is_float();

  // If not exact then allow comparing float to int/bigint
  if !exact && (a_is_float || a_is_small) && (b_is_float || b_is_small) {
    return Ok(EqResult::Concluded(cmp_numbers_not_exact(a, b)));
  } else if a_is_float && b_is_float {
    return Ok(EqResult::Concluded(cmp_floats(a, b)));
  }

  // If types don't compare equal, we can stop comparing here?
  // TODO: Except when we compare numbers
  //  let order = cmp_type_order(a, b);
  //  if order != Ordering::Equal {
  //    return EqResult::Concluded(order);
  //  }

  cmp_terms_primary(a, b, exact)
}

#[inline]
fn cmp_floats(a: Term, b: Term) -> Ordering {
  // Assume we know both values are floats
  unsafe { cmp_f64_naive(a.get_float_unchecked(), b.get_float_unchecked()) }
}

/// Naive f64 comparison, which does not work with NaN and Infinities
#[inline]
fn cmp_f64_naive(a: f64, b: f64) -> Ordering {
  debug_assert!(!a.is_nan());
  debug_assert!(!b.is_nan());
  debug_assert!(!a.is_infinite());
  debug_assert!(!b.is_infinite());

  if a < b {
    return Ordering::Less;
  } else if a > b {
    return Ordering::Greater;
  }
  Ordering::Equal
}

fn cmp_numbers_not_exact(_a: Term, _b: Term) -> Ordering {
  unimplemented!("eq_numbers_not_exact")
}

/// Compare two atoms for equality. Returns the ordering result.
fn cmp_atoms(a: Term, b: Term) -> Ordering {
  assert_ne!(a, Term::nil());
  let atomp_a = atom::lookup(a);
  debug_assert!(!atomp_a.is_null(), "cmp_atoms: atom lookup {} failed", a);

  assert_ne!(b, Term::nil());
  let atomp_b = atom::lookup(b);
  debug_assert!(!atomp_b.is_null(), "cmp_atoms: atom lookup {} failed", b);

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
fn cmp_type_order(a: Term, b: Term) -> Ordering {
  if a.is_cons() && b == Term::nil()
    || a.is_tuple() && b == Term::empty_tuple()
    || a.is_binary() && b == Term::empty_binary()
  {
    return Ordering::Greater;
  }

  let aclass = classify::classify_term(a);
  let bclass = classify::classify_term(b);
  aclass.cmp(&bclass)
}

/// Switch between comparisons for equality by primary tag (immediate or boxes
/// or fail immediately for different primary tags).
fn cmp_terms_primary(a: Term, b: Term, exact: bool) -> RtResult<EqResult> {
  let a_prim_tag = a.get_term_tag();
  let b_prim_tag = b.get_term_tag();
  // println!("cmp {} tag={:?} vs {} tag={:?}", a, a_prim_tag, b, b_prim_tag);

  if a_prim_tag != b_prim_tag {
    // different primary types, compare their classes
    // This can be optimized a little but is there any value in optimization?
    return Ok(EqResult::Concluded(cmp_type_order(a, b)));
  }

  match a_prim_tag {
    PrimaryTag::BOX_PTR => {
      if a.is_cp() || b.is_cp() {
        panic!("eq_terms for CP is unsupported")
      }
      Ok(Concluded(cmp_terms_immed_box(a, b)?))
    }

    PrimaryTag::CONS_PTR => {
      if !b.is_cons() {
        return Ok(EqResult::Concluded(cmp_mixed_types(a, b)?));
      }

      Ok(unsafe { cmp_cons(a, b) })
    }

    _ => {
      // Any non-boxed compare
      Ok(EqResult::Concluded(cmp_terms_immed(a, b, exact)?))
    } //_ => panic!("Primary tag {:?} eq_terms unsupported", a_prim_tag)
  }
}

// TODO: Optimize by doing case on tag bits
fn cmp_terms_immed(a: Term, b: Term, _exact: bool) -> RtResult<Ordering> {
  if (a == Term::nil() || a == Term::empty_tuple() || a == Term::empty_binary())
    && (a.raw() == b.raw())
  {
    return Ok(Ordering::Equal);
  }

  if a.is_local_port() {
    if b.is_local_port() {
      unimplemented!("cmp local vs local port")
    } else if b.is_external_port() {
      unimplemented!("cmp local vs ext port")
    } else {
      return cmp_mixed_types(a, b);
    }
  }

  if a.is_local_pid() {
    if b.is_local_pid() {
      // Concluded by comparing raw values
      return Ok(a.raw().cmp(&b.raw()));
    } else if b.is_external_pid() {
      unimplemented!("cmp local vs ext pid")
    } else {
      return cmp_mixed_types(a, b);
    }
  }

  if a.is_boxed() {
    return cmp_terms_immed_box(a, b);
  }

  // if both are internal immediates, compare their raw values or their tags
  if a.is_internal_immediate() && b.is_internal_immediate() {
    let a_tag = a.get_term_tag();
    let b_tag = b.get_term_tag();

    // If tags are same, we can conclude with comparing their values
    if a_tag == b_tag {
      let a_val = a.get_term_val_without_tag();
      let b_val = b.get_term_val_without_tag();
      return Ok(a_val.cmp(&b_val));
    }
    return Ok(a_tag.cmp(&b_tag));
  }

  unimplemented!("eq_terms_immed {} {}", a, b)
}

// TODO: Optimize by doing case on tag bits
#[inline]
fn cmp_terms_immed_box(a: Term, b: Term) -> RtResult<Ordering> {
  if a.is_tuple() {
    if b.is_tuple() {
      unimplemented!("cmp tuple vs tuple")
    } else {
      return cmp_mixed_types(a, b);
    }
  } else if a.is_map() {
    if a.is_flat_map() {
      if !b.is_flat_map() {
        if b.is_hash_map() {
          let b_size = b.map_size();
          return Ok(a.map_size().cmp(&b_size));
        }
      } else {
        // Compare two flat maps
        unimplemented!("cmp flatmap vs flatmap (+exact)")
      }
    } else if !b.is_hash_map() {
      if b.is_flat_map() {
        let b_size = b.map_size();
        return Ok(a.map_size().cmp(&b_size));
      }
    } else {
      // Compare two hash maps
      unimplemented!("cmp flatmap vs flatmap (+exact)")
    }

  // Hashmap compare strategy:
  // Phase 1. While keys are identical
  //    Do synchronous stepping through leafs of both trees in hash
  //    order. Maintain value compare result of minimal key.
  //
  // Phase 2. If key diff was found in phase 1
  //    Ignore values from now on.
  //    Continue iterate trees by always advancing the one
  //    lagging behind hash-wise. Identical keys are skipped.
  //    A minimal key can only be candidate as tie-breaker if we
  //    have passed that hash value in the other tree (which means
  //    the key did not exist in the other tree).
  } else if a.is_float() {
    if !b.is_float() {
      // TODO: If b is integer and we don't do exact comparison?
      return cmp_mixed_types(a, b);
    } else {
      let a_float = a.get_float()?;
      let b_float = b.get_float()?;
      return Ok(a_float.partial_cmp(&b_float).unwrap());
    }
  } else if a.is_big_int() {
    if !b.is_big_int() {
      return cmp_mixed_types(a, b);
    }
  } else if a.is_export() {
    if !b.is_export() {
      return cmp_mixed_types(a, b);
    }
    // Compare two exports: from utils.c line ~2918
    // cmp atoms a.module and b.module
    // cmp atoms a.fn and b.fn
    // cmp arity
    unimplemented!("compare 2 exports")
  } else if a.is_boxed() {
    if a.is_binary() {
      if b.is_binary() {
        return unsafe { cmp_binary(a, b) };
      }
    }
    if !a.is_fun() {
      return cmp_mixed_types(a, b);
    }
    // Compare 2 function objects: from utils.c line ~2937
    // compare a.module, b.module
    // compare old_index
    // compare old_uniq
    // compare num_Free
    unimplemented!("compare 2 fun objects")
  } else if a.is_external_pid() {
    if b.is_local_pid() {
      unimplemented!("compare ext vs local pid")
    } else if b.is_external_pid() {
      unimplemented!("compare ext vs ext pid")
    } else {
      return cmp_mixed_types(a, b);
    }
  } else if a.is_external_port() {
    if b.is_local_port() {
      unimplemented!("compare ext vs local port")
    } else if b.is_external_port() {
      unimplemented!("compare ext vs ext port")
    } else {
      return cmp_mixed_types(a, b);
    }
  } else if a.is_local_ref() {
    if b.is_local_ref() {
      unimplemented!("compare local vs local ref")
    } else if b.is_external_ref() {
      unimplemented!("compare local vs ext ref")
    } else {
      return cmp_mixed_types(a, b);
    }
  } else if a.is_external_ref() {
    if b.is_local_ref() {
      unimplemented!("compare ext vs local ref")
    } else if b.is_external_ref() {
      unimplemented!("compare ext vs ext ref")
    } else {
      return cmp_mixed_types(a, b);
    }
  } else {
    // must be a binary
    assert!(a.is_binary());
    if !b.is_binary() {
      return cmp_mixed_types(a, b);
    }
    unimplemented!("cmp binaries")
  }
  unimplemented!("eq_terms_immed_box {} {}", a, b)
}

#[inline]
unsafe fn cmp_binary(a: Term, b: Term) -> RtResult<Ordering> {
  let a_trait = boxed::Binary::get_trait_from_term(a);
  let b_trait = boxed::Binary::get_trait_from_term(b);
  let a_size = (*a_trait).get_bit_size();
  let b_size = (*b_trait).get_bit_size();
  if a_size != b_size {
    return Ok(a_size.cmp(&b_size));
  }

  println!("Going to compare {} {}", a, b);
  println!(
    "sizes - {} {} vs {} {}",
    a_size,
    (*a_trait).get_byte_size(),
    b_size,
    (*b_trait).get_byte_size()
  );

  // Try figure out a compatible byte- or bit-reader combination for A arg and
  // B arg and then call a branch function which will do the same for B.
  match (*a_trait).get_byte_reader() {
    Some(a_reader) => cmp_reader_vs_binary(a, b, a_reader, b_trait),
    None => {
      let a_reader = (*a_trait).get_bit_reader();
      cmp_reader_vs_binary(a, b, a_reader, b_trait)
    }
  }
}

/// Generic function which will branch statically depending whether we've been
/// able to get a byte-reader for A arg, or a bit-reader. It will further branch
/// by doing the same for B arg.
unsafe fn cmp_reader_vs_binary<AReader>(
  a: Term,
  b: Term,
  a_reader: AReader,
  b_trait: *const TBinary,
) -> RtResult<Ordering>
where
  AReader: TDataReader,
{
  match (*b_trait).get_byte_reader() {
    Some(b_reader) => cmp_reader_vs_reader(a, b, a_reader, b_reader),
    None => {
      let b_reader = (*b_trait).get_bit_reader();
      cmp_reader_vs_reader(a, b, a_reader, b_reader)
    }
  }
}

/// Now that we finally have two compatible readers, get some bytes and see
/// how they compare until a first mismatch.
unsafe fn cmp_reader_vs_reader<AReader, BReader>(
  _a: Term,
  _b: Term,
  a_reader: AReader,
  b_reader: BReader,
) -> RtResult<Ordering>
where
  AReader: TDataReader,
  BReader: TDataReader,
{
  //  assert_eq!(
  //    a_reader.get_bit_size(),
  //    b_reader.get_bit_size(),
  //    "Comparing 2 binaries with different bit sizes {} ({}) vs {} ({})",
  //    a,
  //    a_reader.get_bit_size(),
  //    b,
  //    b_reader.get_bit_size()
  //  );
  let size = a_reader.get_bit_size();

  // Use rounded up byte-size, and then because the bit-sizes must be equal,
  // the last byte bits will have same size
  let n_bytes = size.get_byte_size_rounded_up().bytes();

  for i in 0..n_bytes {
    let a_byte = a_reader.read(i);
    let b_byte = b_reader.read(i);
    if a_byte != b_byte {
      return Ok(a_byte.cmp(&b_byte));
    }
  }
  // No differences we've been able to find
  return Ok(Ordering::Equal);
}

/// Deeper comparison of two values with different types
fn cmp_mixed_types(a: Term, b: Term) -> RtResult<Ordering> {
  unimplemented!("cmp_mixed_types {} vs {}", a, b)
}

/// Compare two cons (list) cells.
/// In case when first elements are equal and a deeper comparison is required,
/// we will store the position and return `EqResult::CompareNested`.
/// This will be pushed to a helper stack by the caller (`cmp_terms()`).
/// The function cannot fail.
unsafe fn cmp_cons(a: Term, b: Term) -> EqResult {
  let mut a_ptr = a.get_cons_ptr();
  let mut b_ptr = b.get_cons_ptr();

  loop {
    // Check the heads
    let a_head = (*a_ptr).hd();
    let b_head = (*b_ptr).hd();

    if !Term::is_same(a_head, b_head) {
      // Recurse into a.hd and b.hd, but push a.tl and b.tl to continue
      let continue_op = ContinueCompare::Cons {
        a: (*a_ptr).tl(),
        b: (*b_ptr).tl(),
      };
      return EqResult::CompareNested {
        a: a_head,
        b: b_head,
        state: continue_op,
      };
    }

    // See the tails
    let atl = (*a_ptr).tl();
    let btl = (*b_ptr).tl();

    if Term::is_same(atl, btl) {
      return EqResult::Concluded(Ordering::Equal);
    }
    if !atl.is_list() || !btl.is_list() {
      // Just do a regular compare of `a.tl` vs `b.tl`
      let continue_op = ContinueCompare::AnyType { a: atl, b: btl };
      return EqResult::CompareNested {
        a: atl,
        b: btl,
        state: continue_op,
      };
    }

    // Take the next linked cons cell and continue comparing
    a_ptr = atl.get_cons_ptr();
    b_ptr = btl.get_cons_ptr();
  }
}

// fn cmp_terms_box(a: Term, b: Term) -> RtResult<EqResult> {
//  println!("Comparing {} vs. {}", a, b);
//  let a_ptr = a.get_box_ptr::<boxed::BoxHeader>();
//  let b_ptr = b.get_box_ptr::<boxed::BoxHeader>();
//
//
//  // TODO: see if cmp_terms_immed_box can be useful
//  unimplemented!("eq_terms_box")
//}
