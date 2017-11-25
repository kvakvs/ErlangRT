//! Opcodes group of modules provides inline implementations of BEAM opcodes.
pub mod op_bif;
pub mod op_data;
pub mod op_execution;
pub mod op_fun;
pub mod op_list;
pub mod op_memory;
pub mod op_predicates;

pub use beam::opcodes::op_bif::*;
pub use beam::opcodes::op_data::*;
pub use beam::opcodes::op_execution::*;
pub use beam::opcodes::op_fun::*;
pub use beam::opcodes::op_list::*;
pub use beam::opcodes::op_memory::*;
pub use beam::opcodes::op_predicates::*;


use beam::gen_op;
use rt_defs::Word;
use emulator::code::opcode::RawOpcode;
use emulator::runtime_ctx::Context;


/// Run a check whether opcode is not too large (within the supported range).
// TODO: Maybe #[inline] but now let compiler decide
#[cfg(debug_assertions)]
pub fn assert_arity(op: RawOpcode, val: Word) {
  debug_assert!(op < gen_op::OPCODE_MAX, "Opcode is too large");
  debug_assert_eq!(gen_op::ARITY_MAP[op as usize] as Word, val,
                   "Opcode {}={} arity is expected to be {}",
                   gen_op::opcode_name(op as u8), op, val);
}


#[cfg(not(debug_assertions))]
#[inline]
pub fn assert_arity(_op: RawOpcode, _val: Word) {}


/// Display an error about opcode not supported/not implemented.
pub fn unknown_opcode(op: RawOpcode, ctx: &Context) {
  println!("{}", ctx);
  panic!("vm_dispatch: Opcode {:?} '{}' not implemented",
         op, gen_op::opcode_name(op))
}
