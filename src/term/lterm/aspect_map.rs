//! Functions to manipulate an LTerm as an boxed pointer to an Erlang Map.
//! Part of LTerm impl.

//use rt_defs::Word;
//use term::immediate;


pub trait MapAspect {
  /// Check whether a value is a map.
  fn is_map(&self) -> bool { false }

  /// Check whether a value is a small map < 32 elements (Flat). Does NOT check
  /// that the value is a map (assert!) assuming that the caller has checked
  /// it by now.
  fn is_flat_map(&self) -> bool { false }

  /// Check whether a value is a hash map >= 32 elements (HAMT). Does NOT check
  /// that the value is a map (assert!) assuming that the caller has checked
  /// it by now.
  fn is_hash_map(&self) -> bool { false }

  fn map_size(&self) -> usize { 0 }
}


impl MapAspect for super::LTerm {
}
