use super::Context;

use beam::disp_result::{DispatchResult};
use emulator::process::{Process};
use rt_defs::{Arity};
use term::lterm::{LTerm};
use term::raw::ho_closure::{HOClosure};


fn module() -> &'static str { "runtime_ctx.call_closure: " }


/// The `closure` is a callable closure with some frozen variables made with
/// `fun() -> code end`.
pub fn apply(ctx: &mut Context,
             _curr_p: &mut Process,
             closure: *const HOClosure,
             args: &[LTerm]) -> DispatchResult
{
  let arity = args.len();
  ctx.live = arity + 1;

  // Actual call is performed for passed args + frozen args, so add them
  let target_arity = unsafe { (*closure).mfa.arity + (*closure).nfree };

  if target_arity != arity as Arity {
    println!("{}badarity target_arity={} call_arity={}", module(), target_arity, arity);
    return DispatchResult::badarity();
  }

  panic!("call_closure")
//  DispatchResult::Normal
}
