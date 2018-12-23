use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
};

/// Sends to x0 value x1, x1 is moved to x0 as result of the operation.
/// If process with pid x0 does not exist, no error is raised.
/// Structure: send()
pub struct OpcodeSend {}

impl OpcodeSend {
  pub const ARITY: usize = 0;

  #[inline]
  pub fn run(
    vm: &VM,
    ctx: &mut Context,
    _curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let sched = vm.get_scheduler_p();
    let x1 = ctx.get_x(1);
    if let Some(p) = unsafe { (*sched).lookup_pid_mut(ctx.get_x(0)) } {
      p.deliver_message(x1);
    }

    ctx.set_x(0, x1);
    Ok(DispatchResult::Normal)
  }
}
