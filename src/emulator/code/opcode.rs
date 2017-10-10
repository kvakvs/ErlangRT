//! Opcode enum wraps the opcode from opcode table. Special conversion rules
//! may be used when running in debug mode for extra safety checks, in release
//! no checks are done and simple opcode is stored.
//!
use defs::Word;
use beam::gen_op;

#[cfg(debug_assertions)]
use term::immediate::imm3;


// TODO: Possibly will have to extend this type to fit new optimized opcodes.
pub type RawOpcode = u8;


/// Convert the raw (numeric) opcode into memory format. This is a simple
/// value for release build but is decorated for debug build. We use a special
/// subtag of Immediate3.
#[inline]
#[cfg(debug_assertions)]
pub fn to_memory_word(raw: RawOpcode) -> Word {
  imm3::create_imm3(raw as Word, imm3::IMM3_OPCODE_PREFIX)
}


//#[inline]
//#[cfg(not(debug_assertions))]
//pub fn to_memory_word(raw: RawOpcode) -> Word {
//  raw as Word
//}


/// Convert the opcode from memory format into raw. For debug build it was
/// decorated as Immediate3.
#[inline]
#[cfg(debug_assertions)]
pub fn from_memory_word(m: Word) -> RawOpcode {
  let raw = imm3::imm3_value(m);
  debug_assert!(raw <= gen_op::OPCODE_MAX as Word);
  raw as RawOpcode
}


//#[inline]
//#[cfg(not(debug_assertions))]
//pub fn to_memory_word(m: Word) -> RawOpcode {
//  m as RawOpcode
//}
