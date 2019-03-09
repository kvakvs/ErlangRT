use crate::{
  beam::disp_result::DispatchResult,
  defs::exc_type::ExceptionType,
  emulator::{process::Process, runtime_ctx::Context},
  fail::{RtErr, RtResult},
  term::lterm::LTerm,
};

/// Set up a try-catch stack frame for possible stack unwinding. Label points
/// at a `try_case` opcode where the error will be investigated.
/// We just write the cp given to the given Y register as a catch-value.
/// Structure: try(reg:regy, label:cp)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeTry, arity: 2,
  run: { Self::try_opcode(curr_p, yreg, catch_label) },
  args: yreg(yreg), cp_or_nil(catch_label),
);

impl OpcodeTry {
  #[inline]
  pub fn try_opcode(
    curr_p: &mut Process,
    yreg: LTerm,
    catch_label: LTerm,
  ) -> RtResult<DispatchResult> {
    curr_p.num_catches += 1;

    let hp = &mut curr_p.heap;

    // Write catch value into the given stack register
    let catch_val = LTerm::make_catch(catch_label.get_cp_ptr());
    hp.set_y(yreg.get_special_value(), catch_val)?;
    // curr_p.heap.print_stack();

    Ok(DispatchResult::Normal)
  }
}

/// End try-catch by clearing the catch value on stack
/// Structure: try_end(reg:regy)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeTryEnd, arity: 1,
  run: { Self::try_end(ctx, curr_p, y) },
  args: yreg(y),
);

impl OpcodeTryEnd {
  #[inline]
  pub fn try_end(
    ctx: &mut Context,
    curr_p: &mut Process,
    yreg: LTerm,
  ) -> RtResult<DispatchResult> {
    curr_p.num_catches -= 1;

    let hp = &mut curr_p.heap;
    hp.set_y(yreg.get_special_value(), LTerm::nil())?;

    // Not sure why this is happening here, copied from Erlang/OTP
    if ctx.get_x(0).is_non_value() {
      // Clear error and shift regs x1-x2-x3 to x0-x1-x2
      curr_p.clear_exception();
      ctx.set_x(0, ctx.get_x(1));
      ctx.set_x(1, ctx.get_x(2));
      ctx.set_x(2, ctx.get_x(3));
    }

    Ok(DispatchResult::Normal)
  }
}

/// Concludes the catch, removes catch value from stack and shifts registers
/// contents to prepare for exception checking.
/// Structure: try_case(reg:regy)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeTryCase, arity: 1,
  run: { Self::try_case(ctx, curr_p, y) },
  args: yreg(y),
);

impl OpcodeTryCase {
  #[inline]
  pub fn try_case(
    ctx: &mut Context,
    curr_p: &mut Process,
    yreg: LTerm,
  ) -> RtResult<DispatchResult> {
    curr_p.num_catches -= 1;

    let hp = &mut curr_p.heap;
    hp.set_y(yreg.get_special_value(), LTerm::nil())?;

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
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeRaise, arity: 2,
  run: { Self::raise(stacktrace, exc_value) },
  args: load(stacktrace), load(exc_value),
);

impl OpcodeRaise {
  #[inline]
  pub fn raise(raise_trace: LTerm, raise_val: LTerm) -> RtResult<DispatchResult> {
    let exc_type = match get_trace_from_exc(raise_trace) {
      None => ExceptionType::Error,
      Some(et) => et,
    };

    // curr_p.set_exception(exc_type, raise_val);
    // curr_p.set_stacktrace(raise_trace);

    Err(RtErr::Exception(exc_type, raise_val))
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
