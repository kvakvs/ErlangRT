//! Functions to manipulate an LTerm as a boxed value on a process heap.
//! Part of LTerm impl.

use rt_defs::Word;
use term::primary;


pub trait BoxedAspect {
  /// Check whether primary tag of a value is `TAG_BOX`.
  fn is_box(&self) -> bool;

  fn box_ptr(&self) -> *const Word;

  fn box_ptr_mut(&self) -> *mut Word;
}


impl BoxedAspect for super::LTerm {

  #[inline]
  fn is_box(&self) -> bool {
    self.primary_tag() == primary::TAG_BOX
  }


  #[inline]
  fn box_ptr(&self) -> *const Word {
    assert!(self.is_box());
    primary::pointer(self.value)
  }


  #[inline]
  fn box_ptr_mut(&self) -> *mut Word {
    assert!(self.is_box());
    primary::pointer_mut(self.value)
  }
}


/// From a pointer to heap create a generic box
#[inline]
pub fn make_box(ptr: *const Word) -> super::LTerm {
  super::LTerm { value: primary::make_box_raw(ptr) }
}
