//! Define `HeapIterator` which can step over the heap
use emulator::heap::*;
use term::lterm::LTerm;


pub struct HeapIterator {
  p: DataPtr,
  end: DataPtr,
}


impl HeapIterator {
  pub fn new(begin: DataPtr, end: DataPtr) -> HeapIterator {
    HeapIterator { p: begin, end }
  }


  /// Read current value at the iterator location.
  pub unsafe fn read_term(&self) -> LTerm {
    let DataPtr::Ptr(p) = self.p;
    LTerm::from_raw(*p)
  }
}


impl Iterator for HeapIterator {
  type Item = DataPtr;

  fn next(&mut self) -> Option<Self::Item> {
    let DataPtr::Ptr(p) = self.p;
    let next_p = unsafe { DataPtr::Ptr(p.offset(1)) };

    let end = self.end;
    if next_p >= end {
      return None
    }

    self.p = next_p;
    unsafe { Some(self.p) }
  }
}
