//! Opcode enum wraps the opcode from opcode table. Special conversion rules
//! may be used when running in debug mode for extra safety checks, in release
//! no checks are done and simple opcode is stored.
use crate::{
  beam::gen_op,
  defs::Word,
  term::{SpecialTag, Term},
};

// TODO: Possibly will have to extend this type to fit new optimized opcodes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RawOpcode(pub u8);

impl RawOpcode {
  pub fn get(self) -> u8 {
    let RawOpcode(raw8) = self;
    raw8
  }
}

/// Convert the raw (numeric) opcode into memory format. This is a simple
/// value for release build but is decorated for debug build. We use special
/// term type for this.
#[inline]
#[cfg(debug_assertions)]
pub fn to_memory_word(raw: RawOpcode) -> Word {
  let RawOpcode(raw8) = raw;
  Term::make_special(SpecialTag::OPCODE, raw8 as Word).raw()
}

#[inline]
#[cfg(not(debug_assertions))]
pub fn to_memory_word(raw: RawOpcode) -> Word {
  raw.0 as Word
}

/// Convert the opcode from memory format into raw. For debug build it was
/// decorated as Immediate3.
#[cfg(debug_assertions)]
pub fn from_memory_word(m: Word) -> RawOpcode {
  let as_term = Term::from_raw(m);
  debug_assert_eq!(
    as_term.get_special_tag(),
    SpecialTag::OPCODE,
    "Opcode 0x{m:x} from code memory must be tagged as Special/Opcode");
  debug_assert!(
    as_term.get_opcode_value() <= gen_op::OPCODE_MAX.0 as usize,
    "Value for rawOpcode is too big, get {} expected max {}",
    as_term.get_opcode_value(),
    gen_op::OPCODE_MAX.0
  );
  let opc = RawOpcode(as_term.get_opcode_value() as u8);
  opc as RawOpcode
}

#[inline]
#[cfg(not(debug_assertions))]
pub fn from_memory_word(m: Word) -> RawOpcode {
  RawOpcode(m as u8)
}

/// Debug version: Load an opcode and assert that it is decorated as Immediate3.
#[inline]
#[cfg(debug_assertions)]
pub fn from_memory_ptr(ptr: *const Word) -> RawOpcode {
  let mem_content = unsafe { *ptr };
  let as_term = Term::from_raw(mem_content);
  debug_assert!(
    as_term.is_special(),
    "Disasm: Opcode from memory {:p} must be tagged as Special",
    ptr
  );
  debug_assert_eq!(
    as_term.get_special_tag(),
    SpecialTag::OPCODE,
    "Disasm: Opcode 0x{mem_content:x} from code memory {ptr:p} must be tagged as Special/Opcode");
  debug_assert!(
    as_term.get_opcode_value() <= gen_op::OPCODE_MAX.0 as usize,
    "Value for rawOpcode is too big, get {} expected max {}",
    as_term.get_opcode_value(),
    gen_op::OPCODE_MAX.0
  );
  let opc = RawOpcode(as_term.get_opcode_value() as u8);
  opc as RawOpcode
}

/// Release version. Load an opcode.
#[inline]
#[cfg(not(debug_assertions))]
pub fn from_memory_ptr(p: *const Word) -> RawOpcode {
  unsafe { RawOpcode(*p as u8) }
}

// Testing section
//

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_opcode_word() {
    for i in 0..gen_op::OPCODE_MAX.get() {
      let memw = to_memory_word(RawOpcode(i));
      let opc = from_memory_word(memw);
      assert_eq!(opc, RawOpcode(i));
    }
  }
}
