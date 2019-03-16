use crate::term::{
  boxed,
  value::{PrimaryTag, Term, LARGEST_SMALL, SMALLEST_SMALL, TERM_TAG_BITS},
};

impl Term {
  // === === SMALL INTEGERS === === ===
  //

  #[inline]
  pub fn is_integer(self) -> bool {
    self.is_small() || self.is_big_int()
  }

  /// Check whether the value is a small integer
  #[inline]
  pub fn is_small(self) -> bool {
    self.get_term_tag() == PrimaryTag::SMALL_INT
  }

  #[inline]
  pub const fn make_char(c: char) -> Self {
    Self::make_small_unsigned(c as usize)
  }

  #[inline]
  pub const fn make_small_unsigned(val: usize) -> Self {
    Self::make_from_tag_and_value(PrimaryTag::SMALL_INT, val)
  }

  pub const fn small_0() -> Self {
    Self::make_from_tag_and_value(PrimaryTag::SMALL_INT, 0)
  }

  pub const fn small_1() -> Self {
    Self::make_from_tag_and_value(PrimaryTag::SMALL_INT, 1)
  }

  pub const fn make_small_signed(val: isize) -> Self {
    Self::make_from_tag_and_signed_value(PrimaryTag::SMALL_INT, val)
  }

  /// Check whether a signed isize fits into small integer range
  #[inline]
  pub fn small_fits(val: isize) -> bool {
    val >= SMALLEST_SMALL && val <= LARGEST_SMALL
  }

  #[inline]
  pub fn get_small_signed(self) -> isize {
    debug_assert!(
      self.is_small(),
      "Small is expected, got raw=0x{:x}",
      self.value
    );
    (self.value as isize) >> TERM_TAG_BITS
  }

  #[inline]
  pub fn get_small_unsigned(self) -> usize {
    debug_assert!(self.is_small());
    debug_assert!(
      (self.value as isize) >= 0,
      "term::small_unsigned is negative {}",
      self
    );
    self.get_term_val_without_tag()
  }

  // === === BIG INTEGERS === ===
  //

  /// Check whether a term is boxed and then whether it points to a word of
  /// memory tagged as float
  pub fn is_big_int(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_BIGINTEGER)
  }
}
