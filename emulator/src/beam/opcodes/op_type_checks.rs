use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::{self, RtResult},
  term::{boxed, lterm::LTerm},
};
use num::Signed;

/// Same function for all type check opcodes
#[inline]
fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (LTerm, LTerm) {
  let fail = ctx.fetch_term();
  let value = ctx.fetch_and_load(&mut curr_p.heap);
  (fail, value)
}

/// Checks that argument is an atom, otherwise jumps to label.
/// Structure: is_atom(on_false:label, val:src)
pub struct OpcodeIsAtom {}

impl OpcodeIsAtom {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = fetch_args(ctx, curr_p);
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
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = fetch_args(ctx, curr_p);
    if !val.is_fun() {
      ctx.jump(fail_label)
    }
    Ok(DispatchResult::Normal)
  }
}

/// Checks that argument is a function or closure otherwise jumps to label.
/// Structure: is_function2(on_false:label, arg:src, arity:smalluint)
pub struct OpcodeIsFunction2 {}

impl OpcodeIsFunction2 {
  pub const ARITY: usize = 3;

  #[inline]
  fn fetch_args(
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<(LTerm, LTerm, usize)> {
    let hp = &mut curr_p.heap;
    let fail = ctx.fetch_term();
    let value = ctx.fetch_and_load(hp);

    let arity_t = ctx.fetch_and_load(hp);
    let arity = if arity_t.is_small() {
      arity_t.get_small_unsigned()
    } else {
      unsafe {
        let big_p = boxed::Bignum::const_from_term(arity_t)?;
        // negative arity bignum is not ok
        if (*big_p).value.is_negative() {
          return fail::create::badarg();
        }
        // positive arity bignum is ok but can't possibly match
        !0usize
      }
    };

    Ok((fail, value, arity))
  }

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val, arity) = Self::fetch_args(ctx, curr_p)?;
    println!("is_function2? {}", val);
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
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = fetch_args(ctx, curr_p);
    if !val.is_small() && !val.is_big_int() {
      ctx.jump(fail_label)
    }
    Ok(DispatchResult::Normal)
  }
}


/// Checks that argument is a boxed tuple or an empty tuple.
/// Structure: is_tuple(on_false:label, val:src)
pub struct OpcodeIsTuple {}

impl OpcodeIsTuple {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = fetch_args(ctx, curr_p);
    if val != LTerm::empty_tuple() && !val.is_tuple() {
      ctx.jump(fail_label)
    }
    Ok(DispatchResult::Normal)
  }
}


/// Checks that argument is a boxed binary or an empty binary.
/// Structure: is_binary(on_false:label, val:src)
pub struct OpcodeIsBinary {}

impl OpcodeIsBinary {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = fetch_args(ctx, curr_p);
    if val != LTerm::empty_binary() && !val.is_binary() {
      ctx.jump(fail_label)
    }
    Ok(DispatchResult::Normal)
  }
}


/// Checks that argument is a boxed containing a floating point number.
/// Structure: is_float(on_false:label, val:src)
pub struct OpcodeIsFloat {}

impl OpcodeIsFloat {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = fetch_args(ctx, curr_p);
    if !val.is_float() {
      ctx.jump(fail_label)
    }
    Ok(DispatchResult::Normal)
  }
}


/// Checks that argument is either a smallint, a bigint or a float.
/// Structure: is_number(on_false:label, val:src)
pub struct OpcodeIsNumber {}

impl OpcodeIsNumber {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = fetch_args(ctx, curr_p);
    if !val.is_number() {
      ctx.jump(fail_label)
    }
    Ok(DispatchResult::Normal)
  }
}


/// Checks that argument is local or remote pid.
/// Structure: is_pid(on_false:label, val:src)
pub struct OpcodeIsPid {}

impl OpcodeIsPid {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = fetch_args(ctx, curr_p);
    if !val.is_pid() {
      ctx.jump(fail_label)
    }
    Ok(DispatchResult::Normal)
  }
}


/// Checks that argument is either a local or remote reference.
/// Structure: is_reference(on_false:label, val:src)
pub struct OpcodeIsReference {}

impl OpcodeIsReference {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = fetch_args(ctx, curr_p);
    if !val.is_ref() {
      ctx.jump(fail_label)
    }
    Ok(DispatchResult::Normal)
  }
}


/// Checks that argument is either a local or remote port.
/// Structure: is_port(on_false:label, val:src)
pub struct OpcodeIsPort {}

impl OpcodeIsPort {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = fetch_args(ctx, curr_p);
    if !val.is_port() {
      ctx.jump(fail_label)
    }
    Ok(DispatchResult::Normal)
  }
}

/// Checks that argument is a NIL or a cons pointer, otherwise jumps to label.
/// Structure: is_list(on_false:label, val:src)
pub struct OpcodeIsList {}

impl OpcodeIsList {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val) = fetch_args(ctx, curr_p);
    if !val.is_list() {
      ctx.jump(fail_label)
    }
    Ok(DispatchResult::Normal)
  }
}
