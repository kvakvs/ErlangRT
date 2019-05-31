//use super::Term;
use crate::defs::Word;
use crate::term::{Term, boxed};


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

  #[allow(dead_code)]
  pub fn restart(&mut self) {
    self.position = self.start;
  }

  pub fn next(&mut self) -> *mut Word {
    unsafe {
      let val = Term::from_raw(*self.position);
      if val.is_header_word() {
        let sz = boxed::BoxHeader::headerword_to_storage_size(val.raw());
        self.position = self.position.add(sz.words);
      } else {
        self.position = self.position.add(1);
      }
    }
    self.position
  }
}
