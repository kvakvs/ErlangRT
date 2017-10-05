//! Instruction pointer is used to refer directly to the running code. Also
//! it holds a reference counter to the function where it executes.

/// Defines a code position in module by referring to a function and an offset.
/// Function pointer is refcounted, and function points to a module which is
/// also refcounted.
use emulator::function;
use defs::Word;

pub struct InstrPointer {
  fun: function::Ptr,
  instr_index: Word,
}

impl InstrPointer {
  pub fn new(fun: function::Ptr, instr_index: Word) -> InstrPointer {
    InstrPointer { fun, instr_index }
  }
}
