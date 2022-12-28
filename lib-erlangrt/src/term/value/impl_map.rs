//! Submodule implements Map features of an Erlang Term
use crate::term::{boxed, SpecialConst, SpecialTag, Term};

impl Term {
  #[inline]
  pub const fn empty_map() -> Self {
    Self::make_special(SpecialTag::CONST, SpecialConst::EMPTY_MAP.0)
  }

  /// Check whether a value is a map.
  pub fn is_map(self) -> bool {
    self == Self::empty_map() || self.is_boxed_of_type(boxed::BOXTYPETAG_MAP)
  }

  /// Check whether a value is a small map < 32 elements (Flat). Does NOT check
  /// that the value is a map (assert!) assuming that the caller has checked
  /// it by now.
  pub fn is_flat_map(self) -> bool {
    self == Self::empty_map() // || true
  }

  /// Check whether a value is a hash map >= 32 elements (HAMT). Does NOT check
  /// that the value is a map (assert!) assuming that the caller has checked
  /// it by now.
  pub fn is_hash_map(self) -> bool {
    false
  }

  pub fn map_size(self) -> usize {
    let p = self.get_box_ptr::<boxed::Map>();
    unsafe { (*p).get_count() }
  }
}
