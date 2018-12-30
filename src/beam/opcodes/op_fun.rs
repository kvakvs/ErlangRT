//! Module implements opcodes related to function objects/lambdas.
use crate::{
  beam::disp_result::DispatchResult,
  emulator::{
    function::FunEntry,
    gen_atoms,
    mfa::MFArity,
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

    let hp = &mut curr_p.heap;
    let closure = unsafe {
      let nfrozen = (*fe).nfrozen as usize;
      let frozen = ctx.registers_slice(0, nfrozen);
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
    let fun_object = ctx.get_x(arity);

    // need mutable closure to possibly update dst in it later, during `apply`
    if let Ok(closure) = unsafe { boxed::Closure::mut_from_term(fun_object) } {
      // `fun_object` is a callable closure made with `fun() -> code end`
      runtime_ctx::call_closure::apply(vm, ctx, curr_p, closure, args)
    } else if let Ok(export) = unsafe { boxed::Export::mut_from_term(fun_object) } {
      // `fun_object` is an export made with `fun module:name/0`
      runtime_ctx::call_export::apply(vm, ctx, curr_p, export, args, true)
    } else {
      fail::create::badfun()
    }
  }
}

/// Applies args in `x[0..arity]` to module specified by an atom or a tuple in
/// `x[arity]` and function specified by an atom in `x[arity+1]`.
/// Structure: apply(arity:uint)
/// Expects: `x[0..arity-1]` args, `x[arity]` = module, `x[arity+1]` = function
pub struct OpcodeApply {}

impl OpcodeApply {
  pub const ARITY: usize = 1;

  #[inline]
  pub fn run(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let arity = ctx.fetch_term().get_small_unsigned();
    let mfa = MFArity::new(ctx.get_x(arity), ctx.get_x(arity + 1), arity);

    ctx.live = arity + 2;

    fixed_apply(vm, ctx, curr_p, &mfa, 0)?;
    Ok(DispatchResult::Normal)
  }
}

/// Applies args in `x[0..arity]` to module specified by an atom or a tuple in
/// `x[arity]` and function specified by an atom in `x[arity+1]`. The call is
/// tail recursive, and `dealloc` words are removed from stack preserving the
/// CP on stack top.
/// Structure: apply_last(arity:smallint, dealloc:smallint)
/// Expects: `x[0..arity-1]` args, `x[arity]` = module, `x[arity+1]` = function
pub struct OpcodeApplyLast {}

impl OpcodeApplyLast {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context) -> (usize, usize) {
    let arity = ctx.fetch_term().get_small_unsigned();
    let dealloc = ctx.fetch_term().get_small_unsigned();
    (arity, dealloc)
  }

  #[inline]
  pub fn run(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (arity, dealloc) = Self::fetch_args(ctx);

    let module = ctx.get_x(arity);
    let function = ctx.get_x(arity + 1);
    if !function.is_atom() {
      return fail::create::badarg();
    }

    ctx.live = arity + 2;

    let mfa = MFArity::new(module, function, arity);
    fixed_apply(vm, ctx, curr_p, &mfa, dealloc)?;
    Ok(DispatchResult::Normal)
  }
}

/// Perform application of module:function/arity to args stored in registers,
/// with optional deallocation.
fn fixed_apply(
  vm: &mut VM,
  ctx: &mut Context,
  curr_p: &mut Process,
  mfa: &MFArity,
  dealloc: usize,
) -> RtResult<()> {
  if mfa.m == gen_atoms::ERLANG && mfa.f == gen_atoms::APPLY && mfa.arity == 3 {
    panic!("TODO special handling for apply on apply/3");
  }

  println!("call_mfa {}", mfa);
  let l_result = vm.code_server.lookup_mfa(mfa, true);
  if l_result.is_err() {
    return fail::create::undef();
  }

  let args = ctx.registers_slice(0, mfa.arity);
  ctx.call_mfa(vm, curr_p, &l_result.unwrap(), args, dealloc == 0)?;
  Ok(())
}
