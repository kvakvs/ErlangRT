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
use term::lterm::LTerm;
use term::lterm::aspect_cp::CpAspect;
use term::lterm::aspect_smallint::SmallintAspect;
use term::raw::ho_closure::HOClosure;
//use term::raw::ho_import::HOImport;
use term::raw::ho_export::HOExport;


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

  // Take function object argument
  let fobj = ctx.regs[arity];
  if let Ok(closure) = unsafe { HOClosure::from_term(fobj) } {
    // `fobj` is a callable closure made with `fun() -> code end`
    call_closure(ctx, curr_p, closure, args)
  } else if let Ok(export) = unsafe { HOExport::from_term(fobj) } {
    // `fobj` is an export made with `fun module:name/0`
    call_export(ctx, curr_p, export, args)
  } else {
    return DispatchResult::Error(ExceptionType::Error, gen_atoms::BADFUN)
  }
}


/// The `closure` is a callable closure with some frozen variables made with
/// `fun() -> code end`.
fn call_closure(ctx: &mut Context,
                _curr_p: &mut Process,
                closure: *const HOClosure,
                args: &[LTerm]) -> DispatchResult
{
  let arity = args.len();
  ctx.live = arity + 1;

  if unsafe { (*closure).mfa.arity } != arity as Arity {
    return DispatchResult::Error(ExceptionType::Error, gen_atoms::BADARITY)
  }

  panic!("call_closure")
//  DispatchResult::Normal
}


/// The `exp` is an export made with `fun module:name/0` which can point to
/// either an Erlang function or to a BIF (native built-in function).
fn call_export(ctx: &mut Context,
               _curr_p: &mut Process,
               export: *const HOExport,
               args: &[LTerm]) -> DispatchResult
{
  // The `fobj` is a callable closure made with `fun() -> code end`
  let arity = args.len();
  ctx.live = arity + 1;

  if unsafe { (*export).exp.mfa.arity } != arity as Arity {
    return DispatchResult::Error(ExceptionType::Error, gen_atoms::BADARITY)
  }

  panic!("call_export")
//  DispatchResult::Normal
}
