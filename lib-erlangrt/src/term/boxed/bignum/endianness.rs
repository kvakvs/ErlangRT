#[derive(Debug, Clone)]
pub enum Endianness {
  Big,
  Little,
}

impl Endianness {
  #[cfg(target_endian = "big")]
  pub fn new() -> Self {
    Endianness::Big
  }
  #[cfg(target_endian = "little")]
  pub fn new() -> Self {
    Endianness::Little
  }
}
