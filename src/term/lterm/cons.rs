//! Utility functions for handling lists
use crate::{
  defs::ExceptionType,
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
