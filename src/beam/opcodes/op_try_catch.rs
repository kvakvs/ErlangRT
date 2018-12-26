use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::lterm::LTerm,
};

/// Set up a try-catch stack frame for possible stack unwinding. Label points
/// at `try_case` opcode where the error will be investigated.
/// We just write the cp given to the given Y register.
/// Structure: try(reg, label:cp)
pub struct OpcodeTry {}

impl OpcodeTry {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context) -> (LTerm, LTerm) {
    let reg = ctx.fetch_term();
    let catch_label = ctx.fetch_term();
    (reg, catch_label)
  }

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (reg, catch_label) = Self::fetch_args(ctx);
    debug_assert!(reg.is_regy());

    curr_p.num_catches += 1;
    let hp = &mut curr_p.heap;

    // Write catch value into the given stack register
    let catch_val = LTerm::make_catch(catch_label.get_cp_ptr());
    hp.set_y(reg.get_special_value(), catch_val)?;
    // curr_p.heap.print_stack();

    Ok(DispatchResult::Normal)
  }
}
