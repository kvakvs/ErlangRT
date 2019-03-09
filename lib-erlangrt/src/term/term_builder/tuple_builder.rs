use crate::{
  emulator::heap::Heap,
  fail::RtResult,
  term::{boxed, lterm::*},
};

/// Helper allows allocating a tuple and setting its elements.
pub struct TupleBuilder {
  tuple_ptr: *mut boxed::Tuple,
}

impl TupleBuilder {
  #[inline]
  pub fn with_arity(arity: usize, hp: &mut Heap) -> RtResult<Self> {
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

// Must import TupleBuilder for this helper to appear
impl Heap {
  /// Create a 2-tuple.
  #[inline]
  pub fn tuple2(&mut self, a: Term, b: Term) -> RtResult<Term> {
    let tb = TupleBuilder::with_arity(2, self)?;
    unsafe {
      tb.set_element(0, a);
      tb.set_element(1, b);
    }
    Ok(tb.make_term())
  }
}
