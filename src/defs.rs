//!
//! Helper module defines types used everywhere in the VM runtime
//!
use num;
use num::ToPrimitive;

pub type Word = usize;
pub type SWord = isize;

/// Replace with appropriate f32 or fixed/compact for embedded platform
pub type Float = f64;

#[cfg(target_pointer_width = "32")]
pub const WORD_BITS: Word = 32;

#[cfg(target_pointer_width = "64")]
pub const WORD_BITS: Word = 64;

/// Max value for a positive small integer packed into immediate2 low level
/// Term. Assume word size minus 4 bits for imm1 tag and 1 for sign
pub const MAX_POS_SMALL: Word = 1 << (WORD_BITS - 4 - 1);
pub const MAX_XREGS: Word = 256;

/// Represents either Word or a BigInteger
#[derive(Debug, Eq, PartialEq)]
pub enum Integral {
  Word(Word),
  BigInt(num::BigInt),
}

impl Integral {
  pub fn from_big(big: num::BigInt) -> Integral {
    if big.bits() < WORD_BITS {
      return Integral::Word(big.to_usize().unwrap());
    }
    Integral::BigInt(big)
  }
}
