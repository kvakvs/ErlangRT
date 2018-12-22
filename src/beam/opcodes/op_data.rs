//! Module implements opcodes related to reading, writing, and moving data.

use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::lterm::*,
};

/// Load a value from `src` and store it into `dst`. Source can be any literal
/// term, a register or a stack cell. Destination can be any register or a
/// stack cell.
/// Structure: move(src:src, dst:dst)
// TODO: Optimize this by having specialized move instructions with packed arg
pub struct OpcodeMove {}

impl OpcodeMove {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (LTerm, LTerm) {
    let src = ctx.fetch_and_load(&curr_p.heap);
    let dst = ctx.fetch_term();
    (src, dst)
  }

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (src, dst) = Self::fetch_args(ctx, curr_p);
    ctx.store_value(src, dst, &mut curr_p.heap)?;

    Ok(DispatchResult::Normal)
  }
}
