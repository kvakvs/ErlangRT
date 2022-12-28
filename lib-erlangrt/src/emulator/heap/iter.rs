//! Define `HeapIterator` which can step over the heap
use crate::{
  defs::WordSize,
  term::{boxed, PrimaryTag, Term},
};

// This is used by heap walkers such as "dump.rs"
#[allow(dead_code)]
pub struct HeapIterator {
  p: *const Term,
  end: *const Term,
}

impl HeapIterator {
  pub fn new(begin: *const Term, end: *const Term) -> HeapIterator {
    HeapIterator { p: begin, end }
  }

  pub unsafe fn next(&mut self) -> Option<*const Term> {
    // Peek inside *p to see if we're at a header, and if so - step over it
    // using header arity. Otherwise step by 1 cell
    let val = self.p.read();
    let size = match val.get_term_tag() {
      PrimaryTag::HEADER => boxed::BoxHeader::headerword_to_storage_size(val.raw()),
      _ => WordSize::one(),
    };

    self.p = self.p.add(size.words);

    let end = self.end;
    if self.p >= end {
      return None;
    }

    Some(self.p)
  }
}
