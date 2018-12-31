use crate::{
  emulator::heap::Heap,
  fail::RtResult,
  term::{lterm::lterm_impl::LTerm, term_builder::TupleBuilder},
};

/// Create a 2-tuple `{badmatch, Arg}`.
pub fn make_tuple2(elem1: LTerm, elem2: LTerm, hp: &mut Heap) -> RtResult<LTerm> {
  let tb = TupleBuilder::with_arity(2, hp)?;
  unsafe {
    tb.set_element_base0(0, elem1);
    tb.set_element_base0(1, elem2);
  }
  Ok(tb.make_term())
}
