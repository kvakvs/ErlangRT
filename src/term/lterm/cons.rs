//! Utility functions for handling lists
use crate::{
  defs::exc_type::ExceptionType,
  emulator::{gen_atoms, heap::Heap},
  fail::{Error, RtResult},
  term::{boxed, lterm::lterm_impl::LTerm, term_builder::ListBuilder},
};

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
        return Err(Error::Exception(ExceptionType::Error, gen_atoms::BADARG));
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
  debug_assert_ne!(src, LTerm::nil());
  let mut lb = ListBuilder::new(hp)?;
  let mut src_p = src.get_cons_ptr();

  // Copy elements one by one
  loop {
    lb.set((*src_p).hd());
    let src_tl = (*src_p).tl();
    if !src_tl.is_cons() {
      return Ok((lb.make_term(), lb.get_write_p()));
    }
    lb.next()?;
    src_p = src_tl.get_cons_ptr();
  }
}

/// For each list element run the function. Tail element (usually NIL) is ignored.
pub fn for_each<T>(lst: LTerm, mut func: T)
where
  T: FnMut(LTerm),
{
  if lst == LTerm::nil() {
    return;
  }
  let mut p = lst.get_cons_ptr();
  loop {
    let hd_el = unsafe { (*p).hd() };
    func(hd_el);

    let tl_el = unsafe { (*p).tl() };
    if tl_el.is_cons() {
      // for a list tail element step forward
      p = tl_el.get_cons_ptr();
    } else {
      // for a non list end here
      break;
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
  if lst == LTerm::nil() {
    return false;
  }
  let mut p = lst.get_cons_ptr();
  loop {
    let hd_el = unsafe { (*p).hd() };
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
