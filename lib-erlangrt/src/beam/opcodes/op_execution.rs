//! Module implements opcodes related to execution control: Calls, jumps,
//! returns etc.
use crate::{
  beam::disp_result::DispatchResult,
  emulator::{
    process::Process,
    runtime_ctx::{call_bif, Context},
    vm::VM,
  },
  fail::{self, RtResult},
  term::{boxed, compare, lterm::*},
};
use core::cmp::Ordering;

fn module() -> &'static str {
  "opcodes::op_execution: "
}

/// Perform a call to a `location` in code, storing address of the next opcode
/// in `ctx.cp`.
/// Structure: call(arity:int, loc:CP)
define_opcode!(_vm, ctx, _curr_p,
  name: OpcodeCall, arity: 2,
  run: { Self::call(ctx, arity, dst) },
  args: usize(arity), term(dst)
);

impl OpcodeCall {
  #[inline]
  pub fn call(ctx: &mut Context, arity: usize, dst: LTerm) -> RtResult<DispatchResult> {
    ctx.live = arity;
    debug_assert!(dst.is_boxed(), "Call location must be a box (have {})", dst);

    ctx.cp = ctx.ip; // Points at the next opcode after this
    ctx.jump(dst);

    Ok(DispatchResult::Normal)
  }
}

/// Perform a call to a `location` in code, the `ctx.cp` is not updated.
/// Behaves like a jump?
/// Structure: call_only(arity:int, dst:cp)
define_opcode!(_vm, ctx, _curr_p,
  name: OpcodeCallOnly, arity: 2,
  run: { Self::call_only(ctx, arity, dst) },
  args: usize(arity), term(dst)
);

impl OpcodeCallOnly {
  #[inline]
  pub fn call_only(
    ctx: &mut Context,
    arity: usize,
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    ctx.live = arity;
    ctx.jump(dst); // jump will assert if the location is cp
    Ok(DispatchResult::Normal)
  }
}

/// Deallocates stack, and performs a tail-recursive call (jump) to a `location`
/// in code.
/// Structure: call_last(arity:smallint, dst:cp, dealloc:smallint)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeCallLast, arity: 3,
  run: { Self::call_last(ctx, curr_p, arity, dst, dealloc) },
  args: usize(arity), term(dst), usize(dealloc)
);

impl OpcodeCallLast {
  #[inline]
  pub fn call_last(
    ctx: &mut Context,
    curr_p: &mut Process,
    arity: usize,
    dst: LTerm,
    dealloc: usize,
  ) -> RtResult<DispatchResult> {
    ctx.live = arity;
    let hp = &mut curr_p.heap;
    ctx.set_cp(hp.stack_deallocate(dealloc));
    ctx.jump(dst);
    Ok(DispatchResult::Normal)
  }
}

/// Performs a tail recursive call to a Destination mfarity (a `HOImport`
/// object on the heap which contains `Mod`, `Fun`, and  `Arity`) which can
/// point to an external function or a BIF. Does not update the `ctx.cp`.
/// Structure: call_ext_only(arity:int, import:boxed)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeCallExtOnly, arity: 2,
  run: { Self::call_ext_only(vm, ctx, curr_p, arity, dst) },
  args: usize(arity), term(dst)
);

impl OpcodeCallExtOnly {
  #[inline]
  pub fn call_ext_only(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    arity: usize,
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    let args = ctx.registers_slice(0, arity);
    shared_call_ext(vm, ctx, curr_p, dst, LTerm::nil(), args, false)
  }
}

/// Performs a call to a Destination mfarity (a `HOImport` object on the heap
/// which contains `Mod`, `Fun`, and  `Arity`) which can point to an external
/// function or a BIF. Updates the `ctx.cp` with return IP.
/// Structure: call_ext(arity:int, destination:boxed)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeCallExt, arity: 2,
  run: { Self::call_ext(vm, ctx, curr_p, arity, dst) },
  args: usize(arity), term(dst)
);

impl OpcodeCallExt {
  #[inline]
  pub fn call_ext(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    arity: usize,
    dst: LTerm,
  ) -> RtResult<DispatchResult> {
    let args = ctx.registers_slice(0, arity);
    shared_call_ext(vm, ctx, curr_p, dst, LTerm::nil(), args, true)
  }
}

/// Deallocates stack and performs a tail call to destination.
/// Structure: call_ext_last(arity:int, destination:boxed, dealloc:smallint)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeCallExtLast, arity: 3,
  run: { Self::call_ext_last(vm, ctx, curr_p, arity, dst, dealloc) },
  args: usize(arity), term(dst), usize(dealloc)
);

impl OpcodeCallExtLast {
  #[inline]
  pub fn call_ext_last(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    arity: usize,
    dst: LTerm,
    dealloc: usize,
  ) -> RtResult<DispatchResult> {
    let new_cp = curr_p.heap.stack_deallocate(dealloc);
    ctx.set_cp(new_cp);
    let args = ctx.registers_slice(0, arity);
    shared_call_ext(vm, ctx, curr_p, dst, LTerm::nil(), args, false)
  }
}

