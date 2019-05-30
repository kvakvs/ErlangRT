//use super::Term;
use crate::defs::Word;
use crate::term::Term;

/// Walks a linear heap forward from start to the end, jumping over the objects
pub struct HeapWalker {
  start: *mut Word,
  stop: *mut Word,
  position: *mut Word,
}

impl HeapWalker {
  pub fn new(start: *mut Word, stop: *mut Word) -> Self {
    Self {
      start,
      stop,
      position: start,
    }
  }

  pub fn restart(&mut self) {
    self.position = self.start;
  }

  pub fn next(&mut self) -> *mut Word {
    unsafe {
      let val = Term::from_raw(*self.position);
      self.position = self.position.add(1);
    }
    self.position
  }
}
