//! Module implements opcodes related to reading, writing, and moving data.

use beam::gen_op;
use beam::opcodes::assert_arity;
use rt_defs::{DispatchResult};
use emulator::process::Process;
use emulator::runtime_ctx::Context;
use emulator::function::FunEntry;
//use term::lterm::aspect_boxed::BoxedAspect;
use term::lterm::aspect_cp::CpAspect;
use term::raw::ho_closure::HOClosure;


/// Load a value from `src` and store it into `dst`. Source can be any literal
/// term, a register or a stack cell. Destination can be any register or a
/// stack cell.
#[inline]
pub fn opcode_move(ctx: &mut Context,
                   curr_p: &mut Process) -> DispatchResult {
  // Structure: move(src:src, dst:dst)
  // TODO: Optimize this by having specialized move instructions with packed arg
  assert_arity(gen_op::OPCODE_MOVE, 2);

  let src = ctx.fetch_term();
  let dst = ctx.fetch_term();
  ctx.store(src, dst, &mut curr_p.heap);
  DispatchResult::Normal
}


#[inline]
pub fn opcode_make_fun2(ctx: &mut Context,
                        curr_p: &mut Process) -> DispatchResult {
  // Structure: make_fun2(lambda_index)
  assert_arity(gen_op::OPCODE_MAKE_FUN2, 1);

  let fe_box = ctx.fetch_term();
  let fe = fe_box.cp_get_ptr() as *const FunEntry;
  //panic!("boom");
  let hp = &mut curr_p.heap;
  let closure = unsafe {
    let nfree = (*fe).nfree as usize;
    let p = HOClosure::place_into(hp,
                                  fe.as_ref().unwrap(),
                                  &ctx.regs[0..nfree]);
    p.unwrap()
  };
  ctx.regs[0] = closure;

  DispatchResult::Normal
}
