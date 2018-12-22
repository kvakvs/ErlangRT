//! Module implements opcodes related to lists manipulation.

use crate::{
  beam::disp_result::DispatchResult,
  emulator::{heap::allocate_cons, process::Process, runtime_ctx::Context, vm::VM},
  fail::RtResult,
  term::lterm::LTerm,
};

fn module() -> &'static str {
  "opcodes::op_list: "
}

/// Read the source `value` and check whether it is a list and not NIL. On
/// false jump to the label `fail`.
/// Structure: is_nonempty_list(fail:cp, value:src)
pub struct OpcodeIsNonemptyList {}

impl OpcodeIsNonemptyList {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (LTerm, LTerm) {
    let fail = ctx.fetch_term();
    let value = ctx.fetch_and_load(&mut curr_p.heap);
    (fail, value)
  }

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail, value) = Self::fetch_args(ctx, curr_p);
    assert!(fail.is_cp() || fail == LTerm::nil());

    if value == LTerm::nil() && !value.is_cons() && fail != LTerm::nil() {
      // jump to fail label
      ctx.jump(fail)
    }

    Ok(DispatchResult::Normal)
  }
}

/// Check whether the value `value` is an empty list, jump to the `fail` label
/// if it is not NIL.
/// Structure: is_nil(fail:CP, value:src)
pub struct OpcodeIsNil {}

impl OpcodeIsNil {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (LTerm, LTerm) {
    let fail = ctx.fetch_term();
    let value = ctx.fetch_and_load(&mut curr_p.heap);
    (fail, value)
  }

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail, value) = Self::fetch_args(ctx, curr_p);
    assert!(fail.is_cp() || fail == LTerm::nil());

    if value != LTerm::nil() && fail != LTerm::nil() {
      // jump to fail label
      ctx.jump(fail)
    }

    Ok(DispatchResult::Normal)
  }
}

/// Take a list `value` and split it into a head and tail, they are stored in
/// `hd` and `tl` destinations respectively.
/// Structure: get_list(value:src, hd:dst, tl:dst)
pub struct OpcodeGetList {}

impl OpcodeGetList {
  pub const ARITY: usize = 3;

  #[inline]
  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (LTerm, LTerm, LTerm) {
    let src = ctx.fetch_and_load(&mut curr_p.heap);
    let dst_hd = ctx.fetch_term();
    let dst_tl = ctx.fetch_term();
    (src, dst_hd, dst_tl)
  }

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (src, dst_hd, dst_tl) = Self::fetch_args(ctx, curr_p);

    if src == LTerm::nil() {
      // TODO: is this badmatch here?
      panic!("Attempt to get_list on a nil[]");
    }
    assert!(
      src.is_cons(),
      "{}get_list: expected a cons, got: {}",
      module(),
      src
    );

    let hp = &mut curr_p.heap;

    unsafe {
      let cons_p = src.get_cons_ptr();
      ctx.store_value((*cons_p).hd(), dst_hd, hp)?;
      ctx.store_value((*cons_p).tl(), dst_tl, hp)?;
    }

    Ok(DispatchResult::Normal)
  }
}

/// Given head and tail sources, `hd` and `tl`, read them and compose into a
/// new list cell which is stored into `dst`.
/// Structure: put_list(hd:src, tl:src, dst:dst)
pub struct OpcodePutList {}

impl OpcodePutList {
  pub const ARITY: usize = 3;

  #[inline]
  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (LTerm, LTerm, LTerm) {
    let src_hd = ctx.fetch_and_load(&mut curr_p.heap);
    let src_tl = ctx.fetch_and_load(&mut curr_p.heap);
    let dst = ctx.fetch_term();
    (src_hd, src_tl, dst)
  }

  #[inline]
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (src_hd, src_tl, dst) = Self::fetch_args(ctx, curr_p);

    let hp = &mut curr_p.heap;

    unsafe {
      let cons_p = allocate_cons(hp).unwrap();
      (*cons_p).set_hd(src_hd);
      (*cons_p).set_tl(src_tl);
      ctx.store_value(LTerm::make_cons(cons_p), dst, hp)?;
    }

    Ok(DispatchResult::Normal)
  }
}
