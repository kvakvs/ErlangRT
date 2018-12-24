//! Module implements opcodes related to function objects/lambdas.

use crate::{
  beam::disp_result::DispatchResult,
  emulator::{
    function::FunEntry,
    process::Process,
    runtime_ctx::{self, Context},
    vm::VM,
  },
  fail::{self, RtResult},
  term::boxed,
};
use core::slice;

/// Structure: make_fun2(lambda_index:uint)
/// on load the argument is rewritten with a pointer to the funentry
pub struct OpcodeMakeFun2 {}

impl OpcodeMakeFun2 {
  pub const ARITY: usize = 1;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let fe_box = ctx.fetch_term();
    let fe = fe_box.get_cp_ptr::<FunEntry>();

    // panic!("boom");
    let hp = &mut curr_p.heap;
    let closure = unsafe {
      let nfree = (*fe).nfree as usize;
      let frozen = ctx.registers_slice(nfree);
      boxed::Closure::create_into(hp, fe.as_ref().unwrap(), frozen)?
    };
    ctx.set_x(0, closure);

    Ok(DispatchResult::Normal)
  }
}

/// Structure: call_fun(arity:uint)
/// Expects: x[0..arity-1] = args. x[arity] = fun object
pub struct OpcodeCallFun {}

impl OpcodeCallFun {
  pub const ARITY: usize = 1;

  #[inline]
  pub fn run(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let arity = ctx.fetch_term().get_small_unsigned();
    let args = unsafe { slice::from_raw_parts(&ctx.get_x(0), arity) };

    // Take function object argument
    let fobj = ctx.get_x(arity);

    // need mutable closure to possibly update dst in it later, during `apply`
    if let Ok(closure) = unsafe { boxed::Closure::mut_from_term(fobj) } {
      // `fobj` is a callable closure made with `fun() -> code end`
      runtime_ctx::call_closure::apply(vm, ctx, curr_p, closure, args)
    } else if let Ok(export) = unsafe { boxed::Export::mut_from_term(fobj) } {
      // `fobj` is an export made with `fun module:name/0`
      runtime_ctx::call_export::apply(vm, ctx, curr_p, export, args, true)
    } else {
      fail::create::badfun()
    }
  }
}
