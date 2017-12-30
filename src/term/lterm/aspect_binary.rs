use term::immediate;
use term::lterm::*;
use term::raw::*;


/// Implements features of `LTerm` related to binary values.
pub trait BinaryAspect {
  unsafe fn is_binary(&self) -> bool;
  fn is_empty_binary(&self) -> bool;
}


impl BinaryAspect for super::LTerm {

  unsafe fn is_binary(&self) -> bool {
    if self.is_empty_binary() { return true }
    if !self.is_box() { return false }

    // from_term should be efficient enough (only type cast and one comparison)
    HOBinary::from_term(*self).is_ok()
  }


  /// Check whether a value is an empty binary.
  #[inline]
  fn is_empty_binary(&self) -> bool {
    self.value == immediate::IMM2_SPECIAL_EMPTY_BIN_RAW
  }
}


/// Create an empty binary value.
#[inline]
pub fn empty_binary() -> super::LTerm {
  super::LTerm { value: immediate::IMM2_SPECIAL_EMPTY_BIN_RAW }
}
