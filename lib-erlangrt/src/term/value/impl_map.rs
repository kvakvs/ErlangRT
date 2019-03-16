use crate::term::{boxed, value::Term};

impl Term {
  // === === MAP === ===
  //

  /// Check whether a value is a map.
  pub fn is_map(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_MAP)
  }

  /// Check whether a value is a small map < 32 elements (Flat). Does NOT check
  /// that the value is a map (debug_assert!) assuming that the caller has checked
  /// it by now.
  pub fn is_flat_map(self) -> bool {
    false
  }

  /// Check whether a value is a hash map >= 32 elements (HAMT). Does NOT check
  /// that the value is a map (debug_assert!) assuming that the caller has checked
  /// it by now.
  pub fn is_hash_map(self) -> bool {
    false
  }

  pub fn map_size(self) -> usize {
    0
  }
}
