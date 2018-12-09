//! Module implements opcodes related to execution control: Calls, jumps,
//! returns etc.

use crate::{
  beam::{disp_result::DispatchResult, gen_op, opcodes::assert_arity},
  emulator::{
    code::CodePtr,
    process::Process,
    runtime_ctx::{call_bif, Context},
    vm::VM,
  },
  fail::RtResult,
  defs::stack::IStack,
  term::{boxed, lterm::*},
};


fn module() -> &'static str {
  "opcodes::op_execution: "
}


/// Perform a call to a `location` in code, storing address of the next opcode
/// in `ctx.cp`.
#[inline]
pub fn opcode_call(
  _vm: &VM,
  ctx: &mut Context,
  _curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: call(arity:int, loc:CP)
  assert_arity(gen_op::OPCODE_CALL, 2);

  let arity = ctx.fetch_term();
  ctx.live = arity.get_small_unsigned();

  let location = ctx.fetch_term();
  debug_assert!(
    location.is_boxed(),
    "Call location must be a box (have {})",
    location
  );

  ctx.cp = ctx.ip; // Points at the next opcode after this
  ctx.ip = CodePtr::from_cp(location);

  Ok(DispatchResult::Normal)
}


/// Perform a call to a `location` in code, the `ctx.cp` is not updated.
/// Behaves like a jump?
#[inline]
pub fn opcode_call_only(
  _vm: &VM,
  ctx: &mut Context,
  _curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: call_only(arity:int, loc:cp)
  assert_arity(gen_op::OPCODE_CALL_ONLY, 2);

  let arity = ctx.fetch_term();
  ctx.live = arity.get_small_unsigned();

  let location = ctx.fetch_term();
  debug_assert!(
    location.is_boxed(),
    "Call location must be a box (have {})",
    location
  );

  ctx.ip = CodePtr::from_cp(location);

  Ok(DispatchResult::Normal)
}


/// Performs a tail recursive call to a Destination mfarity (a `HOImport`
/// object on the heap which contains `Mod`, `Fun`, and  `Arity`) which can
/// point to an external function or a BIF. Does not update the `ctx.cp`.
#[inline]
pub fn opcode_call_ext_only(
  vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: call_ext_only(arity:int, import:boxed)
  assert_arity(gen_op::OPCODE_CALL_EXT_ONLY, 2);

  let arity = ctx.fetch_term().get_small_unsigned();
  let args = ctx.registers_slice(arity);
  shared_call_ext(vm, ctx, curr_p, LTerm::nil(), args, false)
}


/// Performs a call to a Destination mfarity (a `HOImport` object on the heap
/// which contains `Mod`, `Fun`, and  `Arity`) which can point to an external
/// function or a BIF. Updates the `ctx.cp` with return IP.
#[inline]
pub fn opcode_call_ext(
  vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: call_ext(arity:int, destination:boxed)
  assert_arity(gen_op::OPCODE_CALL_EXT, 2);

  let arity = ctx.fetch_term().get_small_unsigned();
  let args = ctx.registers_slice(arity);
  shared_call_ext(vm, ctx, curr_p, LTerm::nil(), args, true)
}


#[inline]
fn shared_call_ext(
  vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
  fail_label: LTerm,
  args: &[LTerm],
  save_cp: bool,
) -> RtResult<DispatchResult> {
  ctx.live = args.len();

  // HOImport object on heap which contains m:f/arity
  let imp0 = ctx.fetch_term();

  match unsafe { boxed::Import::const_from_term(imp0) } {
    Ok(import_ptr) => unsafe {
      if (*import_ptr).is_bif {
        // Perform a BIF application
        let cb_target = call_bif::CallBifTarget::ImportPointer(import_ptr);
        call_bif::apply(
          ctx,
          curr_p,
          fail_label,
          cb_target,
          args,
          LTerm::make_xreg(0),
          true,
        )
      } else {
        // Perform a regular call to BEAM code, save CP and jump
        //
        if save_cp {
          ctx.cp = ctx.ip; // Points at the next opcode after this
        }
        ctx.ip = (*import_ptr)
          .resolve(vm.code_server.borrow_mut().as_mut())
          .unwrap();
        Ok(DispatchResult::Normal)
      }
    },
    Err(_err) => {
      // Create a `{badfun, _}` error
      //panic!("bad call_ext target {}", imp0);
      DispatchResult::badfun_val(imp0, &mut curr_p.heap)
    }
  }
}


/// Jump to the value in `ctx.cp`, set `ctx.cp` to NULL. Empty stack means that
/// the process has no more code to execute and will end with reason `normal`.
#[inline]
pub fn opcode_return(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: return()
  assert_arity(gen_op::OPCODE_RETURN, 0);

  if ctx.cp.is_null() {
    if curr_p.heap.stack_depth() == 0 {
      // Process end of life: return on empty stack
      panic!("{}Process exit: normal; x0={}", module(), ctx.x(0))
    } else {
      panic!("{}Return instruction with 0 in ctx.cp", module())
    }
  }

  ctx.ip = ctx.cp;
  ctx.cp = CodePtr::null();

  Ok(DispatchResult::Normal)
}


#[inline]
pub fn opcode_func_info(
  _vm: &VM,
  ctx: &mut Context,
  _curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  assert_arity(gen_op::OPCODE_FUNC_INFO, 3);
  let m = ctx.fetch_term();
  let f = ctx.fetch_term();
  let arity = ctx.fetch_term();

  panic!("{}function_clause {}:{}/{}", module(), m, f, arity)
  //DispatchResult::Error
}


/// Create an error:badmatch exception
#[inline]
pub fn opcode_badmatch(
  _vm: &VM,
  ctx: &mut Context,
  curr_p: &mut Process,
) -> RtResult<DispatchResult> {
  // Structure: badmatch(LTerm)
  assert_arity(gen_op::OPCODE_BADMATCH, 1);

  let hp = &mut curr_p.heap;
  let val = ctx.fetch_and_load(hp);
  DispatchResult::badmatch_val(val, hp)
}
