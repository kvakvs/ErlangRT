use crate::term::{
  boxed,
  value::{PrimaryTag, Term, TERM_TAG_MASK},
};

impl Term {
  // === === LISTS/CONS CELLS === === ===
  //

  #[inline]
  pub fn is_list(self) -> bool {
    self == Self::nil() || self.is_cons()
  }

  /// Check whether the value is a CONS pointer
  #[inline]
  pub fn is_cons(self) -> bool {
    self.get_term_tag() == PrimaryTag::CONS_PTR
  }

  #[inline]
  pub fn get_cons_ptr(self) -> *const boxed::Cons {
    self.assert_is_not_boxheader_guard_value();
    debug_assert!(self.is_cons(), "Value is not a cons: {}", self);
    (self.value & (!TERM_TAG_MASK)) as *const boxed::Cons
  }

  #[inline]
  pub fn get_cons_ptr_mut(self) -> *mut boxed::Cons {
    self.assert_is_not_boxheader_guard_value();
    debug_assert!(self.is_cons(), "Value is not a cons: {}", self);
    (self.value & (!TERM_TAG_MASK)) as *mut boxed::Cons
  }


  /// Create a Term from pointer to Cons cell. Pass a pointer to `Term` or
  /// a pointer to `boxed::Cons`. Attempting to create cons cell to Null pointer
  /// will create NIL (`[]`)
  #[inline]
  pub fn make_cons<T>(p: *const T) -> Self {
    Self {
      value: if p.is_null() {
        return Self::nil();
      } else {
        (p as usize) | PrimaryTag::CONS_PTR.0
      },
    }
  }

  pub unsafe fn cons_is_ascii_string(self) -> bool {
    debug_assert!(self.is_cons());

    // TODO: List iterator
    let mut cons_p = self.get_cons_ptr();
    loop {
      let hd = (*cons_p).hd();
      if !hd.is_small() {
        return false;
      }

      let hd_value = hd.get_small_signed();
      if hd_value < 32 || hd_value >= 127 {
        return false;
      }

      let tl = (*cons_p).tl();
      if !tl.is_cons() {
        // NIL [] tail is required for a true string
        return tl == Self::nil();
      }
      cons_p = tl.get_cons_ptr();
    }
  }
}
