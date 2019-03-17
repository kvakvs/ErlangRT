//! Code Pointer (Continuation Pointer) term aspect
//! A CP term is a box pointer with highest 63rd bit set.
use crate::{
  defs,
  term::value::{PrimaryTag, Term},
};

impl Term {
  /// This bit is set on boxed values which are CP pointers
  pub const HIGHEST_BIT_CP: usize = 1usize << (defs::WORD_BITS - 1);

  #[inline]
  pub fn make_cp<T>(p: *const T) -> Self {
    assert_eq!(
      p as usize & PrimaryTag::TAG_MASK,
      0,
      "Creating CP: Pointer must be aligned to usize"
    );
    let tagged_p = (p as usize) | Self::HIGHEST_BIT_CP;
    Self::make_boxed(tagged_p as *const T)
  }

  #[inline]
  pub fn is_cp(self) -> bool {
    if !self.is_boxed() {
      return false;
    }
    self.value & Self::HIGHEST_BIT_CP == Self::HIGHEST_BIT_CP
  }

  pub fn get_cp_ptr<T>(self) -> *const T {
    debug_assert_eq!(
      self.value & Self::HIGHEST_BIT_CP,
      Self::HIGHEST_BIT_CP,
      "A CP pointer must have its highest bit set"
    );
    // Trust Self::get_box_ptr to do the bit magic then remove the highest bit
    let p_val = self.get_box_ptr_unchecked::<T>() as usize;
    (p_val & (Self::HIGHEST_BIT_CP - 1)) as *const T
  }
}
