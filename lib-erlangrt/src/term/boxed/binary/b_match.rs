use crate::term::{boxed, lterm::LTerm};

/// Match buffer is a part of `BinaryMatchState`
pub struct BinaryMatchBuffer {
  pub orig: LTerm,
  pub base: *const u8,
  pub offset: usize,
  pub bit_size: usize,
}

/// Matchstate is stored on heap as a heap object. Followed by 1 or more save
/// offset `LTerm`s.
pub struct BinaryMatchState {
  pub bin_header: boxed::binary::Binary,
  pub mb: BinaryMatchBuffer,
}
