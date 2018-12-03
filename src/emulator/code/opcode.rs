//! Opcode enum wraps the opcode from opcode table. Special conversion rules
//! may be used when running in debug mode for extra safety checks, in release
//! no checks are done and simple opcode is stored.
//!
use beam::gen_op;
use rt_defs::Word;
use term::lterm::{LTerm, SPECIALTAG_OPCODE};

//use std::iter;


// TODO: Possibly will have to extend this type to fit new optimized opcodes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RawOpcode(pub u8);

//impl iter::Step for RawOpcode {
//  fn steps_between(start: &Self, end: &Self) -> Option<usize> {
//    let RawOpcode(s) = start;
//    let RawOpcode(e) = end;
//    Some((e - s) as usize)
//  }
//
//  fn replace_one(&mut self) -> Self {
//    unimplemented!()
//  }
//
//  fn replace_zero(&mut self) -> Self {
//    unimplemented!()
//  }
//
//  fn add_one(&self) -> Self {
//    let RawOpcode(x) = self;
//  }
//
//  fn sub_one(&self) -> Self {
//    unimplemented!()
//  }
//
//  fn add_usize(&self, n: usize) -> Option<Self> {
//    unimplemented!()
//  }
//}


impl RawOpcode {
  pub fn get(self) -> u8 {
    let RawOpcode(raw8) = self;
    raw8
  }
}
//pub type RawOpcode = u8;


/// Convert the raw (numeric) opcode into memory format. This is a simple
/// value for release build but is decorated for debug build. We use special
/// term type for this.
#[inline]
#[cfg(debug_assertions)]
pub fn to_memory_word(raw: RawOpcode) -> Word {
  let RawOpcode(raw8) = raw;
  LTerm::make_special(SPECIALTAG_OPCODE, raw8 as Word).raw()
}


#[inline]
#[cfg(not(debug_assertions))]
pub fn to_memory_word(raw: RawOpcode) -> Word {
  raw as Word
}


/// Convert the opcode from memory format into raw. For debug build it was
/// decorated as Immediate3.
#[cfg(debug_assertions)]
pub fn from_memory_word(m: Word) -> RawOpcode {
  let as_term = LTerm::from_raw(m);
  debug_assert_eq!(as_term.get_special_tag(), SPECIALTAG_OPCODE,
                   "Opcode 0x{:x} from code memory must be tagged as Special/Opcode",
                   m);
  debug_assert!(as_term.get_special_value() < 256);
  let opc = RawOpcode(as_term.get_special_value() as u8);
  debug_assert!(opc <= gen_op::OPCODE_MAX);
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
  debug_assert_eq!(as_term.get_special_tag(), SPECIALTAG_OPCODE,
                   "Opcode 0x{:x} from code memory {:p} must be tagged as Special/Opcode",
                   m, p);
  debug_assert!(as_term.get_special_value() < 256);
  let opc = RawOpcode(as_term.get_special_value() as u8);
  debug_assert!(opc <= gen_op::OPCODE_MAX);
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
    for i in 0..gen_op::OPCODE_MAX.get() {
      let memw = to_memory_word(RawOpcode(i));
      let opc = from_memory_word(memw);
      assert_eq!(opc, RawOpcode(i));
    }
  }
}