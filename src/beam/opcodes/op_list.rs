//! Module implements opcodes related to lists manipulation.

//use term::raw::rcons::ConsPtr;
use beam::gen_op;
use beam::opcodes::assert_arity;
use defs::DispatchResult;
use emulator::code::CodePtr;
use emulator::heap::Heap;
use emulator::runtime_ctx::Context;


fn module() -> &'static str { "opcodes::op_list: " }


#[inline]
pub fn opcode_is_nonempty_list(ctx: &mut Context,
                               hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_IS_NONEMPTY_LIST, 2);
  let fail = ctx.fetch_term(); // jump if not a list

  assert!(fail.is_cp() || fail.is_nil());

  let list = ctx.fetch_and_load(hp);

  if list.is_nil() && !list.is_cons() {
    if !fail.is_nil() {
      // jump to fail label
      ctx.ip = CodePtr::from_cp(fail)
    }
  }

  DispatchResult::Normal
}


#[inline]
pub fn opcode_is_nil(ctx: &mut Context, hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_IS_NIL, 2);
  let fail = ctx.fetch_term(); // jump if not a list
  assert!(fail.is_cp() || fail.is_nil());

  let list = ctx.fetch_and_load(hp);

  if !list.is_nil() {
    if !fail.is_nil() {
      // jump to fail label
      ctx.ip = CodePtr::from_cp(fail)
    }
  }

  DispatchResult::Normal
}


#[inline]
pub fn opcode_get_list(ctx: &mut Context, hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_GET_LIST, 3);
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


#[inline]
pub fn opcode_put_list(ctx: &mut Context, hp: &mut Heap) -> DispatchResult {
  assert_arity(gen_op::OPCODE_PUT_LIST, 3);
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