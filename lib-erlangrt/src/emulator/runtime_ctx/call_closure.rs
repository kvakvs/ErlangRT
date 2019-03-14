use super::Context;
use crate::{
  beam::disp_result::DispatchResult,
  defs::Arity,
  emulator::{process::Process, vm::VM},
  fail::{self, RtResult},
  term::{boxed, lterm::*},
};

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
  args: &[Term],
) -> RtResult<DispatchResult> {
  let args_len = args.len();

  let (closure_arity, closure_nfrozen) =
    unsafe { ((*closure).mfa.arity as usize, (*closure).nfrozen) };
  // The call is performed for passed args + frozen args, so the actual arity
  // will be incoming args length + frozen length
  let actual_call_arity = closure_nfrozen + args_len;

  unsafe {
    let frozen = (*closure).get_frozen();
    // copy frozen values into registers after the arity
    ctx
      .registers_slice_mut(args_len, closure_nfrozen)
      .copy_from_slice(frozen);
  }

  ctx.live = actual_call_arity;
  println!("{}", ctx);

  if actual_call_arity != closure_arity as Arity {
    println!(
      "{}badarity call_arity={} nfrozen={} args_len={}",
      module(),
      closure_arity,
      closure_nfrozen,
      args_len
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
