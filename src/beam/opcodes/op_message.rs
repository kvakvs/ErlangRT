use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::lterm::*,
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
    if let Some(p) = unsafe {
      let x0 = ctx.get_x(0);
      if !x0.is_pid() {
        return DispatchResult::badarg();
      }
      (*sched).lookup_pid_mut(x0) } {
      p.deliver_message(x1);
    }

    ctx.set_x(0, x1);
    Ok(DispatchResult::Normal)
  }
}

/// Picks up next message in the message queue and places it into `x0`.
/// If there is no next message, jumps to `fail` label which points to a `wait`
/// or `wait_timeout` instruction.
/// Structure: loop_rec(fail:cp, _source)
pub struct OpcodeLoopRec {}

impl OpcodeLoopRec {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context) -> (LTerm, LTerm) {
    let fail = ctx.fetch_term();
    let source = ctx.fetch_term();
    (fail, source)
  }

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail, _source) = Self::fetch_args(ctx);
    if let Some(msg) = curr_p.recv_current_message() {
      ctx.set_x(0, msg);
    } else {
      ctx.jump(fail);
    }
    Ok(DispatchResult::Normal)
  }
}

/// Advances message receive pointer to the next message then jumps to label
/// which points to a `loop_rec` instruction.
/// Structure: loop_rec_end(label:cp)
pub struct OpcodeLoopRecEnd {}

impl OpcodeLoopRecEnd {
  pub const ARITY: usize = 1;

  #[inline]
  fn fetch_args(ctx: &mut Context) -> LTerm {
    let label = ctx.fetch_term();
    label
  }

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let label = Self::fetch_args(ctx);
    curr_p.recv_step_over();
    ctx.jump(label);
    Ok(DispatchResult::Normal)
  }
}

/// Removes the current message in the process message list and moves it to `x0`
/// Structure: remove_message()
pub struct OpcodeRemoveMessage {}

impl OpcodeRemoveMessage {
  pub const ARITY: usize = 0;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let message = curr_p.recv_remove_message();
    ctx.set_x(0, message);
    Ok(DispatchResult::Normal)
  }
}
