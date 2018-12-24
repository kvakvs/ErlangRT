//! Module implements opcodes related to execution control: Calls, jumps,
//! returns etc.

use crate::{
  beam::disp_result::DispatchResult,
  emulator::{
    code::CodePtr,
    process::Process,
    runtime_ctx::{call_bif, Context},
    vm::VM,
  },
  fail::RtResult,
  term::{boxed, compare, lterm::*},
};
use std::cmp::Ordering;

fn module() -> &'static str {
  "opcodes::op_execution: "
}

/// Perform a call to a `location` in code, storing address of the next opcode
/// in `ctx.cp`.
/// Structure: call(arity:int, loc:CP)
pub struct OpcodeCall {}

impl OpcodeCall {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    _curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let arity = ctx.fetch_term();
    ctx.live = arity.get_small_unsigned();

    let location = ctx.fetch_term();
    debug_assert!(
      location.is_boxed(),
      "Call location must be a box (have {})",
      location
    );

    ctx.cp = ctx.ip; // Points at the next opcode after this
    ctx.jump(location);

    Ok(DispatchResult::Normal)
  }
}

/// Perform a call to a `location` in code, the `ctx.cp` is not updated.
/// Behaves like a jump?
/// Structure: call_only(arity:int, loc:cp)
pub struct OpcodeCallOnly {}

impl OpcodeCallOnly {
  pub const ARITY: usize = 2;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    _curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let arity = ctx.fetch_term();
    ctx.live = arity.get_small_unsigned();

    let dst = ctx.fetch_term();
    ctx.jump(dst); // jump will assert if the location is cp
    Ok(DispatchResult::Normal)
  }
}

/// Deallocates stack, and performs a tail-recursive call (jump) to a `location`
/// in code.
/// Structure: call_last(arity:smallint, dst:cp, dealloc:smallint)
pub struct OpcodeCallLast {}

impl OpcodeCallLast {
  pub const ARITY: usize = 3;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let arity = ctx.fetch_term();
    ctx.live = arity.get_small_unsigned();

    let dst = ctx.fetch_term(); // jump will assert if the location is cp

    let dealloc = ctx.fetch_term();
    let hp = &mut curr_p.heap;
    ctx.set_cp(hp.stack_deallocate(dealloc.get_small_unsigned()));

    ctx.jump(dst);
    Ok(DispatchResult::Normal)
  }
}

/// Performs a tail recursive call to a Destination mfarity (a `HOImport`
/// object on the heap which contains `Mod`, `Fun`, and  `Arity`) which can
/// point to an external function or a BIF. Does not update the `ctx.cp`.
/// Structure: call_ext_only(arity:int, import:boxed)
pub struct OpcodeCallExtOnly {}

impl OpcodeCallExtOnly {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (usize, LTerm) {
    let arity = ctx.fetch_term().get_small_unsigned();
    let dst = ctx.fetch_and_load(&mut curr_p.heap);
    (arity, dst)
  }

  #[inline]
  pub fn run(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (arity, dst) = Self::fetch_args(ctx, curr_p);
    let args = ctx.registers_slice(arity);
    shared_call_ext(vm, ctx, curr_p, dst, LTerm::nil(), args, false)
  }
}

/// Performs a call to a Destination mfarity (a `HOImport` object on the heap
/// which contains `Mod`, `Fun`, and  `Arity`) which can point to an external
/// function or a BIF. Updates the `ctx.cp` with return IP.
/// Structure: call_ext(arity:int, destination:boxed)
pub struct OpcodeCallExt {}

impl OpcodeCallExt {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (usize, LTerm) {
    let arity = ctx.fetch_term().get_small_unsigned();
    let dst = ctx.fetch_and_load(&mut curr_p.heap);
    (arity, dst)
  }

  #[inline]
  pub fn run(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (arity, dst) = Self::fetch_args(ctx, curr_p);
    let args = ctx.registers_slice(arity);
    shared_call_ext(vm, ctx, curr_p, dst, LTerm::nil(), args, true)
  }
}