/// Arg: dst_import: boxed::Import which will contain MFArity to call.
#[inline]
fn shared_call_ext(
  vm: &mut VM,
  ctx: &mut Context,
  curr_p: &mut Process,
  dst_import: LTerm,
  fail_label: LTerm,
  args: &[LTerm],
  save_cp: bool,
) -> RtResult<DispatchResult> {
  ctx.live = args.len();

  match unsafe { boxed::Import::const_from_term(dst_import) } {
    Ok(import_ptr) => unsafe {
      if (*import_ptr).is_bif {
        // Perform a BIF application
        let cb_target = call_bif::CallBifTarget::ImportPointer(import_ptr);
        call_bif::find_and_call_bif(
          vm,
          ctx,
          curr_p,
          fail_label,
          cb_target,
          args,
          LTerm::make_regx(0),
          true,
        )
      } else {
        // Perform a regular call to BEAM code, save CP and jump
        //
        if save_cp {
          ctx.cp = ctx.ip; // Points at the next opcode after this
        }
        let cs = vm.get_code_server_p();
        ctx.ip = (*import_ptr).resolve(&mut (*cs))?;
        Ok(DispatchResult::Normal)
      }
    },
    Err(_err) => {
      // Create a `{badfun, _}` error
      // panic!("bad call_ext target {}", imp0);
      fail::create::badfun_val(dst_import, &mut curr_p.heap)
    }
  }
}

/// Jump to the value in `ctx.cp`, set `ctx.cp` to NULL. Empty stack means that
/// the process has no more code to execute and will end with reason `normal`.
/// Structure: return()
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeReturn, arity: 0,
  run: { Self::return_opcode(ctx, curr_p) },
  args:
);

impl OpcodeReturn {
  #[inline]
  pub fn return_opcode(
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    if ctx.cp.is_null() {
      if curr_p.heap.stack_depth() == 0 {
        // Process end of life: return on empty stack
        println!(
          "Process end of life (return on empty stack) x0={}",
          ctx.get_x(0)
        );
        // return Err(Error::Exception(ExceptionType::Exit, gen_atoms::NORMAL));
        return Ok(DispatchResult::Finished);
      } else {
        panic!(
          "{}Return instruction with null CP and nonempty stack. Possible error in CP value management",
          module()
        )
      }
    }

    ctx.jump_ptr(ctx.cp.get_pointer());
    ctx.clear_cp();

    Ok(DispatchResult::Normal)
  }
}

define_opcode!(_vm, ctx, _curr_p,
  name: OpcodeFuncInfo, arity: 3,
  run: { Self::func_info(m, f, arity) },
  args: term(m), term(f), usize(arity)
);

impl OpcodeFuncInfo {
  #[inline]
  pub fn func_info(
    m: LTerm,
    f: LTerm,
    arity: usize,
  ) -> RtResult<DispatchResult> {
    panic!("{}function_clause {}:{}/{}", module(), m, f, arity);
    // DispatchResult::Error
  }
}

/// Create an error:badmatch exception
/// Structure: badmatch(LTerm)
define_opcode!(_vm, _ctx, curr_p,
  name: OpcodeBadmatch, arity: 1,
  run: { Self::badmatch(curr_p, val) },
  args: load(val)
);

impl OpcodeBadmatch {
  #[inline]
  pub fn badmatch(
    curr_p: &mut Process,
    val: LTerm,
  ) -> RtResult<DispatchResult> {
    let hp = &mut curr_p.heap;
    fail::create::badmatch_val(val, hp)
  }
}

/// Compares Arg with tuple of pairs {Value1, Label1, ...} and jumps to Label
/// if it is equal. If none compared, will jump to FailLabel
/// Structure: select_val(val:src, on_fail:label, tuple_pairs:src)
define_opcode!(_vm, ctx, _curr_p,
  name: OpcodeSelectVal, arity: 3,
  run: { Self::select_val(ctx, val, fail, pairs) },
  args: load(val), cp_not_nil(fail), literal_tuple(pairs)
);

impl OpcodeSelectVal {
  #[inline]
  pub fn select_val(
    ctx: &mut Context,
    val: LTerm,
    fail: LTerm,
    pairs: *const boxed::Tuple,
  ) -> RtResult<DispatchResult> {
    let pairs_count = unsafe { (*pairs).get_arity() / 2 };

    for i in 0..pairs_count {
      let sel_val = unsafe { boxed::Tuple::get_element_base0(pairs, i * 2) };
      if compare::cmp_terms(val, sel_val, true)? == Ordering::Equal {
        let sel_label = unsafe { boxed::Tuple::get_element_base0(pairs, i * 2 + 1) };
        ctx.jump(sel_label);
        return Ok(DispatchResult::Normal);
      }
    }

    // None matched, jump to fail label
    ctx.jump(fail);
    Ok(DispatchResult::Normal)
  }
}

/// Jumps to label.
/// Structure: jump(dst:label)
define_opcode!(_vm, ctx, _curr_p,
  name: OpcodeJump, arity: 1,
  run: { Self::jump(ctx, dst) },
  args: term(dst)
);

impl OpcodeJump {
  #[inline]
  pub fn jump(ctx: &mut Context, dst: LTerm) -> RtResult<DispatchResult> {
    ctx.jump(dst);
    Ok(DispatchResult::Normal)
  }
}
