use crate::{
  beam::{disp_result::DispatchResult},
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
};

/// Checks that argument is an atom, otherwise jumps to label.
/// Structure: is_atom(on_false:label, val:src)
pub struct OpcodeIsAtom {}

impl OpcodeIsAtom {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let hp = &curr_p.heap;
    let fail_label = ctx.fetch_term();
    let val = ctx.fetch_and_load(hp);

    if !val.is_atom() {
      ctx.jump(fail_label)
    }

    Ok(DispatchResult::Normal)
  }
}

/// Checks that argument is a function or closure otherwise jumps to label.
/// Structure: is_function(on_false:label, arg:src)
pub struct OpcodeIsFunction {}

impl OpcodeIsFunction {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let hp = &curr_p.heap;
    let fail_label = ctx.fetch_term();
    let val = ctx.fetch_and_load(hp);

    if !val.is_fun() {
      ctx.jump(fail_label)
    }

    Ok(DispatchResult::Normal)
  }
}

/// Checks that argument is a function or closure otherwise jumps to label.
/// Structure: is_function(on_false:label, arg:src)
pub struct OpcodeIsFunction2 {}

impl OpcodeIsFunction2 {
  pub const ARITY: usize = 3;

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let hp = &curr_p.heap;
    let fail_label = ctx.fetch_term();
    let val = ctx.fetch_and_load(hp);
    let arity = ctx.fetch_and_load(hp).get_small_unsigned();

    if !val.is_fun_of_arity(arity) {
      ctx.jump(fail_label)
    }

    Ok(DispatchResult::Normal)
  }
}