/// Deallocates stack and performs a tail call to destination.
/// Structure: call_ext_last(arity:int, destination:boxed, dealloc:smallint)
pub struct OpcodeCallExtLast {}

impl OpcodeCallExtLast {
  pub const ARITY: usize = 3;

  #[inline]
  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (usize, LTerm, usize) {
    let arity = ctx.fetch_term().get_small_unsigned();
    let dst = ctx.fetch_and_load(&mut curr_p.heap);
    let dealloc = ctx.fetch_term().get_small_unsigned();
    (arity, dst, dealloc)
  }

  #[inline]
  pub fn run(
    vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (arity, dst, dealloc) = Self::fetch_args(ctx, curr_p);

    let new_cp = curr_p.heap.stack_deallocate(dealloc);
    ctx.set_cp(new_cp);

    let args = ctx.registers_slice(arity);
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
      DispatchResult::badfun_val(dst_import, &mut curr_p.heap)
    }
  }
}

/// Jump to the value in `ctx.cp`, set `ctx.cp` to NULL. Empty stack means that
/// the process has no more code to execute and will end with reason `normal`.
/// Structure: return()
pub struct OpcodeReturn {}

impl OpcodeReturn {
  pub const ARITY: usize = 0;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    if ctx.cp.is_null() {
      if curr_p.heap.stack_depth() == 0 {
        // Process end of life: return on empty stack
        panic!("{}Process exit: normal; x0={}", module(), ctx.get_x(0))
      } else {
        panic!(
          "{}Return instruction with 0 in ctx.cp. Possible error in CP value management",
          module()
        )
      }
    }

    ctx.ip = ctx.cp;
    ctx.cp = CodePtr::null();

    Ok(DispatchResult::Normal)
  }
}

pub struct OpcodeFuncInfo {}

impl OpcodeFuncInfo {
  pub const ARITY: usize = 3;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    _curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let m = ctx.fetch_term();
    let f = ctx.fetch_term();
    let arity = ctx.fetch_term();

    panic!("{}function_clause {}:{}/{}", module(), m, f, arity)
    // DispatchResult::Error
  }
}

/// Create an error:badmatch exception
/// Structure: badmatch(LTerm)
pub struct OpcodeBadmatch {}

impl OpcodeBadmatch {
  pub const ARITY: usize = 1;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let hp = &mut curr_p.heap;
    let val = ctx.fetch_and_load(hp);
    DispatchResult::badmatch_val(val, hp)
  }
}

/// Compares Arg with tuple of pairs {Value1, Label1, ...} and jumps to Label
/// if it is equal. If none compared, will jump to FailLabel
/// Structure: select_val(val:src, on_fail:label, tuple_pairs:src)
pub struct OpcodeSelectVal {}

impl OpcodeSelectVal {
  pub const ARITY: usize = 3;

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let hp = &curr_p.heap;
    let val = ctx.fetch_and_load(hp);
    let fail_label = ctx.fetch_term();

    let pairs_tuple = ctx.fetch_and_load(hp);
    debug_assert!(pairs_tuple.is_tuple());
    let tuple_ptr = pairs_tuple.get_box_ptr::<boxed::Tuple>();
    let pairs_count = unsafe { (*tuple_ptr).get_arity() / 2 };

    for i in 0..pairs_count {
      let sel_val = unsafe { boxed::Tuple::get_element_base0(tuple_ptr, i * 2) };
      if compare::cmp_terms(val, sel_val, true)? == Ordering::Equal {
        let sel_label = unsafe { boxed::Tuple::get_element_base0(tuple_ptr, i * 2 + 1) };
        ctx.jump(sel_label);
        return Ok(DispatchResult::Normal);
      }
    }

    // None matched, jump to fail label
    ctx.jump(fail_label);
    Ok(DispatchResult::Normal)
  }
}
