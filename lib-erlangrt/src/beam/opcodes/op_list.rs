//! Module implements opcodes related to lists manipulation.

use crate::{
  beam::disp_result::DispatchResult,
  emulator::{heap::allocate_cons, process::Process, runtime_ctx::Context, },
  fail::{self, RtResult},
  term::value::Term,
};

// Read the source `value` and check whether it is a list and not NIL. On
// false jump to the label `fail`.
// Structure: is_nonempty_list(fail:cp, value:src)
define_opcode!(_vm, ctx, _curr_p,
  name: OpcodeIsNonemptyList, arity: 2,
  run: { Self::is_nonempty_list(ctx, fail, value) },
  args: cp_or_nil(fail), load(value),
);

impl OpcodeIsNonemptyList {
  #[inline]
  pub fn is_nonempty_list(
    ctx: &mut Context,
    fail: Term,
    value: Term,
  ) -> RtResult<DispatchResult> {
    if value == Term::nil() && !value.is_cons() && fail != Term::nil() {
      // jump to fail label
      ctx.jump(fail)
    }
    Ok(DispatchResult::Normal)
  }
}

// Check whether the value `value` is an empty list, jump to the `fail` label
// if it is not NIL.
// Structure: is_nil(fail:CP, value:src)
define_opcode!(_vm, ctx, _curr_p,
  name: OpcodeIsNil, arity: 2,
  run: { Self::is_nil(ctx, fail, value) },
  args: cp_or_nil(fail), load(value),
);

impl OpcodeIsNil {
  #[inline]
  pub fn is_nil(
    ctx: &mut Context,
    fail: Term,
    value: Term,
  ) -> RtResult<DispatchResult> {
    if value != Term::nil() && fail != Term::nil() {
      // jump to fail label
      ctx.jump(fail)
    }
    Ok(DispatchResult::Normal)
  }
}

// Take a list `value` and split it into a head and tail, they are stored in
// `hd` and `tl` destinations respectively.
// Structure: get_list(value:src, hd:dst, tl:dst)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeGetList, arity: 3,
  run: { Self::decons(ctx, curr_p, src, dst_hd, dst_tl) },
  args: load(src), term(dst_hd), term(dst_tl),
);

impl OpcodeGetList {
  #[inline]
  pub fn decons(
    ctx: &mut Context,
    curr_p: &mut Process,
    src: Term,
    dst_hd: Term,
    dst_tl: Term,
  ) -> RtResult<DispatchResult> {
    if src == Term::nil() {
      // TODO: is this badmatch here?
      panic!("Attempt to get_list on a nil[]");
    }

    let hp = curr_p.get_heap_mut();
    if !src.is_cons() {
      return fail::create::badarg_val(src, hp);
    }

    unsafe {
      let cons_p = src.get_cons_ptr();
      ctx.store_value((*cons_p).hd(), dst_hd, hp)?;
      ctx.store_value((*cons_p).tl(), dst_tl, hp)?;
    }

    Ok(DispatchResult::Normal)
  }
}

// Given head and tail sources, `hd` and `tl`, read them and compose into a
// new list cell which is stored into `dst`.
// Structure: put_list(hd:src, tl:src, dst:dst)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodePutList, arity: 3,
  run: { Self::cons(ctx, curr_p, src_hd, src_tl, dst) },
  args: load(src_hd), load(src_tl), term(dst),
);

impl OpcodePutList {
  #[inline]
  pub fn cons(
    ctx: &mut Context,
    curr_p: &mut Process,
    src_hd: Term,
    src_tl: Term,
    dst: Term,
  ) -> RtResult<DispatchResult> {
    let hp = curr_p.get_heap_mut();

    unsafe {
      let cons_p = allocate_cons(hp).unwrap();
      (*cons_p).set_hd(src_hd);
      (*cons_p).set_tl(src_tl);
      ctx.store_value(Term::make_cons(cons_p), dst, hp)?;
    }

    Ok(DispatchResult::Normal)
  }
}

// Retrieve head of a cons cell.
// Structure: get_hd(cons:src, dst:dst)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeGetHd, arity: 2,
  run: { Self::hd(ctx, curr_p, cons, dst) },
  args: load(cons), term(dst),
);

impl OpcodeGetHd {
  #[inline]
  pub fn hd(
    ctx: &mut Context,
    curr_p: &mut Process,
    cons: Term,
    dst: Term,
  ) -> RtResult<DispatchResult> {
    let hp = curr_p.get_heap_mut();
    let val = unsafe { (*cons.get_cons_ptr()).hd() };
    ctx.store_value(val, dst, hp)?;
    Ok(DispatchResult::Normal)
  }
}

// Retrieve tail of a cons cell.
// Structure: get_tl(cons:src, dst:dst)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeGetTl, arity: 2,
  run: { Self::tl(ctx, curr_p, cons, dst) },
  args: load(cons), term(dst),
);

impl OpcodeGetTl {
  #[inline]
  pub fn tl(
    ctx: &mut Context,
    curr_p: &mut Process,
    cons: Term,
    dst: Term,
  ) -> RtResult<DispatchResult> {
    let hp = curr_p.get_heap_mut();
    let val = unsafe { (*cons.get_cons_ptr()).tl() };
    ctx.store_value(val, dst, hp)?;
    Ok(DispatchResult::Normal)
  }
}
