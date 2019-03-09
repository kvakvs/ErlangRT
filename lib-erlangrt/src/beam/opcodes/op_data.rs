//! Module implements opcodes related to reading, writing, and moving data.

use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, },
  fail::RtResult,
};

/// Load a value from `src` and store it into `dst`. Source can be any literal
/// term, a register or a stack cell. Destination can be any register or a
/// stack cell.
/// Structure: move(src:src, dst:dst)
// TODO: Optimize this by having specialized move instructions with packed arg
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeMove, arity: 2,
  run: {
    ctx.store_value(src, dst, &mut curr_p.heap)?;
    Ok(DispatchResult::Normal)
  },
  args: load(src), term(dst),
);
