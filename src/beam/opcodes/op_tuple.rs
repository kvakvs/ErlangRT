//! Module implements opcodes related to tuple creation and manipulation.
use crate::{
  beam::{disp_result::DispatchResult, gen_op::OPCODE_PUT},
  emulator::{
    code::opcode, process::Process, runtime_ctx::Context, vm::VM,
  },
  fail::RtResult,
  term::{boxed, lterm::LTerm},
};

//fn module() -> &'static str {
//  "opcodes::op_tuple: "
//}

/// Creates an empty tuple of `arity` and places the pointer to it into `dst`.
/// Followed by multiple `put` instructions which will set tuple elements.
/// Structure: put_tuple(arity:smallint, dst)
pub struct OpcodePutTuple {}

impl OpcodePutTuple {
  pub const ARITY: usize = 2;

  #[inline]
  fn fetch_args(ctx: &mut Context) -> (usize, LTerm) {
    let arity = ctx.fetch_term().get_small_unsigned();
    let dst = ctx.fetch_term();
    (arity, dst)
  }

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (arity, dst) = Self::fetch_args(ctx);
    let hp = &mut curr_p.heap;
    let tuple_p = boxed::Tuple::create_into(hp, arity)?;

    ctx.store_value(LTerm::make_boxed(tuple_p), dst, hp)?;

    // Now continue fetching opcodes if there are more `put` operations
    for i in 0..arity {
      let op = opcode::from_memory_word(ctx.fetch());
      if op != OPCODE_PUT {
        ctx.unfetch();
        break;
      }
      let val = ctx.fetch_term();
      unsafe { boxed::Tuple::set_element_base0(tuple_p, i, val); }
    }

    Ok(DispatchResult::Normal)
  }
}
