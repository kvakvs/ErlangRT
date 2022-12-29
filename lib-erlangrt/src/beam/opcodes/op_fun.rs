//! Module implements opcodes related to function objects/lambdas.
use crate::{
  beam::disp_result::DispatchResult,
  emulator::{
    function::FunEntry,
    gen_atoms,
    heap::THeapOwner,
    mfa::ModFunArity,
    process::Process,
    runtime_ctx::{self, RuntimeContext},
    vm::VM,
  },
  fail::{self, RtResult},
  term::{boxed, Term},
};

// Create a closure from a lambda table item (loaded from a BEAM file).
// Structure: make_fun2(lambda_index:uint)
// on load the argument is rewritten with a CP pointer to the funentry
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeMakeFun2, arity: 1,
  run: { Self::make_fun2(ctx, curr_p, export) },
  args: term(export),
);

impl OpcodeMakeFun2 {
  #[inline]
  pub fn make_fun2(
    ctx: &mut RuntimeContext,
    curr_p: &mut Process,
    export: Term,
  ) -> RtResult<DispatchResult> {
    let fun_entry = export.get_cp_ptr::<FunEntry>();
    let hp = curr_p.get_heap_mut();
    let closure = unsafe {
      let nfrozen = (*fun_entry).nfrozen;
      let frozen = ctx.registers_slice(0, nfrozen);
      boxed::Closure::create_into(hp, fun_entry.as_ref().unwrap(), frozen)?
    };
    ctx.set_x(0, closure);
    Ok(DispatchResult::Normal)
  }
}

// Structure: call_fun(arity:uint)
// Expects: x[0..arity-1] = args. x[arity] = fun object
define_opcode!(vm, ctx, curr_p,
  name: OpcodeCallFun, arity: 1,
  run: { Self::call_fun(vm, ctx, curr_p, arity) },
  args: usize(arity),
);

impl OpcodeCallFun {
  #[inline]
  pub fn call_fun(
    vm: &mut VM,
    ctx: &mut RuntimeContext,
    curr_p: &mut Process,
    arity: usize,
  ) -> RtResult<DispatchResult> {
    let args = ctx.registers_slice(0, arity);

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

// Applies args in `x[0..arity]` to module specified by an atom or a tuple in
// `x[arity]` and function specified by an atom in `x[arity+1]`.
// Structure: apply(arity:uint)
// Expects: `x[0..arity-1]` args, `x[arity]` = module, `x[arity+1]` = function
define_opcode!(vm, ctx, curr_p,
  name: OpcodeApply, arity: 1,
  run: { Self::apply(vm, ctx, curr_p, arity) },
  args: usize(arity),
);

impl OpcodeApply {
  #[inline]
  pub fn apply(
    vm: &mut VM,
    ctx: &mut RuntimeContext,
    curr_p: &mut Process,
    arity: usize,
  ) -> RtResult<DispatchResult> {
    let mfa = ModFunArity::new(ctx.get_x(arity), ctx.get_x(arity + 1), arity);
    ctx.live = arity + 2;
    fixed_apply(vm, ctx, curr_p, &mfa, 0)?;
    Ok(DispatchResult::Normal)
  }
}

// Applies args in `x[0..arity]` to module specified by an atom or a tuple in
// `x[arity]` and function specified by an atom in `x[arity+1]`. The call is
// tail recursive, and `dealloc` words are removed from stack preserving the
// CP on stack top.
// Structure: apply_last(arity:smallint, dealloc:smallint)
// Expects: `x[0..arity-1]` args, `x[arity]` = module, `x[arity+1]` = function
define_opcode!(vm, ctx, curr_p,
  name: OpcodeApplyLast, arity: 2,
  run: { Self::apply_last(vm, ctx, curr_p, arity, dealloc) },
  args: usize(arity), usize(dealloc),
);

impl OpcodeApplyLast {
  #[inline]
  pub fn apply_last(
    vm: &mut VM,
    ctx: &mut RuntimeContext,
    curr_p: &mut Process,
    arity: usize,
    dealloc: usize,
  ) -> RtResult<DispatchResult> {
    let module = ctx.get_x(arity);
    let function = ctx.get_x(arity + 1);
    if !function.is_atom() {
      return fail::create::badarg();
    }

    ctx.live = arity + 2;

    let mfa = ModFunArity::new(module, function, arity);
    fixed_apply(vm, ctx, curr_p, &mfa, dealloc)?;
    Ok(DispatchResult::Normal)
  }
}

/// Perform application of module:function/arity to args stored in registers,
/// with optional deallocation.
fn fixed_apply(
  vm: &mut VM,
  ctx: &mut RuntimeContext,
  curr_p: &mut Process,
  mfa: &ModFunArity,
  dealloc: usize,
) -> RtResult<()> {
  if mfa.m == gen_atoms::ERLANG && mfa.f == gen_atoms::APPLY && mfa.arity == 3 {
    panic!("TODO special handling for apply on apply/3");
  }

  println!("call_mfa {mfa}");
  let l_result = vm.code_server.lookup_mfa(mfa, true);
  if l_result.is_err() {
    return fail::create::undef();
  }

  let args = ctx.registers_slice(0, mfa.arity);
  ctx.call_mfa(vm, curr_p, &l_result.unwrap(), args, dealloc == 0)?;
  Ok(())
}
