//! Utility functions for handling lists
use crate::{
  defs::{exc_type::ExceptionType, sizes::ByteSize},
  emulator::{gen_atoms, heap::heap_trait::THeap},
  fail::{RtErr, RtResult},
  term::{boxed, term_builder::ListBuilder, value::Term},
};

// TODO: Rewrite this with for_each when i can think clear again
pub fn list_length(val: Term) -> RtResult<usize> {
  if val == Term::nil() {
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
      if tl != Term::nil() {
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
  src: Term,
  hp: &mut THeap,
) -> RtResult<(Term, *mut boxed::Cons)> {
  let mut lb = ListBuilder::new()?;

  // Copy elements one by one
  if let Ok(Some(tail)) = for_each(src, |elem| {
    lb.append(elem, hp)?;
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
pub fn for_each<T>(lst: Term, mut func: T) -> RtResult<Option<Term>>
where
  T: FnMut(Term) -> RtResult<()>,
{
  if lst == Term::nil() {
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
pub fn any<T>(lst: Term, mut predicate: T) -> bool
where
  T: FnMut(Term) -> bool,
{
  // Not found
  if lst == Term::nil() {
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
pub fn find_first<T>(lst: Term, mut predicate: T) -> Option<*const boxed::Cons>
where
  T: FnMut(Term) -> bool,
{
  // Not found
  if lst == Term::nil() {
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
pub unsafe fn rust_str_to_list(s: &String, hp: &mut THeap) -> RtResult<Term> {
  let mut lb = ListBuilder::new()?;
  for pos_char in s.char_indices() {
    let ch = pos_char.1 as usize;
    let char_term = Term::make_small_unsigned(ch);
    lb.append(char_term, hp)?;
  }
  Ok(lb.make_term())
}

/// Given an integer Term, convert it to a string with `base`.
pub unsafe fn integer_to_list(val: Term, hp: &mut THeap) -> RtResult<Term> {
  if val.is_big_int() {
    panic!("TODO: impl integer_to_list for bigint");
  }

  let base = 10isize;
  let mut i_val = val.get_small_signed();
  let mut lb = ListBuilder::new()?;

  let sign = if i_val < 0 {
    i_val = -i_val;
    true
  } else {
    false
  };

  if i_val == 0 {
    lb.append(Term::make_char('0'), hp)?;
  } else {
    loop {
      let digit = '0' as usize + (i_val % base) as usize;
      let digit_term = Term::make_small_unsigned(digit);
      if i_val == 0 {
        break;
      }
      lb.prepend(digit_term, hp)?;
      i_val /= base;
    } // loop

    if sign {
      lb.prepend(Term::make_char('-'), hp)?;
    }
  } // if not 0

  return Ok(lb.make_term());
}

pub fn get_iolist_size(list: Term) -> ByteSize {
  let mut result = ByteSize::new(0);
  for_each(list, |elem| {
    if elem.is_small() {
      // Any small integer even larger than 256 counts as 1 byte
      result.add(1);
    } else if elem.is_binary() {
      result.add_bytesize(unsafe { elem.binary_byte_size() });
    } else if elem.is_cons() {
      result.add_bytesize(get_iolist_size(elem));
    }
    Ok(())
  })
  .unwrap();
  // unwrap above may panic if you feed a non-iolist or OOM or something
  result
}
