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

    // Now continue fetching opcodes if there are more `put` operations
    for i in 0..arity {
      let op = opcode::from_memory_word(ctx.fetch());
      if op != OPCODE_PUT {
        ctx.unfetch();
        break;
      }
      let val = ctx.fetch_term();
      // println!("- put {}, {}", i, val);
      unsafe { boxed::Tuple::set_element_base0(tuple_p, i, val); }
    }

    ctx.store_value(LTerm::make_boxed(tuple_p), dst, hp)?;
    Ok(DispatchResult::Normal)
  }
}


/// Checks that tuple in argument1 has arity `arity` otherwise jumps to fail.
/// Structure: test_arity(on_false:label, value:tuple, arity:int)
pub struct OpcodeTestArity {}

impl OpcodeTestArity {
  pub const ARITY: usize = 3;

  #[inline]
  fn fetch_args(ctx: &mut Context, curr_p: &mut Process) -> (LTerm, LTerm, usize) {
    let on_false = ctx.fetch_term();
    let val = ctx.fetch_and_load(&mut curr_p.heap);
    let arity = ctx.fetch_term().get_small_unsigned();
    (on_false, val, arity)
  }

  #[inline]
  pub fn run(
    _vm: &mut VM,
    ctx: &mut Context,
    curr_p: &mut Process,
  ) -> RtResult<DispatchResult> {
    let (fail_label, val, arity) = Self::fetch_args(ctx, curr_p);
    // Possibly even not a tuple
    if !val.is_tuple() {
      ctx.jump(fail_label)
    }
    else {
      // Get tuple arity and check it
      let tuple_p = val.get_tuple_ptr();
      if unsafe { (*tuple_p).get_arity() } != arity {
        ctx.jump(fail_label)
      }
    }
    Ok(DispatchResult::Normal)
  }
}
