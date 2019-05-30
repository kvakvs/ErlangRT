use super::RuntimeContext;
use crate::{
  beam::disp_result::DispatchResult,
  defs::Arity,
  emulator::{
    process::Process,
    runtime_ctx::call_native_fun::{self, CallBifTarget},
    vm::VM,
  },
  fail::{self, RtResult},
  term::{boxed, *},
};

fn module() -> &'static str {
  "runtime_ctx.call_export: "
}

/// The `exp` is an export made with `fun module:name/0` which can point to
/// either an Erlang function or to a BIF (native built-in function).
pub fn apply(
  vm: &mut VM,
  ctx: &mut RuntimeContext,
  curr_p: &mut Process,
  export: *const boxed::Export,
  args: &[Term],
  save_cp: bool,
) -> RtResult<DispatchResult> {
  // The `fobj` is a callable closure made with `fun() -> code end`
  let arity = args.len();
  ctx.live = arity + 1;

  let mfa = unsafe { (*export).exp.mfa };
  if mfa.arity != arity as Arity {
    println!(
      "{}badarity target_arity={} expected_arity={}",
      module(),
      mfa.arity,
      arity
    );
    return fail::create::badarity();
  }

  if vm.code_server.native_functions.mfa_exists(&mfa) {
    return call_native_fun::find_and_call_native_fun(
      vm,
      ctx,
      curr_p,
      Term::nil(),
      CallBifTarget::MFArity(mfa),
      args,
      Term::make_register_x(0),
      false,
    );
  } else {
    let code_server = vm.get_code_server_p();
    match unsafe { (*code_server).lookup_beam_code_and_load(&mfa) } {
      Ok(ip) => {
        if save_cp {
          ctx.cp = ctx.ip
        }
        ctx.ip = ip
      }
      Err(_e) => return fail::create::undef(),
    }
  }

  Ok(DispatchResult::Normal)
}
