use crate::{
  defs::BitSize,
  term::{
    boxed::{self, binary::TBinary},
    Term,
  },
};

/// Stores binary write state.
/// The following opcodes which build a binary will refer to this state in order
/// to know the destination for adding more bits.
pub struct CurrentBinaryState {
  pub dst: Option<*mut dyn TBinary>,
  pub offset: BitSize,
}

impl CurrentBinaryState {
  pub fn new() -> Self {
    Self {
      dst: None,
      offset: BitSize::with_bits(0),
    }
  }

  /// After bs_init instruction this is reset to offset 0 and the new writable bin.
  pub fn reset(&mut self, dst: Term) {
    self.dst = Some(unsafe { boxed::Binary::get_trait_mut_from_term(dst) });
    self.offset = BitSize::with_bits(0);
  }

  #[inline]
  pub fn valid(&self) -> bool {
    self.dst.is_some()
  }
}
