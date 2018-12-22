//! Module implements opcodes related to lists manipulation.

use crate::{
  beam::{disp_result::DispatchResult},
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
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let fail = ctx.fetch_term(); // jump if not a list

    assert!(fail.is_cp() || fail == LTerm::nil());

    let list = ctx.fetch_and_load(&curr_p.heap);

    if list == LTerm::nil() && !list.is_cons() && fail != LTerm::nil() {
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
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let fail = ctx.fetch_term(); // jump if not a list
    assert!(fail.is_cp() || fail == LTerm::nil());

    let list = ctx.fetch_and_load(&curr_p.heap);

    if list != LTerm::nil() && fail != LTerm::nil() {
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
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let hp = &mut curr_p.heap;
    let src = ctx.fetch_and_load(hp); // take src

    let hd = ctx.fetch_term(); // put src's head into hd
    let tl = ctx.fetch_term(); // put src's tail into tl

    if src == LTerm::nil() {
      panic!("Attempt to get_list on a nil[]");
    }
    assert!(
      src.is_cons(),
      "{}get_list: expected a cons, got {}",
      module(),
      src
    );

    unsafe {
      let cons_p = src.get_cons_ptr();
      ctx.store((*cons_p).hd(), hd, hp);
      ctx.store((*cons_p).tl(), tl, hp);
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
  pub fn run(
    _vm: &VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let hp = &mut curr_p.heap;
    let hd = ctx.fetch_and_load(hp); // take hd
    let tl = ctx.fetch_and_load(hp); // take tl
    let dst = ctx.fetch_term(); // put `[hd | tl]` into dst

    unsafe {
      let cons_p = allocate_cons(hp).unwrap();
      (*cons_p).set_hd(hd);
      (*cons_p).set_tl(tl);
      ctx.store(LTerm::make_cons(cons_p), dst, hp);
    }

    Ok(DispatchResult::Normal)
  }
}
