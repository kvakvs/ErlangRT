use crate::term::value::Term;

/// A cons is 2 values stored together on heap forming a singly-linked list node.
/// Each is a fully tagged term so anyone who is parsing the heap will see this
/// as two independent values.
pub struct Cons {
  value: [Term; 2],
}

impl Cons {
  #[inline]
  pub unsafe fn hd(&self) -> Term {
    self.value[0]
  }

  #[inline]
  pub unsafe fn tl(&self) -> Term {
    self.value[1]
  }

  #[inline]
  pub unsafe fn set_hd(&mut self, val: Term) {
    self.value[0] = val
  }

  #[inline]
  pub unsafe fn set_tl(&mut self, val: Term) {
    self.value[1] = val
  }
}
