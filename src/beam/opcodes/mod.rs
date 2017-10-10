pub mod op_execution;
pub mod op_memory;

pub use beam::opcodes::op_execution::*;
pub use beam::opcodes::op_memory::*;

use beam::gen_op;
use defs::Word;
use emulator::code::opcode::RawOpcode;


// TODO: Maybe #[inline] but now let compiler decide
pub fn assert_arity(op: RawOpcode, val: Word) {
  assert!(op < gen_op::OPCODE_MAX, "Opcode is too large");
  assert_eq!(gen_op::ARITY_MAP[op as usize] as Word, val,
             "Opcode {}={} arity is expected to be {}",
             gen_op::opcode_name(op as u8), op, val);
}
