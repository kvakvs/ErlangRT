use super::Context;
use std::ptr;

use beam::disp_result::{DispatchResult};
use emulator::function::CallableLocation;
use emulator::process::{Process};
use emulator::vm::VM;
use rt_defs::{Arity};
use term::lterm::*;
use term::raw::*;


fn module() -> &'static str { "runtime_ctx.call_closure: " }


/// The `closure` is a callable closure with some frozen variables made with
/// `fun() -> code end`.
pub fn apply(vm: &VM,
             ctx: &mut Context,
             _curr_p: &mut Process,
             closure: *const HOClosure,
             args: &[LTerm]) -> DispatchResult
{
  let in_arity = args.len();

  // Actual call is performed for passed args + frozen args, so add them
  let full_arity = unsafe { (*closure).mfa.arity as usize + (*closure).nfree };
  ctx.live = full_arity;

  // Copy extra args from after nfree field
  unsafe {
    let frozen_ptr = &(*closure).frozen as *const LTerm;
    let dst_ptr = ctx.regs.as_mut_ptr().add(in_arity);
    ptr::copy(frozen_ptr, dst_ptr, (*closure).nfree);
  }

  if full_arity != in_arity as Arity {
    println!("{}badarity full_arity={} call_arity={}", module(), full_arity, in_arity);
    return DispatchResult::badarity();
  }

  ctx.cp = ctx.ip;
  let dst = unsafe { (*closure).dst };
  ctx.ip = match dst {
    CallableLocation::Code(p) =>
      p.code_ptr(vm.code_server.borrow().as_ref()),
    CallableLocation::NeedUpdate =>
      panic!("Must not have this value here"),
  };
  DispatchResult::Normal
}
