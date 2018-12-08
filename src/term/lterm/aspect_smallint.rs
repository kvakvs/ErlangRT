////! Do not import this file directly, use `use term::lterm::*;` instead.
//
//use defs::{SWord, Word, MAX_UNSIGNED_SMALL, MIN_NEG_SMALL, MAX_POS_SMALL};
//use term::immediate;
//
//
//pub trait SmallintAspect {
//  /// Check whether a value is a small integer.
//  fn is_small(&self) -> bool;
//  fn small_get_s(&self) -> SWord;
//  fn small_get_u(&self) -> Word;
//}
//
//
//impl SmallintAspect for super::LTerm {
//
//  #[inline]
//  fn is_small(&self) -> bool {
//    immediate::is_small_raw(self.value)
//  }
//
//
//  #[inline]
//  fn small_get_s(&self) -> SWord {
//    immediate::get_imm1_value_s(self.value)
//  }
//
//
//  #[inline]
//  fn small_get_u(&self) -> Word {
//    immediate::get_imm1_value(self.value)
//  }
//
//}
//
//
//#[inline]
//pub fn make_small_u(n: Word) -> super::LTerm {
//  assert!(n <= MAX_UNSIGNED_SMALL,
//          "make_small_u n=0x{:x} <= limit=0x{:x}", n, MAX_UNSIGNED_SMALL);
//  super::LTerm { value: immediate::make_small_raw(n as SWord) }
//}
//
//
//#[inline]
//pub fn make_small_s(n: SWord) -> super::LTerm {
//  // TODO: Do the proper min neg small
//  assert!(fits_small(n),
//          "make_small_s: n=0x{:x} does not fit small range", n);
//  super::LTerm { value: immediate::make_small_raw(n) }
//}
//
//
///// Check whether a signed value `n` will fit into small integer range.
//#[inline]
//pub fn fits_small(n: SWord) -> bool {
//  n >= MIN_NEG_SMALL && n <= MAX_POS_SMALL
//}
