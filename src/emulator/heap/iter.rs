//! Define `HeapIterator` which can step over the heap
use emulator::heap::*;
use term::lterm::LTerm;
use term::primary;


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

    // Peek inside *p to see if we're at a header, and if so - step over it
    // using header arity. Otherwise step by 1 cell
    let val = unsafe { *p };
    let mut size = if primary::get_tag(val) == primary::TAG_HEADER {
      primary::header::get_arity(val) as isize
    } else {
      1isize
    };

    let next_p = unsafe { DataPtr::Ptr(p.offset(size)) };

    let end = self.end;
    if next_p >= end {
      return None
    }

    self.p = next_p;
    unsafe { Some(self.p) }
  }
}
