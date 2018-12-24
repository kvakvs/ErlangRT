use super::Context;
use crate::{
  beam::disp_result::DispatchResult,
  defs::Arity,
  emulator::{process::Process, vm::VM},
  fail::{self, RtResult},
  term::{boxed, lterm::*},
};
use core::ptr;

fn module() -> &'static str {
  "runtime_ctx.call_closure: "
}

/// The `closure` is a callable closure with some frozen variables made with
/// `fun() -> code end`.
pub fn apply(
  vm: &mut VM,
  ctx: &mut Context,
  _curr_p: &mut Process,
  closure: *mut boxed::Closure,
  args: &[LTerm],
) -> RtResult<DispatchResult> {
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
    println!(
      "{}badarity full_arity={} call_arity={}",
      module(),
      full_arity,
      in_arity
    );
    return fail::create::badarity();
  }

  ctx.cp = ctx.ip;
  let dst = unsafe { (*closure).dst.clone() };

  // For dst, extract the code pointer, or update it
  // TODO: Verify code pointer module version, and possibly invalidate it.
  // OR TODO: subscribe from all exports to the module and get invalidation notifications
  ctx.ip = match dst {
    Some(p) => p.ptr,
    None => unsafe {
      let cs = vm.get_code_server_p();
      (*closure).update_location(&mut (*cs))?
    },
  };
  Ok(DispatchResult::Normal)
}
