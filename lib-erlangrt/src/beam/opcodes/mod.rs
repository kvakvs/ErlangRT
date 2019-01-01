//! Opcodes group of modules provides inline implementations of BEAM opcodes.
pub mod op_bif;
pub mod op_data;
pub mod op_execution;
pub mod op_fun;
pub mod op_list;
pub mod op_memory;
pub mod op_message;
pub mod op_predicates;
pub mod op_try_catch;
pub mod op_tuple;
pub mod op_type_checks;

pub use crate::beam::opcodes::{
  op_bif::*, op_data::*, op_execution::*, op_fun::*, op_list::*, op_memory::*,
  op_message::*, op_predicates::*, op_try_catch::*, op_tuple::*, op_type_checks::*,
};
use crate::{
  beam::gen_op,
  defs::Word,
  emulator::{code::opcode::RawOpcode, runtime_ctx::Context},
};

/// Debug-time assertion to guard against incompatible opcode arity on BEAM
/// version changes.
#[inline]
pub fn assert_arity(op: RawOpcode, code_expected_arity: Word) {
  debug_assert!(
    op <= gen_op::OPCODE_MAX,
    "Opcode {:?} is too large, max {:?}",
    op,
    gen_op::OPCODE_MAX
  );
  let genop_arity = gen_op::ARITY_MAP[op.get() as usize] as Word;
  debug_assert_eq!(
    genop_arity,
    code_expected_arity,
    "Opcode {}={} code expects arity {}, while genop table has {}",
    gen_op::opcode_name(op),
    op.get(),
    code_expected_arity,
    genop_arity
  );
}

/// Display an error about opcode not supported/not implemented.
pub fn unknown_opcode(op: RawOpcode, ctx: &Context) {
  println!("{}", ctx);
  panic!(
    "vm_dispatch: Opcode {:?} '{}' not implemented",
    op.get(),
    gen_op::opcode_name(op)
  )
}
