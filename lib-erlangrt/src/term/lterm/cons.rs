//! Utility functions for handling lists
use crate::{
  defs::exc_type::ExceptionType,
  emulator::{gen_atoms, heap::Heap},
  fail::{RtErr, RtResult},
  term::{boxed, lterm::lterm_impl::LTerm, term_builder::ListBuilder},
};

// TODO: Rewrite this with for_each when i can think clear again
pub fn list_length(val: LTerm) -> RtResult<usize> {
  if val == LTerm::nil() {
    return Ok(0);
  }

  let mut cons_p = val.get_cons_ptr();
  let mut count = 1;
  loop {
    let tl = unsafe { (*cons_p).tl() };

    if tl.is_cons() {
      count += 1;
      cons_p = tl.get_cons_ptr();
    } else {
      if tl != LTerm::nil() {
        return Err(RtErr::Exception(ExceptionType::Error, gen_atoms::BADARG));
      }
      return Ok(count);
    }
  }
}

/// Copies list `src` to heap `hp`.
/// Arg: `src` list, must not be NIL (check for it before calling this).
/// Returns: pointer to the list head (first element of the result tuple) and
/// the pointer to the last cell (second element).
/// OBSERVE that the tail of the returned copy is uninitialized memory.
pub unsafe fn copy_list_leave_tail(
  src: LTerm,
  hp: &mut Heap,
) -> RtResult<(LTerm, *mut boxed::Cons)> {
  let mut lb = ListBuilder::new(hp)?;

  // Copy elements one by one
  if let Ok(Some(tail)) = for_each(src, |elem| {
    lb.append(elem)?;
    Ok(())
  }) {
    return Ok((lb.make_term_with_tail(tail), lb.tail_p));
  }
  Ok((lb.make_term(), lb.tail_p))

  //  loop {
  //    lb.append((*src_p).hd());
  //    let src_tl = (*src_p).tl();
  //    if !src_tl.is_cons() {
  //      return Ok((lb.make_term(), lb.get_write_p()));
  //    }
  //    lb.next()?;
  //    src_p = src_tl.get_cons_ptr();
  //  }
}

/// For each list element run the function. Tail element (usually NIL) is ignored.
/// Returns: Tail element (NIL for proper list) or `None` for empty list
pub fn for_each<T>(lst: LTerm, mut func: T) -> RtResult<Option<LTerm>>
where
  T: FnMut(LTerm) -> RtResult<()>,
{
  if lst == LTerm::nil() {
    return Ok(None);
  }
  let mut p = lst.get_cons_ptr();
  loop {
    let hd_el = unsafe { (*p).hd() };
    func(hd_el)?;

    let tl_el = unsafe { (*p).tl() };
    if tl_el.is_cons() {
      // for a list tail element step forward
      p = tl_el.get_cons_ptr();
    } else {
      // for a non list end here
      return Ok(Some(tl_el));
    }
  }
}

/// Finds if any element of lst satisfies `predicate` function.
/// For each list element run the predicate until it returns true, then the
/// result becomes true. If predicate did not return true for any element,
/// the result becomes false.
pub fn any<T>(lst: LTerm, mut predicate: T) -> bool
where
  T: FnMut(LTerm) -> bool,
{
  // Not found
  if lst == LTerm::nil() {
    return false;
  }
  let mut p = lst.get_cons_ptr();
  loop {
    let hd_el = unsafe { (*p).hd() };
    // Found at *p
    if predicate(hd_el) {
      return true;
    }

    let tl_el = unsafe { (*p).tl() };
    if tl_el.is_cons() {
      // for a list tail element step forward
      p = tl_el.get_cons_ptr();
    } else {
      // for a non list end here
      return false;
    }
  }
}

/// Finds and returns first element of lst which satisfies `predicate`.
pub fn find_first<T>(lst: LTerm, mut predicate: T) -> Option<*const boxed::Cons>
  where
      T: FnMut(LTerm) -> bool,
{
  // Not found
  if lst == LTerm::nil() {
    return None;
  }
  let mut p = lst.get_cons_ptr();
  loop {
    let hd_el = unsafe { (*p).hd() };
    // Found at *p
    if predicate(hd_el) {
      return Some(p);
    }

    let tl_el = unsafe { (*p).tl() };
    if tl_el.is_cons() {
      // for a list tail element step forward
      p = tl_el.get_cons_ptr();
    } else {
      // for a non list end here
      return None;
    }
  }
}

/// Given Rust `String`, create list of characters on heap
// TODO: Optimize by adding new string type which is not a list?
pub unsafe fn rust_str_to_list(s: &String, hp: &mut Heap) -> RtResult<LTerm> {
  let mut lb = ListBuilder::new(hp)?;
  for pos_char in s.char_indices() {
    let ch = pos_char.1 as usize;
    lb.append(LTerm::make_small_unsigned(ch))?;
  }
  Ok(lb.make_term())
}

/// Given an integer LTerm, convert it to a string with `base`.
pub unsafe fn integer_to_list(val: LTerm, hp: &mut Heap) -> RtResult<LTerm> {
  if val.is_big_int() {
    panic!("TODO: impl integer_to_list for bigint");
  }

  let base = 10isize;
  let mut i_val = val.get_small_signed();
  let mut lb = ListBuilder::new(hp)?;

  let sign = if i_val < 0 {
    i_val = -i_val;
    true
  } else {
    false
  };

  if i_val == 0 {
    lb.append(LTerm::make_char('0'))?;
  } else {
    loop {
      let digit = '0' as usize + (i_val % base) as usize;
      let digit_term = LTerm::make_small_unsigned(digit);
      if i_val == 0 {
        break;
      }
      lb.prepend(digit_term)?;
      i_val /= base;
    } // loop

    if sign {
      lb.prepend(LTerm::make_char('-'))?;
    }
  } // if not 0

  return Ok(lb.make_term());
}
