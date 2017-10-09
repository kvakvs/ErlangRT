pub mod execution_control;
pub mod memory;

pub use beam::opcodes::execution_control::*;
pub use beam::opcodes::memory::*;

use beam::gen_op;
use defs::Word;


// TODO: Maybe #[inline] but now let compiler decide
pub fn assert_arity(op: Word, val: Word) {
  assert!(op < gen_op::OPCODE_MAX, "Opcode is too large");
  assert_eq!(gen_op::ARITY_MAP[op as usize] as Word, val,
             "Opcode {}={} arity is expected to be {}",
             gen_op::opcode_name(op as u8), op, val);
}
