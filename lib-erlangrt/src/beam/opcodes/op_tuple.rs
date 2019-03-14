//! Module implements opcodes related to tuple creation and manipulation.
use crate::{
  beam::{disp_result::DispatchResult, gen_op::OPCODE_PUT},
  emulator::{code::opcode, process::Process, runtime_ctx::Context},
  fail::{self, RtResult},
  term::{boxed, value::Term},
};

// fn module() -> &'static str {
//  "opcodes::op_tuple: "
//}

// Creates an empty tuple of `arity` and places the pointer to it into `dst`.
// Followed by multiple `put` instructions which will set tuple elements.
// Structure: put_tuple(arity:smallint, dst)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodePutTuple, arity: 2,
  run: { Self::put_tuple(ctx, curr_p, arity, dst) },
  args: usize(arity), term(dst),
);

impl OpcodePutTuple {
  #[inline]
  pub fn put_tuple(
    ctx: &mut Context,
    curr_p: &mut Process,
    arity: usize,
    dst: Term,
  ) -> RtResult<DispatchResult> {
    let hp = &mut curr_p.heap;
    let tuple_p = boxed::Tuple::create_into(hp, arity)?;

    // Now continue fetching opcodes if there are more `put` operations
    for i in 0..arity {
      let op = opcode::from_memory_word(ctx.ip_read());
      if op != OPCODE_PUT {
        panic!("put_tuple must be followed by N put opcodes");
      }
      let val = ctx.op_arg_load_term_at(0, hp);
      ctx.ip_advance(2);

      // println!("- put {}, {}", i, val);
      unsafe {
        (*tuple_p).set_element(i, val);
      }
    }

    ctx.store_value(Term::make_boxed(tuple_p), dst, hp)?;
    Ok(DispatchResult::Normal)
  }
}


// Checks that tuple in argument1 has arity `arity` otherwise jumps to fail.
// Structure: test_arity(on_false:label, value:tuple, arity:int)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeTestArity, arity: 3,
  run: { Self::test_arity(ctx, fail, value, arity) },
  args: cp_or_nil(fail), load(value), usize(arity),
);

impl OpcodeTestArity {
  #[inline]
  pub fn test_arity(
    ctx: &mut Context,
    fail: Term,
    value: Term,
    arity: usize,
  ) -> RtResult<DispatchResult> {
    // Possibly even not a tuple
    if !value.is_tuple() {
      ctx.jump(fail)
    } else {
      // Get tuple arity and check it
      let tuple_p = value.get_tuple_ptr();
      if unsafe { (*tuple_p).get_arity() } != arity {
        ctx.jump(fail)
      }
    }
    Ok(DispatchResult::Normal)
  }
}


// From `src` get `index`th element and store it in `dst`.
// Structure: get_tuple_element(src:src, index:smallint, dst:dst)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeGetTupleElement, arity: 3,
  run: { Self::get_tuple_element(ctx, curr_p, src, index, dst) },
  args: load(src), usize(index), term(dst),
);

impl OpcodeGetTupleElement {
  #[inline]
  pub fn get_tuple_element(
    ctx: &mut Context,
    curr_p: &mut Process,
    src: Term,
    index: usize,
    dst: Term,
  ) -> RtResult<DispatchResult> {
    let tuple_p = src.get_tuple_ptr();
    let element = unsafe { (*tuple_p).get_element(index) };
    ctx.store_value(element, dst, &mut curr_p.heap)?;
    Ok(DispatchResult::Normal)
  }
}


// From `src` get `index`th element and store it in `dst`.
// Structure: set_tuple_element(val:src, dst:dst, index:smallint)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeSetTupleElement, arity: 3,
  run: { Self::set_tuple_element(val, dst, index) },
  args: load(val), load(dst), usize(index),
);

impl OpcodeSetTupleElement {
  #[inline]
  pub fn set_tuple_element(
    value: Term,
    dst: Term,
    index: usize,
  ) -> RtResult<DispatchResult> {
    if !dst.is_tuple() {
      return fail::create::badarg();
    }
    let tuple_p = dst.get_tuple_ptr_mut();
    unsafe { (*tuple_p).set_element(index, value) };
    Ok(DispatchResult::Normal)
  }
}


// Test the type of Value, and jump to label if it is not a tuple.
// Test the arity of tuple and jump to label if it is not Arity.
// Test the first element of the tuple and jump to label if it is not atom.
// Structure: is_tagged_tuple(label:cp, value, arity:smallint, atom:atom)
define_opcode!(_vm, ctx, curr_p,
  name: OpcodeIsTaggedTuple, arity: 4,
  run: { Self::is_tagged_tuple(ctx, label, value, arity, atom) },
  args: cp_or_nil(label), load(value), usize(arity), term(atom),
);

impl OpcodeIsTaggedTuple {
  #[inline]
  pub fn is_tagged_tuple(
    ctx: &mut Context,
    label: Term,
    value: Term,
    arity: usize,
    atom: Term,
  ) -> RtResult<DispatchResult> {
    if !value.is_tuple() {
      ctx.jump(label);
    } else {
      let tuple_p = value.get_tuple_ptr();
      if unsafe { (*tuple_p).get_arity() } != arity {
        ctx.jump(label);
      } else {
        debug_assert!(atom.is_atom());
        let first = unsafe { (*tuple_p).get_element(0) };
        // assuming atom parameter is an atom, we can use direct comparison
        // instead of calling compare::cmp_terms/3
        if first != atom {
          ctx.jump(label);
        }
      }
    }
    Ok(DispatchResult::Normal)
  }
}
