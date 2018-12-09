//! Module implements opcodes related to function objects/lambdas.

use crate::{
  beam::{disp_result::DispatchResult, gen_op, opcodes::assert_arity},
  emulator::{
    function::FunEntry,
    process::Process,
    runtime_ctx::{self, Context},
    vm::VM,
  },
  fail::RtResult,
  term::boxed,
};

use std::slice;


#[inline]
pub fn opcode_make_fun2(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: make_fun2(lambda_index:uint)
  // on load the argument is rewritten with a pointer to the funentry
  assert_arity(gen_op::OPCODE_MAKE_FUN2, 1);

  let fe_box = ctx.fetch_term();
  let fe = fe_box.get_cp_ptr::<FunEntry>();

  //panic!("boom");
  let hp = &mut curr_p.heap;
  let closure= unsafe {
    let nfree = (*fe).nfree as usize;
    let frozen = ctx.registers_slice(nfree);
    boxed::Closure::create_into(hp, fe.as_ref().unwrap(), frozen)?
  };
  ctx.set_x(0, closure);

  Ok(DispatchResult::Normal)
}


#[inline]
pub fn opcode_call_fun(
  vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: call_fun(arity:uint)
  // Expects: x[0..arity-1] = args. x[arity] = fun object
  assert_arity(gen_op::OPCODE_CALL_FUN, 1);

  let arity = ctx.fetch_term().get_small_unsigned();
  let args = unsafe { slice::from_raw_parts(&ctx.x(0), arity) };

  // Take function object argument
  let fobj = ctx.x(arity);
  if let Ok(closure) = unsafe { boxed::Closure::const_from_term(fobj) } {
    // `fobj` is a callable closure made with `fun() -> code end`
    runtime_ctx::call_closure::apply(vm, ctx, curr_p, closure, args)
  } else if let Ok(export) = unsafe { boxed::Export::const_from_term(fobj) } {
    // `fobj` is an export made with `fun module:name/0`
    runtime_ctx::call_export::apply(
      ctx,
      curr_p,
      export,
      args,
      true,
      vm.code_server.borrow_mut().as_mut(),
    )
  } else {
    DispatchResult::badfun()
  }
}
