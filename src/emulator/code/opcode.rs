//! Opcode enum wraps the opcode from opcode table. Special conversion rules
//! may be used when running in debug mode for extra safety checks, in release
//! no checks are done and simple opcode is stored.
//!
use defs::Word;
use beam::gen_op;

#[cfg(debug_assertions)]
use term::immediate;


// TODO: Possibly will have to extend this type to fit new optimized opcodes.
pub type RawOpcode = u8;


/// Convert the raw (numeric) opcode into memory format. This is a simple
/// value for release build but is decorated for debug build. We use a special
/// subtag of Immediate3.
#[inline]
#[cfg(debug_assertions)]
pub fn to_memory_word(raw: RawOpcode) -> Word {
  immediate::create_imm3(raw as Word,
                         immediate::IMM3_OPCODE_PREFIX)
}


#[inline]
#[cfg(not(debug_assertions))]
pub fn to_memory_word(raw: RawOpcode) -> Word {
  raw as Word
}


/// Convert the opcode from memory format into raw. For debug build it was
/// decorated as Immediate3.
#[inline]
#[cfg(debug_assertions)]
pub fn from_memory_word(m: Word) -> RawOpcode {
  assert_eq!(immediate::get_imm3_tag(m), immediate::TAG_IMM3_OPCODE,
             "Opcode 0x{:x} from code memory must be tagged as IMM3_OPCODE", m);
  let raw = immediate::get_imm3_value(m);
  debug_assert!(raw <= gen_op::OPCODE_MAX as Word);
  raw as RawOpcode
}


#[inline]
#[cfg(not(debug_assertions))]
pub fn from_memory_word(m: Word) -> RawOpcode {
  m as RawOpcode
}


/// Debug version: Load an opcode and assert that it is decorated as Immediate3.
#[inline]
#[cfg(debug_assertions)]
pub fn from_memory_ptr(p: *const Word) -> RawOpcode {
  let m = unsafe { *p };
  assert_eq!(immediate::get_imm3_tag(m), immediate::TAG_IMM3_OPCODE,
             "Opcode 0x{:x} from code memory {:p} must be tagged as IMM3_OPCODE",
             m, p);
  let raw = immediate::get_imm3_value(m);
  debug_assert!(raw <= gen_op::OPCODE_MAX as Word);
  raw as RawOpcode
}


/// Release version. Load an opcode.
#[inline]
#[cfg(not(debug_assertions))]
pub fn from_memory_ptr(p: *const Word) -> RawOpcode {
  unsafe { *p as RawOpcode }
}

//
// Testing section
//

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_opcode_word() {
    for i in 0..gen_op::OPCODE_MAX {
      let memw = to_memory_word(i);
      let opc = from_memory_word(memw);
      assert_eq!(opc, i);
    }
  }
}