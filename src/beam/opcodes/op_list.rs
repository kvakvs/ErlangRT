//! Module implements opcodes related to lists manipulation.

//use term::raw::rcons::ConsPtr;
use beam::gen_op;
use beam::opcodes::assert_arity;
use rt_defs::DispatchResult;
use emulator::code::CodePtr;
use emulator::process::Process;
use emulator::runtime_ctx::Context;
use term::lterm::*;


fn module() -> &'static str { "opcodes::op_list: " }


/// Read the source `value` and check whether it is a list and not NIL. On
/// false jump to the label `fail`.
#[inline]
pub fn opcode_is_nonempty_list(ctx: &mut Context,
                               curr_p: &mut Process) -> DispatchResult {
  // Structure: is_nonempty_list(fail:cp, value:src)
  assert_arity(gen_op::OPCODE_IS_NONEMPTY_LIST, 2);

  let fail = ctx.fetch_term(); // jump if not a list

  assert!(fail.is_cp() || fail.is_nil());

  let list = ctx.fetch_and_load(&curr_p.heap);

  if list.is_nil() && !list.is_cons() {
    if !fail.is_nil() {
      // jump to fail label
      ctx.ip = CodePtr::from_cp(fail)
    }
  }

  DispatchResult::Normal
}


/// Check whether the value `value` is an empty list, jump to the `fail` label
/// if it is not NIL.
#[inline]
pub fn opcode_is_nil(ctx: &mut Context,
                     curr_p: &mut Process) -> DispatchResult {
  // Structure: is_nil(fail:CP, value:src)
  assert_arity(gen_op::OPCODE_IS_NIL, 2);
  let fail = ctx.fetch_term(); // jump if not a list
  assert!(fail.is_cp() || fail.is_nil());

  let list = ctx.fetch_and_load(&curr_p.heap);

  if !list.is_nil() {
    if !fail.is_nil() {
      // jump to fail label
      ctx.ip = CodePtr::from_cp(fail)
    }
  }

  DispatchResult::Normal
}


/// Take a list `value` and split it into a head and tail, they are stored in
/// `hd` and `tl` destinations respectively.
#[inline]
pub fn opcode_get_list(ctx: &mut Context,
                       curr_p: &mut Process) -> DispatchResult {
  // Structure: get_list(value:src, hd:dst, tl:dst)
  assert_arity(gen_op::OPCODE_GET_LIST, 3);

  let hp = &mut curr_p.heap;
  let src = ctx.fetch_and_load(hp); // take src

  let hd = ctx.fetch_term(); // put src's head into hd
  let tl = ctx.fetch_term(); // put src's tail into tl

  if src.is_nil() {
    panic!("Attempt to get_list on a nil[]");
  }
  assert!(src.is_cons(), "{}get_list: expected a cons, got {}", module(), src);

  unsafe {
    let cons_p = src.cons_get_ptr();
    ctx.store(cons_p.hd(), hd, hp);
    ctx.store(cons_p.tl(), tl, hp);
  }

  DispatchResult::Normal
}


/// Given head and tail sources, `hd` and `tl`, read them and compose into a
/// new list cell which is stored into `dst`.
#[inline]
pub fn opcode_put_list(ctx: &mut Context,
                       curr_p: &mut Process) -> DispatchResult {
  // Structure: put_list(hd:src, tl:src, dst:dst)
  assert_arity(gen_op::OPCODE_PUT_LIST, 3);

  let hp = &mut curr_p.heap;
  let hd = ctx.fetch_and_load(hp); // take hd
  let tl = ctx.fetch_and_load(hp); // take tl
  let dst = ctx.fetch_term(); // put `[hd | tl]` into dst

  unsafe {
    let cons_p = hp.allocate_cons().unwrap();
    cons_p.set_hd(hd);
    cons_p.set_tl(tl);
    ctx.store(cons_p.make_cons(), dst, hp);
  }

  DispatchResult::Normal
}
