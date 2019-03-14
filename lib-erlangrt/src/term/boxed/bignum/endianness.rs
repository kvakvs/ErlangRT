#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Endianness {
  Big,
  Little,
}

impl Endianness {
  #[cfg(target_endian = "big")]
  pub fn default() -> Self {
    Endianness::Big
  }
  #[cfg(target_endian = "little")]
  pub fn default() -> Self {
    Endianness::Little
  }
}
