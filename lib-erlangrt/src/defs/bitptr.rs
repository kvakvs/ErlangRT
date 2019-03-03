use crate::defs::BitSize;

/// Points to byte data with possible non-zero bit offset.
pub struct BitDataPointer {
  pub data: &'static [u8],
  pub offset: BitSize,
}

impl BitDataPointer {
  pub fn new(data: &'static [u8], offset: BitSize) -> Self {
    Self { data, offset }
  }
}
