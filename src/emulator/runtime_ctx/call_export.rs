use super::Context;

use beam::disp_result::{DispatchResult};
use bif;
use emulator::code_srv;
use emulator::process::{Process};
use rt_defs::{Arity};
use term::lterm::*;
use term::raw::ho_export::{HOExport};
use emulator::runtime_ctx::call_bif;
use emulator::runtime_ctx::call_bif::{CallBifTarget};


fn module() -> &'static str { "runtime_ctx.call_export: " }


/// The `exp` is an export made with `fun module:name/0` which can point to
/// either an Erlang function or to a BIF (native built-in function).
pub fn apply(ctx: &mut Context,
             curr_p: &mut Process,
             export: *const HOExport,
             args: &[LTerm],
             save_cp: bool) -> DispatchResult
{
  // The `fobj` is a callable closure made with `fun() -> code end`
  let arity = args.len();
  ctx.live = arity + 1;

  let mfa = unsafe { (*export).exp.mfa };
  if mfa.arity != arity as Arity {
    println!("{}badarity target_arity={} expected_arity={}", module(), mfa.arity, arity);
    return DispatchResult::badarity()
  }

  if bif::is_bif(&mfa) {
    return call_bif::apply(ctx, curr_p, nil(),
                           CallBifTarget::MFArity(mfa),
                           args,
                           LTerm::make_xreg(0),
                           false)
  } else {
    match code_srv::lookup_and_load(&mfa) {
      Ok(ip) => {
        if save_cp { ctx.cp = ctx.ip }
        ctx.ip = ip
      },
      Err(_e) => return DispatchResult::undef()
    }
  }

  //panic!("call_export")
  DispatchResult::Normal
}
