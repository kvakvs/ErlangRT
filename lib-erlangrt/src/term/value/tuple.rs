// use crate::{
//  emulator::heap::Heap,
//  fail::RtResult,
//  term::{value::lterm_impl::Term, term_builder::TupleBuilder},
//};

// NOTE: Use Heap::tuple2 extension in TupleBuilder

///// Create a 2-tuple `{badmatch, Arg}`.
// pub fn make_tuple2(elem1: Term, elem2: Term, hp: &mut Heap) -> RtResult<Term> {
//  let tb = TupleBuilder::with_arity(2, hp)?;
//  unsafe {
//    tb.set_element_base0(0, elem1);
//    tb.set_element_base0(1, elem2);
//  }
//  Ok(tb.make_term())
//}
