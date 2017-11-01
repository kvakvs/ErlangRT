//! Module implements opcodes related to calling built-in functions (BIF).

//use std::ptr;

//use bif::BifFn;
use beam::gen_op;
use beam::opcodes::assert_arity;
use defs::{DispatchResult};
use emulator::heap::ho_import::HOImport;
use emulator::process::Process;
use emulator::runtime_ctx::{Context, call_bif};
use term::lterm::LTerm;


/// Call a bif m:f/0 using `import` stored on heap, there is no way it can fail,
/// so also there is no fail label. Result is stored into `dst`.
#[inline]
pub fn opcode_bif0(ctx: &mut Context,
                   curr_p: &mut Process) -> DispatchResult {
  // Structure: bif0(import:boxed, dst:dst)
  assert_arity(gen_op::OPCODE_BIF0, 2);

  // HOImport object on heap which contains m:f/arity
  let import = HOImport::from_term(ctx.fetch_term());
  let dst = ctx.fetch_term();
  let bif_fn = unsafe { (*import ).resolve_bif() };
  call_bif(ctx,
           curr_p,
           bif_fn,
           LTerm::nil(),
           0,
           dst,
           false);

  DispatchResult::Normal
}
