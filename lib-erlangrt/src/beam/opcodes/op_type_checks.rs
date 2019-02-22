use crate::{
  beam::disp_result::DispatchResult,
  emulator::{process::Process, runtime_ctx::Context, vm::VM},
  fail::{self, RtResult},
  term::{boxed, lterm::LTerm},
};
use num::Signed;

/// Checks that argument is an atom, otherwise jumps to label.
/// Structure: is_atom(on_false:label, val:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsAtom, arity: 2,
  run: {
    if !value.is_atom() { ctx.jump(fail) }
    Ok(DispatchResult::Normal)
  },
  args: cp_not_nil(fail), load(value)
);

/// Checks that argument is a function or closure otherwise jumps to label.
/// Structure: is_function(on_false:label, arg:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsFunction, arity: 2,
  run: {
    if !value.is_fun() { ctx.jump(fail) }
    Ok(DispatchResult::Normal)
  },
  args: cp_not_nil(fail), load(value)
);

/// Checks that argument is a function or closure otherwise jumps to label.
/// Structure: is_function2(on_false:label, value:src, arity:smalluint)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsFunction2, arity: 3,
  run: { Self::is_function2(ctx, fail, value, arity) },
  args: cp_not_nil(fail), load(value), term(arity)
);

impl OpcodeIsFunction2 {
  #[inline]
  fn fetch_arity(
    arity_t: LTerm,
  ) -> RtResult<usize> {
    if arity_t.is_small() {
      Ok(arity_t.get_small_unsigned())
    } else {
      unsafe {
        let big_p = boxed::Bignum::const_from_term(arity_t)?;
        // negative arity bignum is not ok
        if (*big_p).value.is_negative() {
          return fail::create::badarg();
        }
        // positive arity bignum is ok but can't possibly match
        Ok(!0usize)
      }
    }
  }

  #[inline]
  pub fn is_function2(
    ctx: &mut Context,
    fail_label: LTerm,
    val: LTerm,
    arity_as_term: LTerm,
  ) -> RtResult<DispatchResult> {
    let arity = Self::fetch_arity(arity_as_term)?;
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
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsInteger, arity: 2,
  run: {
    if !value.is_small() && !value.is_big_int() { ctx.jump(fail) }
    Ok(DispatchResult::Normal)
  },
  args: cp_not_nil(fail), load(value)
);

/// Checks that argument is a boxed tuple or an empty tuple.
/// Structure: is_tuple(on_false:label, val:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsTuple, arity: 2,
  run: {
    if value != LTerm::empty_tuple() && !value.is_tuple() { ctx.jump(fail) }
    Ok(DispatchResult::Normal)
  },
  args: cp_not_nil(fail), load(value)
);

/// Checks that argument is a boxed binary or an empty binary.
/// Structure: is_binary(on_false:label, val:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsBinary, arity: 2,
  run: {
    if value != LTerm::empty_binary() && !value.is_binary() { ctx.jump(fail) }
    Ok(DispatchResult::Normal)
  },
  args: cp_not_nil(fail), load(value)
);

/// Checks that argument is a boxed containing a floating point number.
/// Structure: is_float(on_false:label, val:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsFloat, arity: 2,
  run: {
    if !value.is_float() { ctx.jump(fail) }
    Ok(DispatchResult::Normal)
  },
  args: cp_not_nil(fail), load(value)
);

/// Checks that argument is either a smallint, a bigint or a float.
/// Structure: is_number(on_false:label, val:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsNumber, arity: 2,
  run: {
    if !value.is_number() { ctx.jump(fail) }
    Ok(DispatchResult::Normal)
  },
  args: cp_not_nil(fail), load(value)
);

/// Checks that argument is local or remote pid.
/// Structure: is_pid(on_false:label, val:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsPid, arity: 2,
  run: {
    if !value.is_pid() { ctx.jump(fail) }
    Ok(DispatchResult::Normal)
  },
  args: cp_not_nil(fail), load(value)
);


/// Checks that argument is either a local or remote reference.
/// Structure: is_reference(on_false:label, val:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsReference, arity: 2,
  run: {
    if !value.is_ref() { ctx.jump(fail) }
    Ok(DispatchResult::Normal)
  },
  args: cp_not_nil(fail), load(value)
);

/// Checks that argument is either a local or remote port.
/// Structure: is_port(on_false:label, val:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsPort, arity: 2,
  run: {
    if !value.is_port() { ctx.jump(fail) }
    Ok(DispatchResult::Normal)
  },
  args: cp_not_nil(fail), load(value)
);

/// Checks that argument is a NIL or a cons pointer, otherwise jumps to label.
/// Structure: is_list(on_false:label, val:src)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsList, arity: 2,
  run: {
    if !value.is_list() { ctx.jump(fail) }
    Ok(DispatchResult::Normal)
  },
  args: cp_not_nil(fail), load(value)
);

