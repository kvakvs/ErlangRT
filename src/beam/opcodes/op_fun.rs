//! Module implements opcodes related to function objects/lambdas.

//use std::ptr;
use std::slice;

use beam::gen_op;
use beam::opcodes::assert_arity;
use beam::vm_loop::DispatchResult;
use emulator::gen_atoms;
use emulator::function::FunEntry;
use emulator::process::Process;
use emulator::runtime_ctx::Context;
use rt_defs::{ExceptionType, Arity};
use term::lterm::aspect_cp::CpAspect;
use term::lterm::aspect_smallint::SmallintAspect;
use term::raw::ho_closure::HOClosure;


#[inline]
pub fn opcode_make_fun2(ctx: &mut Context,
                        curr_p: &mut Process) -> DispatchResult {
  // Structure: make_fun2(lambda_index)
  assert_arity(gen_op::OPCODE_MAKE_FUN2, 1);

  let fe_box = ctx.fetch_term();
  let fe = fe_box.cp_get_ptr() as *const FunEntry;
  //panic!("boom");
  let hp = &mut curr_p.heap;
  let closure = unsafe {
    let nfree = (*fe).nfree as usize;
    let p = HOClosure::place_into(hp,
                                  fe.as_ref().unwrap(),
                                  &ctx.regs[0..nfree]);
    p.unwrap()
  };
  ctx.regs[0] = closure;

  DispatchResult::Normal
}


#[inline]
pub fn opcode_call_fun(ctx: &mut Context,
                       curr_p: &mut Process) -> DispatchResult {
  // Structure: call_fun(arity)
  // Expects: x[0..arity-1] = args. x[arity] = fun object
  assert_arity(gen_op::OPCODE_CALL_FUN, 1);

  let arity = ctx.fetch_term().small_get_u();
  let args = unsafe { slice::from_raw_parts(&ctx.regs[0], arity) };

  if let Some(closure) = unsafe { HOClosure::from_term(ctx.regs[arity]) } {
    ctx.live = arity + 1;

    if unsafe { (*closure).mfa.arity } != arity as Arity {
      return DispatchResult::Error(ExceptionType::Error, gen_atoms::BADARITY)
    }

    panic!("callfun!")
  } else {
    return DispatchResult::Error(ExceptionType::Error, gen_atoms::BADFUN)
  }

  DispatchResult::Normal
}
