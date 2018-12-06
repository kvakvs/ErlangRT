use crate::term::lterm::LTerm;

/// A cons is 2 values stored together on heap forming a singly-linked list node.
/// Each is a fully tagged term so anyone who is parsing the heap will see this
/// as two independent values.
pub struct Cons {
  value: [LTerm; 2],
}

impl Cons {
  pub unsafe fn hd(&self) -> LTerm {
    self.value[0]
  }

  pub unsafe fn tl(&self) -> LTerm {
    self.value[1]
  }

  pub unsafe fn set_hd(&mut self, val: LTerm) {
    self.value[0] = val
  }

  pub unsafe fn set_tl(&mut self, val: LTerm) {
    self.value[1] = val
  }
}
