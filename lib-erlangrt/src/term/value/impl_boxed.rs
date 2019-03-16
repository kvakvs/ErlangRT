use crate::{
  fail::{RtErr, RtResult},
  term::{
    boxed::{BoxHeader, BoxType},
    value::{PrimaryTag, Term},
  },
};

impl Term {
  // === === BOXED === === ===
  //

  // TODO: Some safety checks maybe? But oh well
  #[inline]
  pub fn make_boxed<T>(p: *const T) -> Self {
    let p_val = p as usize;
    assert_eq!(
      p_val & PrimaryTag::TAG_MASK,
      0,
      "Creating a boxed value from {:p} is only allowed for word aligned addresses, it would be 0x{:x} then",
      p,
      p_val & !PrimaryTag::TAG_MASK
    );
    Self {
      value: p_val | PrimaryTag::BOX_PTR.0,
    }
  }

  /// Check whether tag bits of a value equal to `PrimaryTag::BOX_PTR`
  #[inline]
  pub fn is_boxed(self) -> bool {
    self.get_term_tag() == PrimaryTag::BOX_PTR
  }

  /// Just raw decode value bits into a pointer, no safety checks
  #[inline]
  pub fn get_box_ptr_unchecked<T>(self) -> *const T {
    (self.value & !PrimaryTag::TAG_MASK) as *const T
  }
  /// Just raw decode value bits into a mut pointer, no safety checks
  #[inline]
  pub fn get_box_ptr_unchecked_mut<T>(self) -> *mut T {
    (self.value & !PrimaryTag::TAG_MASK) as *mut T
  }

  #[inline]
  pub fn get_box_ptr<T>(self) -> *const T {
    assert!(self.is_boxed());
    self.debug_assert_box_pointer_valid();
    self.get_box_ptr_unchecked::<T>()
  }

  #[inline]
  pub fn get_box_ptr_mut<T>(self) -> *mut T {
    assert!(self.is_boxed());
    self.debug_assert_box_pointer_valid();
    self.get_box_ptr_unchecked_mut::<T>()
  }

  pub fn get_box_ptr_safe<T>(self) -> RtResult<*const T> {
    if !self.is_boxed() {
      return Err(RtErr::TermIsNotABoxed);
    }
    self.debug_assert_box_pointer_valid();
    Ok(self.value as *const T)
  }

  #[cfg(not(debug_assertions))]
  #[inline]
  fn debug_assert_box_pointer_valid(&self) {}

  #[cfg(debug_assertions)]
  fn debug_assert_box_pointer_valid(&self) {
    // Additional check in debug, boxheader has an extra guard word stored
    let boxheader = self.value as *const BoxHeader;
    unsafe { (*boxheader).ensure_valid() };
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
