use crate::{
  defs,
  term::value::{PrimaryTag, Term},
};

impl Term {
  // === === Code Pointer (Continuation Pointer) === ===
  //

  // XXX: Can shift value right by 3 bits (WORD_ALIGN_SHIFT)
  #[inline]
  pub fn make_cp<T>(p: *const T) -> Self {
    assert_eq!(
      p as usize & PrimaryTag::TAG_MASK,
      0,
      "Creating CP: Pointer must be aligned to usize"
    );
    let tagged_p = (p as usize) | defs::HIGHEST_BIT_CP;
    Self::from_raw(tagged_p | PrimaryTag::BOX_PTR.0)
  }

  #[inline]
  pub fn is_cp(self) -> bool {
    if !self.is_boxed() {
      return false;
    }
    self.value & defs::HIGHEST_BIT_CP == defs::HIGHEST_BIT_CP
  }

  pub fn get_cp_ptr<T>(self) -> *const T {
    debug_assert_eq!(
      self.value & defs::HIGHEST_BIT_CP,
      defs::HIGHEST_BIT_CP,
      "A CP pointer must have its highest bit set"
    );
    (self.value & !PrimaryTag::TAG_MASK & (defs::HIGHEST_BIT_CP - 1)) as *const T
  }
}
