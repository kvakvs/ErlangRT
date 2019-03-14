use crate::{
  fail::{RtErr, RtResult},
  term::{
    boxed::{BoxHeader, BoxType},
    value::{Term, TERMTAG_BOXED},
  },
};

impl Term {
  // === === BOXED === === ===
  //

  // TODO: Some safety checks maybe? But oh well
  #[inline]
  pub fn make_boxed<T>(p: *const T) -> Self {
    Self { value: p as usize }
  }

  /// Check whether tag bits of a value equal to TAG_BOXED=0
  #[inline]
  pub fn is_boxed(self) -> bool {
    self.get_term_tag() == TERMTAG_BOXED
  }

  #[inline]
  pub fn get_box_ptr<T>(self) -> *const T {
    assert!(self.is_boxed());
    self.value as *const T
  }

  #[inline]
  pub fn get_box_ptr_mut<T>(self) -> *mut T {
    assert!(self.is_boxed());
    self.value as *mut T
  }

  pub fn get_box_ptr_safe<T>(self) -> RtResult<*const T> {
    if !self.is_boxed() {
      return Err(RtErr::TermIsNotABoxed);
    }
    Ok(self.value as *const T)
  }

  pub fn get_box_ptr_safe_mut<T>(self) -> RtResult<*mut T> {
    if !self.is_boxed() {
      return Err(RtErr::TermIsNotABoxed);
    }
    Ok(self.value as *mut T)
  }

  /// Checks boxed tag to be equal to t, also returns false if not a boxed.
  #[inline]
  pub fn is_boxed_of_type(self, t: BoxType) -> bool {
    self.is_boxed_of_(|boxtag| boxtag == t)
  }

  /// Extracts boxed tag and runs an inline predicate on its boxed tag, allows
  /// for checking multiple boxed tag values. Returns false if not a boxed.
  #[inline]
  pub fn is_boxed_of_<F>(self, pred: F) -> bool
  where
    F: Fn(BoxType) -> bool,
  {
    if !self.is_boxed() {
      return false;
    }
    let box_ptr = self.get_box_ptr::<BoxHeader>();
    let trait_ptr = unsafe { (*box_ptr).get_trait_ptr() };
    let tag = unsafe { (*trait_ptr).get_type() };
    pred(tag)
  }
}
