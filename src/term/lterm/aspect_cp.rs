//! Code Pointer manipulation.
//! CP is tagged as Boxed + Top bit set.

use defs::Word;
use defs;
use term::primary;
use term::lterm::aspect_boxed::{BoxedAspect, make_box};


/// Represents operations on LTerm which contains/is a CP value.
pub trait CpAspect {
  fn is_cp(&self) -> bool;
  fn cp_get_ptr(&self) -> *const Word;
}


impl CpAspect for super::LTerm {
  #[inline]
  fn is_cp(&self) -> bool {
    self.is_box() && (self.value & defs::TAG_CP == defs::TAG_CP)
  }


  #[inline]
  fn cp_get_ptr(&self) -> *const Word {
    assert!(self.is_box(), "CP value must be boxed (have {})", self);
    assert_eq!(self.value & defs::TAG_CP, defs::TAG_CP,
               "CP value must have its top bit set (have 0x{:x})", self.value);
    let untagged_p = self.value & !(defs::TAG_CP | primary::PRIM_MASK);
    untagged_p as *const Word
  }
}


#[inline]
pub fn make_cp(p: *const Word) -> super::LTerm {
  let tagged_p = (p as Word) | defs::TAG_CP;
  make_box(tagged_p as *const Word)
}
