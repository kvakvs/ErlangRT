//! Module implements opcodes related to execution control: Calls, jumps,
//! returns etc.
use crate::{
  beam::disp_result::DispatchResult,
  defs::exc_type::ExceptionType,
  emulator::{
    gen_atoms,
    process::Process,
    runtime_ctx::{
      call_native_fun::{self, find_and_call_native_fun},
      Context, ReturnResult,
    },
    vm::VM,
  },
  fail::{self, RtErr, RtResult},
  term::{boxed, compare, value::*},
};
use core::cmp::Ordering;

fn module() -> &'static str {
  "opcodes::op_execution: "
}

// Perform a call to a `location` in code, storing address of the next opcode
// in `ctx.cp`.
// Structure: call(arity:int, loc:CP)
define_opcode!(_vm, ctx, _curr_p,
  name: OpcodeCall, arity: 2,
  run: { Self::call(ctx, arity, dst) },
  args: usize(arity), term(dst),
);

impl OpcodeCall {
  #[inline]
  pub fn call(ctx: &mut Context, arity: usize, dst: Term) -> RtResult<DispatchResult> {
    ctx.live = arity;
    debug_assert!(dst.is_boxed(), "Call location must be a box (have {})", dst);

    ctx.debug_trace_call("opcode:call", dst, 0, arity);

    ctx.cp = ctx.ip; // Points at the next opcode after this
    ctx.jump(dst);

    Ok(DispatchResult::Normal)
  }
}

// Perform a call to a `location` in code, the `ctx.cp` is not updated.
// Behaves like a jump?
// Structure: call_only(arity:int, dst:cp)
define_opcode!(_vm, ctx, _curr_p,
  name: OpcodeCallOnly, arity: 2,
  run: { Self::call_only(ctx, arity, dst) },
  args: usize(arity), term(dst),
);

impl OpcodeCallOnly {
  #[inline]
  pub fn call_only(
    ctx: &mut Context,
    arity: usize,
    dst: Term,
  ) -> RtResult<DispatchResult> {
    ctx.live = arity;
    ctx.debug_trace_call("opcode:call_only", dst, 0, arity);
    ctx.jump(dst); // jump will assert if the location is cp
    Ok(DispatchResult::Normal)
  }
}

// Deallocates stack, and performs a tail-recursive call (jump) to a `location`
// in code.
// Structure: call_last(arity:smallint, dst:cp, dealloc:smallint)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeCallLast, arity: 3,
  run: { Self::call_last(ctx, curr_p, arity, dst, dealloc) },
  args: usize(arity), term(dst), usize(dealloc),
);

impl OpcodeCallLast {
  #[inline]
  pub fn call_last(
    ctx: &mut Context,
    curr_p: &mut Process,
    arity: usize,
    dst: Term,
    dealloc: usize,
  ) -> RtResult<DispatchResult> {
    ctx.live = arity;
    let hp = &mut curr_p.heap;
    ctx.set_cp(hp.stack_deallocate(dealloc));
    ctx.debug_trace_call("opcode:call_last", dst, 0, arity);
    ctx.jump(dst);
    Ok(DispatchResult::Normal)
  }
}

// Performs a tail recursive call to a Destination mfarity (an `Import`
// object on the heap which contains `Mod`, `Fun`, and  `Arity`) which can
// point to an external BEAM fn or a native fn. Does not update the `ctx.cp`.
// Structure: call_ext_only(arity:int, import:boxed)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeCallExtOnly, arity: 2,
  run: { Self::call_ext_only(vm, ctx, curr_p, arity, dst) },
  args: usize(arity), term(dst),
);

impl OpcodeCallExtOnly {
  #[inline]
  pub fn call_ext_only(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    arity: usize,
    dst: Term,
  ) -> RtResult<DispatchResult> {
    let args = ctx.registers_slice(0, arity);
    ctx.debug_trace_call("opcode:call_ext_only", dst, 0, arity);
    generic_call_ext(vm, ctx, curr_p, dst, Term::nil(), args, false)
  }
}

// Performs a call to a Destination mfarity (an `Import` object on the heap
// which contains `Mod`, `Fun`, and  `Arity`) which can point to an external
// function or a BIF. Updates the `ctx.cp` with return IP.
// Structure: call_ext(arity:int, destination:boxed)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeCallExt, arity: 2,
  run: { Self::call_ext(vm, ctx, curr_p, arity, dst) },
  args: usize(arity), term(dst),
);

impl OpcodeCallExt {
  #[inline]
  pub fn call_ext(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    arity: usize,
    dst: Term,
  ) -> RtResult<DispatchResult> {
    let args = ctx.registers_slice(0, arity);
    ctx.debug_trace_call("opcode:call_ext", dst, 0, arity);
    generic_call_ext(vm, ctx, curr_p, dst, Term::nil(), args, true)
  }
}

// Deallocates stack and performs a tail call to destination.
// Structure: call_ext_last(arity:int, destination:boxed, dealloc:smallint)
define_opcode!(vm, ctx, curr_p,
  name: OpcodeCallExtLast, arity: 3,
  run: { Self::call_ext_last(vm, ctx, curr_p, arity, dst, dealloc) },
  args: usize(arity), term(dst), usize(dealloc),
);

impl OpcodeCallExtLast {
  #[inline]
  pub fn call_ext_last(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
    arity: usize,
    dst: Term,
    dealloc: usize,
  ) -> RtResult<DispatchResult> {
    let new_cp = curr_p.heap.stack_deallocate(dealloc);
    ctx.set_cp(new_cp);
    let args = ctx.registers_slice(0, arity);
    ctx.debug_trace_call("opcode:call_ext_last", dst, 0, arity);
    generic_call_ext(vm, ctx, curr_p, dst, Term::nil(), args, false)
  }
}

