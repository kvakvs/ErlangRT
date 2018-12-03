//! Define `HeapIterator` which can step over the heap
use emulator::heap::IHeapIterator;
use term::lterm::LTerm;
use term::boxed;


// This is used by heap walkers such as "dump.rs"
#[allow(dead_code)]
pub struct HeapIterator {
  p: *const LTerm,
  end: *const LTerm,
}


impl HeapIterator {
  pub fn new(begin: *const LTerm, end: *const LTerm) -> HeapIterator {
    HeapIterator { p: begin, end }
  }
}


impl IHeapIterator<*const LTerm> for HeapIterator {
  unsafe fn next(&mut self) -> Option<*const LTerm> {
    // Peek inside *p to see if we're at a header, and if so - step over it
    // using header arity. Otherwise step by 1 cell
    let val = *self.p;
    let size = match val.get_term_tag() {
      TERMTAG_HEADER => boxed::headerword_to_arity(val.raw()),
      _ => 1usize,
    };

    self.p.add(size);

    let end = self.end;
    if self.p >= end {
      return None
    }

    Some(self.p)
  }
}
