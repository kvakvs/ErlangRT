//! Module implements opcodes related to function objects/lambdas.

use beam::disp_result::DispatchResult;
use beam::gen_op;
use beam::opcodes::assert_arity;
use term::boxed;
use emulator::function::FunEntry;
use emulator::process::Process;
use emulator::runtime_ctx;
use emulator::runtime_ctx::Context;
use emulator::vm::VM;
use std::slice;
use term::lterm::*;
use term::raw::*;


#[inline]
pub fn opcode_make_fun2(_vm: &VM, ctx: &mut Context,
                        curr_p: &mut Process) -> DispatchResult {
  // Structure: make_fun2(lambda_index:uint)
  // on load the argument is rewritten with a pointer to the funentry
  assert_arity(gen_op::OPCODE_MAKE_FUN2, 1);

  let fe_box = ctx.fetch_term();
  let fe = fe_box.cp_get_ptr() as *const FunEntry;

  //panic!("boom");
  let hp = &mut curr_p.heap;
  let closure = unsafe {
    let nfree = (*fe).nfree as usize;
    let p = boxed::Closure::place_into(hp,
                                       fe.as_ref().unwrap(),
                                       &ctx.regs[0..nfree]);
    p.unwrap()
  };
  ctx.regs[0] = closure;

  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_fun(vm: &VM, ctx: &mut Context,
                       curr_p: &mut Process) -> DispatchResult {
  // Structure: call_fun(arity:uint)
  // Expects: x[0..arity-1] = args. x[arity] = fun object
  assert_arity(gen_op::OPCODE_CALL_FUN, 1);

  let arity = ctx.fetch_term().small_get_u();
  let args = unsafe { slice::from_raw_parts(&ctx.regs[0], arity) };

  // Take function object argument
  let fobj = ctx.regs[arity];
  if let Ok(closure) = unsafe { boxed::Closure::from_term(fobj) } {
    // `fobj` is a callable closure made with `fun() -> code end`
    runtime_ctx::call_closure::apply(vm, ctx, curr_p, closure, args)
  } else if let Ok(export) = unsafe { boxed::Export::from_term(fobj) } {
    // `fobj` is an export made with `fun module:name/0`
    runtime_ctx::call_export::apply(
      ctx, curr_p, export, args, true,
      vm.code_server.borrow_mut().as_mut()
    )
  } else {
    return DispatchResult::badfun()
  }
}