/// Arg: dst_import: boxed::Import which will contain MFArity to call.
#[inline]
fn generic_call_ext(
  vm: &mut VM,
  ctx: &mut Context,
  proc: &mut Process,
  dst_import: Term,
  fail_label: Term,
  args: &[Term],
  save_cp: bool,
) -> RtResult<DispatchResult> {
  ctx.live = args.len();

  match unsafe { boxed::Import::mut_from_term(dst_import) } {
    Ok(import_ptr) => unsafe {
      if (*import_ptr).get_is_bif(&vm.code_server) {
        // Perform a BIF application
        let cb_target = call_native_fun::CallBifTarget::ImportPointer(import_ptr);
        let native_dispatch_result = find_and_call_native_fun(
          vm,
          ctx,
          proc,
          fail_label,
          cb_target,
          args,
          Term::make_register_x(0),
          true,
        );
        if save_cp {
          return native_dispatch_result;
        } else {
          // Perform inline return like if it was a tail recursive call
          // Because tail call might happen on an empty stack, the return with
          // empty stack will end the process life here (no more code).
          match ctx.return_and_clear_cp(proc) {
            ReturnResult::EmptyStack => return Ok(DispatchResult::Finished),
            ReturnResult::Success => return native_dispatch_result,
          };
        }
      } else {
        // Perform a regular call to BEAM code, save CP and jump
        //
        if save_cp {
          ctx.cp = ctx.ip; // Points at the next opcode after this
        }
        let code_srv = vm.get_code_server_p();
        let import_dst = (*import_ptr).resolve(&mut (*code_srv))?;
        ctx.jump_ptr(import_dst.get_pointer());
        Ok(DispatchResult::Normal)
      }
    },
    Err(_err) => {
      // Create a `{badfun, _}` error
      // panic!("bad call_ext target {}", imp0);
      fail::create::badfun_val(dst_import, &mut proc.heap)
    }
  }
}

// Jump to the value in `ctx.cp`, set `ctx.cp` to NULL. Empty stack means that
// the process has no more code to execute and will end with reason `normal`.
// Structure: return()
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeReturn, arity: 0,
  run: { Self::return_opcode(ctx, curr_p) },
  args:
);

impl OpcodeReturn {
  #[inline]
  pub fn return_opcode(
    ctx: &mut Context,
    proc: &mut Process,
  ) -> RtResult<DispatchResult> {
    match ctx.return_and_clear_cp(proc) {
      ReturnResult::EmptyStack => Ok(DispatchResult::Finished),
      ReturnResult::Success => Ok(DispatchResult::Normal),
    }
  }
}

define_opcode!(_vm, ctx, proc,
  name: OpcodeFuncInfo, arity: 3,
  run: { Self::func_info(proc, m, f, arity) },
  args: term(m), term(f), usize(arity),
);

impl OpcodeFuncInfo {
  #[inline]
  pub fn func_info(
    proc: &Process,
    m: Term,
    f: Term,
    arity: usize,
  ) -> RtResult<DispatchResult> {
    if cfg!(debug_assertions) {
      println!("{}function_clause {}:{}/{}", module(), m, f, arity);
      proc.context.dump_registers(arity);
    }
    Err(RtErr::Exception(
      ExceptionType::Error,
      gen_atoms::FUNCTION_CLAUSE,
    ))
  }
}

// Create an error:badmatch exception
// Structure: badmatch(Term)
define_opcode!(_vm, _ctx, curr_p,
  name: OpcodeBadmatch, arity: 1,
  run: { Self::badmatch(curr_p, val) },
  args: load(val),
);

impl OpcodeBadmatch {
  #[inline]
  pub fn badmatch(curr_p: &mut Process, val: Term) -> RtResult<DispatchResult> {
    let hp = &mut curr_p.heap;
    fail::create::badmatch_val(val, hp)
  }
}

// Compares Arg with tuple of pairs {Value1, Label1, ...} and jumps to Label
// if it is equal. If none compared, will jump to FailLabel
// Structure: select_val(val:src, on_fail:label, tuple_pairs:src)
define_opcode!(_vm, ctx, _curr_p,
  name: OpcodeSelectVal, arity: 3,
  run: { Self::select_val(ctx, val, fail, pairs) },
  args: load(val), cp_or_nil(fail), literal_tuple(pairs),
);

impl OpcodeSelectVal {
  #[inline]
  pub fn select_val(
    ctx: &mut Context,
    val: Term,
    fail: Term,
    pairs: *const boxed::Tuple,
  ) -> RtResult<DispatchResult> {
    let pairs_count = unsafe { (*pairs).get_arity() / 2 };

    for i in 0..pairs_count {
      let sel_val = unsafe { (*pairs).get_element(i * 2) };
      if compare::cmp_terms(val, sel_val, true)? == Ordering::Equal {
        let sel_label = unsafe { (*pairs).get_element(i * 2 + 1) };
        ctx.jump(sel_label);
        return Ok(DispatchResult::Normal);
      }
    }

    // None matched, jump to fail label
    ctx.jump(fail);
    Ok(DispatchResult::Normal)
  }
}

// Jumps to label.
// Structure: jump(dst:label)
define_opcode!(_vm, ctx, _curr_p,
  name: OpcodeJump, arity: 1,
  run: { Self::jump(ctx, dst) },
  args: term(dst),
);

impl OpcodeJump {
  #[inline]
  pub fn jump(ctx: &mut Context, dst: Term) -> RtResult<DispatchResult> {
    ctx.jump(dst);
    Ok(DispatchResult::Normal)
  }
}
