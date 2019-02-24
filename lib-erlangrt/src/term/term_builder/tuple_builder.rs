use crate::{
  emulator::heap::Heap,
  fail::RtResult,
  term::{boxed, lterm::*},
};

/// Helper allows allocating a tuple and setting its elements.
pub struct TupleBuilder {
  p: *mut boxed::Tuple,
}

impl TupleBuilder {
  #[inline]
  pub fn with_arity(arity: usize, hp: &mut Heap) -> RtResult<Self> {
    let p = boxed::Tuple::create_into(hp, arity)?;
    Ok(Self::new(p))
  }

  #[inline]
  pub fn new(p: *mut boxed::Tuple) -> Self {
    Self { p }
  }

  #[inline]
  pub unsafe fn set_element_base0(&self, i: usize, val: LTerm) {
    boxed::Tuple::set_element_base0(self.p, i, val)
  }

  #[inline]
  pub fn make_term(&self) -> LTerm {
    LTerm::make_boxed(self.p)
  }
}

// Must import TupleBuilder for this helper to appear
impl Heap {
  /// Create a 2-tuple.
  #[inline]
  pub fn tuple2(&mut self, a: LTerm, b: LTerm) -> RtResult<LTerm> {
    let tb = TupleBuilder::with_arity(2, self)?;
    unsafe {
      tb.set_element_base0(0, a);
      tb.set_element_base0(1, b);
    }
    Ok(tb.make_term())
  }
}
