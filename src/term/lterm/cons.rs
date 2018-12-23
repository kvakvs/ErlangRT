//! Utility functions for handling lists
use super::lterm_impl::LTerm;
use crate::{
  emulator::gen_atoms,
  fail::{Error, RtResult},
  defs::{ExceptionType}
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
