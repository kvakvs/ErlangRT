//! Module implements opcodes related to calling built-in functions (BIF).

use beam::gen_op;
use beam::opcodes::assert_arity;
use rt_defs::{DispatchResult};
use emulator::process::Process;
use emulator::runtime_ctx::{Context, call_bif};


/// Call a bif m:f/0 using `import` stored on heap, there is no way it can fail,
/// so also there is no fail label. Result is stored into `dst`.
#[inline]
pub fn opcode_bif0(ctx: &mut Context,
                   curr_p: &mut Process) -> DispatchResult {
  // Structure: bif0(import:boxed, dst:dst)
  assert_arity(gen_op::OPCODE_BIF0, 2);
  call_bif(ctx, curr_p, 0, false)
}


#[inline]
pub fn opcode_bif1(ctx: &mut Context,
                   curr_p: &mut Process) -> DispatchResult {
  // Structure: bif1(fail:cp, import:boxed, arg1:lterm, dst:dst)
  assert_arity(gen_op::OPCODE_BIF1, 4);
  call_bif(ctx, curr_p, 1, false)
}


#[inline]
pub fn opcode_bif2(ctx: &mut Context,
                   curr_p: &mut Process) -> DispatchResult {
  // Structure: bif1(fail:cp, import:boxed, arg1..2:lterm, dst:dst)
  assert_arity(gen_op::OPCODE_BIF2, 5);
  call_bif(ctx, curr_p, 2, false)
}


#[inline]
pub fn opcode_gc_bif1(ctx: &mut Context,
                   curr_p: &mut Process) -> DispatchResult {
  // Structure: gc_bif1(fail:cp, live:small, import:boxed, arg1:lterm, dst:dst)
  assert_arity(gen_op::OPCODE_GC_BIF1, 5);
  call_bif(ctx, curr_p, 1, true)
}


#[inline]
pub fn opcode_gc_bif2(ctx: &mut Context,
                      curr_p: &mut Process) -> DispatchResult {
  // Structure: gc_bif2(fail:CP, live:small, import:boxed, arg1:lterm,
  //                    arg2:lterm, dst:dst)
  assert_arity(gen_op::OPCODE_GC_BIF2, 6);
  call_bif(ctx, curr_p, 2, true)
}


#[inline]
pub fn opcode_gc_bif3(ctx: &mut Context,
                      curr_p: &mut Process) -> DispatchResult {
  // Structure: gc_bif3(fail:CP, live:small, import:boxed, arg1:lterm,
  //                    arg2:lterm, arg3:lterm, dst:dst)
  assert_arity(gen_op::OPCODE_GC_BIF2, 7);
  call_bif(ctx, curr_p, 3, true)
}
