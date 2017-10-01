//!
//! Helper module defines types used everywhere in the VM runtime
//!
use num;
use num::ToPrimitive;

pub type Word = usize;

/// Replace with appropriate f32 or fixed/compact for embedded platform
pub type Float = f64;

#[cfg(target_pointer_width = "32")]
fn platf_bits() -> Word { 32 }

#[cfg(target_pointer_width = "64")]
fn platf_bits() -> Word { 64 }

/// Represents either Word or a BigInteger
#[derive(Debug, Eq, PartialEq)]
pub enum Integral {
  Word(Word),
  BigInt(num::BigInt),
}

impl Integral {
  pub fn from_big(big: num::BigInt) -> Integral {
    if big.bits() < platf_bits() {
      return Integral::Word(big.to_usize().unwrap());
    }
    Integral::BigInt(big)
  }
}
