//!
//! Helper module defines types used everywhere in the VM runtime
//!
use num;

pub type Word = usize;

/// Replace with appropriate f32 or fixed/compact for embedded platform
pub type Float = f64;

/// Represents either Word or a BigInteger
#[derive(Debug)]
pub enum Integral {
  Word(Word),
  BigInt(num::BigInt),
}
