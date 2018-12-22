use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::lterm::LTerm,
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


/// Checks that argument is a small integer or a boxed big integer,
/// otherwise jumps to fail label.
/// Structure: is_integer(on_false:label, val:src)
pub struct OpcodeIsInteger {}

impl OpcodeIsInteger {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (LTerm, LTerm) {
    let fail = ctx.fetch_term();
    let value = ctx.fetch_and_load(&mut curr_p.heap);
    (fail, value)
  }

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = Self::fetch_args(ctx, curr_p);

    if !val.is_small() && !val.is_big_int() {
      ctx.jump(fail_label)
    }

    Ok(DispatchResult::Normal)
  }
}
