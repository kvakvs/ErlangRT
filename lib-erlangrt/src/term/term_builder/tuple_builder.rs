use crate::{
  emulator::heap::THeap,
  fail::RtResult,
  term::{boxed, value::*},
};

/// Helper allows allocating a tuple and setting its elements.
pub struct TupleBuilder {
  tuple_ptr: *mut boxed::Tuple,
}

impl TupleBuilder {
  #[inline]
  pub fn with_arity(arity: usize, hp: &mut THeap) -> RtResult<Self> {
    let new_tuple = boxed::Tuple::create_into(hp, arity)?;
    Ok(Self::new(new_tuple))
  }

  #[inline]
  pub fn new(p: *mut boxed::Tuple) -> Self {
    Self { tuple_ptr: p }
  }

  #[inline]
  pub unsafe fn set_element(&self, i: usize, val: Term) {
    (*self.tuple_ptr).set_element(i, val)
  }

  #[inline]
  pub fn make_term(&self) -> Term {
    Term::make_boxed(self.tuple_ptr)
  }
}

/// Create a 2-tuple.
#[inline]
pub fn tuple2(hp: &mut THeap, a: Term, b: Term) -> RtResult<Term> {
  let tb = TupleBuilder::with_arity(2, hp)?;
  unsafe {
    tb.set_element(0, a);
    tb.set_element(1, b);
  }
  Ok(tb.make_term())
}
