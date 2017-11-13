//! Functions to manipulate an LTerm as an Erlang CONS cell (two words on heap,
//! which contain a head and a tail). Part of LTerm impl.

use defs::Word;
use term::immediate;
use term::primary;
use term::raw::{ConsPtr, ConsPtrMut};
use term::lterm::aspect_smallint::SmallintTerm;

use std::ptr;


fn module() -> &'static str { "lterm/cons_term: " }


/// Represents cons/list/NIL aspects of an LTerm.
pub trait ConsTerm {
  /// Check whether primary tag of a value is `TAG_CONS`.
  fn is_cons(&self) -> bool;

  fn is_list(&self) -> bool;

  /// Check whether a value is a NIL \[ \]
  fn is_nil(&self) -> bool;

  /// Get a proxy object for read-only accesing the cons contents.
  fn cons_get_ptr(&self) -> ConsPtr;

  /// Get a proxy object for looking and modifying cons contents.
  fn cons_get_ptr_mut(&self) -> ConsPtrMut;

  /// Iterate through a string and check if it only contains small integers
  /// and that each integer is in ASCII range.
  unsafe fn cons_is_ascii_string(&self) -> bool;
}


impl ConsTerm for super::LTerm {
  #[inline]
  fn is_cons(&self) -> bool {
    self.primary_tag() == primary::TAG_CONS
  }


  #[inline]
  fn is_list(&self) -> bool {
    self.is_cons() || self.is_nil()
  }


  #[inline]
  fn is_nil(&self) -> bool {
    self.value == immediate::IMM2_SPECIAL_NIL_RAW
  }


  fn cons_get_ptr(&self) -> ConsPtr {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_CONS,
               "{}cons_get_ptr: A cons is expected", module());
    let boxp = primary::pointer(v);
    assert_ne!(boxp, ptr::null(), "null cons 0x{:x}", self.value);
    ConsPtr::from_pointer(boxp)
  }


  fn cons_get_ptr_mut(&self) -> ConsPtrMut {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_CONS,
               "{}cons_get_ptr_mut: A cons is expected", module());
    let boxp = primary::pointer_mut(v);
    ConsPtrMut::from_pointer(boxp)
  }


  unsafe fn cons_is_ascii_string(&self) -> bool {
    // TODO: List iterator
    let mut cons_p = self.cons_get_ptr();
    loop {
      let hd = cons_p.hd();
      if !hd.is_small() {
        return false
      }

      let hd_value = hd.small_get_s();
      if hd_value < 32 || hd_value >= 127 {
        return false
      }

      let tl = cons_p.tl();
      if !tl.is_cons() {
        // NIL [] tail is required for a true string
        return tl.is_nil()
      }
      cons_p = tl.cons_get_ptr();
    }
  }
}


/// From a pointer to heap create a cons box
#[inline]
pub fn make_cons(ptr: *const Word) -> super::LTerm {
  //assert_ne!(ptr, ptr::null());
  super::LTerm { value: primary::make_cons_raw(ptr) }
}

/// Create a NIL value.
#[inline]
pub fn nil() -> super::LTerm {
  super::LTerm { value: immediate::IMM2_SPECIAL_NIL_RAW }
}


pub const fn const_nil() -> super::LTerm {
  super::LTerm { value: immediate::IMM2_SPECIAL_NIL_RAW }
}
