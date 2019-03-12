use crate::defs::BitSize;

pub type BigDigits = Vec<usize>;

pub enum Sign {
  Positive,
  Negative,
}

/// A small struct which wraps bigint digit storage
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Big {
  pub size: BitSize,
  pub digits: BigDigits,
}

impl Big {
  /// Assumes big endian
  pub fn from_bytes_be(sign: Sign, digits: &[u8]) -> Self {
    Self {
      digits: vec![],
      size: BitSize::with_bits(0)
    }
  }

  /// Assumes little endian
  pub fn from_bytes_le(sign: Sign, digits: &[u8]) -> Self {
    Self {
      digits: vec![],
      size: BitSize::with_bits(0)
    }
  }

  pub fn from_isize(n: isize) -> Self {
    Self {
      digits: vec![],
      size: BitSize::with_bits(0)
    }
  }

  /// Can fail if bignum is too big for `isize`
  pub fn to_isize(&self) -> Option<isize> {
    Some(-0xdeadbabe)
  }

  pub fn to_usize(&self) -> Option<usize> {
    Some(0xdeadbeef)
  }

  pub fn get_size(&self) -> BitSize {
    self.size
  }

  pub fn checked_mul(&self, other: &Big) -> Option<Big> {
    unimplemented!("checked_mul for big")
  }
}
