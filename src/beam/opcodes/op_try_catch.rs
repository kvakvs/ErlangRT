use crate::{
  beam::disp_result::DispatchResult,
  defs::exc_type::ExceptionType,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::lterm::LTerm,
};
use crate::fail::Error;

/// Set up a try-catch stack frame for possible stack unwinding. Label points
/// at a `try_case` opcode where the error will be investigated.
/// We just write the cp given to the given Y register as a catch-value.
/// Structure: try(reg:regy, label:cp)
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

/// End try-catch by clearing the catch value on stack
/// Structure: try_end(reg:regy)
pub struct OpcodeTryEnd {}

impl OpcodeTryEnd {
  pub const ARITY: usize = 1;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let reg = ctx.fetch_term();
    debug_assert!(reg.is_regy());

    curr_p.num_catches -= 1;

    let hp = &mut curr_p.heap;
    hp.set_y(reg.get_special_value(), LTerm::nil())?;

    Ok(DispatchResult::Normal)
  }
}

/// Concludes the catch, removes catch value from stack and shifts registers
/// contents to prepare for exception checking.
/// Structure: try_case(reg:regy)
pub struct OpcodeTryCase {}

impl OpcodeTryCase {
  pub const ARITY: usize = 1;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let reg = ctx.fetch_term();
    debug_assert!(reg.is_regy());

    curr_p.num_catches -= 1;

    let hp = &mut curr_p.heap;
    hp.set_y(reg.get_special_value(), LTerm::nil())?;

    // Clear error and shift regs x1-x2-x3 to x0-x1-x2
    curr_p.clear_exception();
    ctx.set_x(0, ctx.get_x(1));
    ctx.set_x(1, ctx.get_x(2));
    ctx.set_x(2, ctx.get_x(3));

    Ok(DispatchResult::Normal)
  }
}

/// Raises the exception. The instruction is encumbered by backward
/// compatibility. Arg0 is a stack trace and Arg1 is the value accompanying
/// the exception. The reason of the raised exception is dug up from the stack
/// trace. Fixed by `raw_raise` in otp21.
/// Structure: raise(stacktrace:term, exc_value:term)
pub struct OpcodeRaise {}

impl OpcodeRaise {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (LTerm, LTerm) {
    let hp = &mut curr_p.heap;
    let stack_trace = ctx.fetch_and_load(hp);
    let exc_value = ctx.fetch_and_load(hp);
    (stack_trace, exc_value)
  }

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (raise_trace, raise_val) = Self::fetch_args(ctx, curr_p);

    let exc_type = match get_trace_from_exc(raise_trace) {
      None => ExceptionType::Error,
      Some(et) => et,
    };

    // curr_p.set_exception(exc_type, raise_val);
    // curr_p.set_stacktrace(raise_trace);

    Err(Error::Exception(exc_type, raise_val))
  }
}

/// In BEAM this extracts pointer to StackTrace struct stored inside bignum on
/// heap. Here for now we just assume it is always error.
fn get_trace_from_exc(trace: LTerm) -> Option<ExceptionType> {
  if trace == LTerm::nil() {
    return None;
  }
  return Some(ExceptionType::Error);
}
