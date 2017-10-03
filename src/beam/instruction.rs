//!
//! Opcodes declared as Rust enum, possibly good enough to last through the
//! prototype phase
//!
use term::low_level::LTerm;
use defs::Word;

/// Friendly instructions are a Rust enum, typesafe and generally cool
#[cfg(feature="friendly_instructions")]
pub enum Instr {
  Move { src: LTerm, dst: LTerm }
}

/// Unfriendly instructions are raw words interpreted as opcode packed with args
/// or a label pointer (in C OTP implementation)
#[cfg(not(feature="friendly_instructions"))]
pub type Instr = Word;
