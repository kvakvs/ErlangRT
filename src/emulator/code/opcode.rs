//! Opcode enum wraps the opcode from opcode table. Special conversion rules
//! may be used when running in debug mode for extra safety checks, in release
//! no checks are done and simple opcode is stored.
//!
use beam::gen_op;
use rt_defs::SpecialTag;
use rt_defs::Word;
use term::lterm::LTerm;


// TODO: Possibly will have to extend this type to fit new optimized opcodes.
pub type RawOpcode = u8;


/// Convert the raw (numeric) opcode into memory format. This is a simple
/// value for release build but is decorated for debug build. We use special
/// term type for this.
#[inline]
#[cfg(debug_assertions)]
pub fn to_memory_word(raw: RawOpcode) -> Word {
  LTerm::make_special(SpecialTag::Opcode, raw).raw()
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
  let as_term = LTerm::from_raw(m);
  debug_assert_eq!(as_term.get_special_tag(), SpecialTag::Opcode,
                   "Opcode 0x{:x} from code memory must be tagged as Special/Opcode",
                   m);
  let opc = as_term.get_special_value();
  debug_assert!(opc <= gen_op::OPCODE_MAX as Word);
  opc as RawOpcode
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
  let as_term = LTerm::from_raw(m);
  debug_assert_eq!(as_term.get_special_tag(), SpecialTag::Opcode,
                   "Opcode 0x{:x} from code memory {:p} must be tagged as Special/Opcode",
                   m, p);
  let opc = as_term.get_special_value();
  debug_assert!(opc <= gen_op::OPCODE_MAX as Word);
  opc as RawOpcode
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