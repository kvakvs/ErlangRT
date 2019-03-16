use crate::term::{
  boxed,
  value::{Term, PrimaryTag},
};

impl Term {
  // === === TUPLES === === ===
  //

  pub fn is_tuple(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_TUPLE)
  }

  // This function only has debug check, in release it will not do any checking
  #[inline]
  pub fn get_tuple_ptr(self) -> *const boxed::Tuple {
    debug_assert!(self.is_tuple(), "Value is not a tuple: {}", self);
    (self.value & (!PrimaryTag::TAG_MASK)) as *const boxed::Tuple
  }

  // This function only has debug check, in release it will not do any checking
  #[inline]
  pub fn get_tuple_ptr_mut(self) -> *mut boxed::Tuple {
    debug_assert!(self.is_tuple(), "Value is not a tuple: {}", self);
    (self.value & (!PrimaryTag::TAG_MASK)) as *mut boxed::Tuple
  }
}
