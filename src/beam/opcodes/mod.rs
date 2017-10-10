pub mod op_execution;
pub mod op_memory;
pub mod op_data;

pub use beam::opcodes::op_execution::*;
pub use beam::opcodes::op_memory::*;
pub use beam::opcodes::op_data::*;


use beam::gen_op;
use defs::Word;
use emulator::code::opcode::RawOpcode;


/// Run a check whether opcode is not too large (within the supported range).
// TODO: Maybe #[inline] but now let compiler decide
#[cfg(debug_assertions)]
pub fn assert_arity(op: RawOpcode, val: Word) {
  assert!(op < gen_op::OPCODE_MAX, "Opcode is too large");
  assert_eq!(gen_op::ARITY_MAP[op as usize] as Word, val,
             "Opcode {}={} arity is expected to be {}",
             gen_op::opcode_name(op as u8), op, val);
}
#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn assert_arity(_op: RawOpcode, _val: Word) {}


/// Display an error about opcode not supported/not implemented.
pub fn unknown_opcode(op: RawOpcode) {
  panic!("vm_dispatch: Opcode {:?} '{}' not implemented",
         op, gen_op::opcode_name(op))
}
