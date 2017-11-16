//! Define `HeapIterator` which can step over the heap
use rt_defs::heap::iter::IHeapIterator;
use rt_defs::heap::ptr::DataPtr;
use term::primary;


// This is used by heap walkers such as "dump.rs"
#[allow(dead_code)]
pub struct HeapIterator {
  p: DataPtr,
  end: DataPtr,
}


impl HeapIterator {
  pub fn new(begin: DataPtr, end: DataPtr) -> HeapIterator {
    HeapIterator { p: begin, end }
  }


//  /// Read current value at the iterator location.
//  pub unsafe fn read_term(&self) -> LTerm {
//    let DataPtr::Ptr(p) = self.p;
//    LTerm::from_raw(*p)
//  }
}


impl IHeapIterator<DataPtr> for HeapIterator {
  fn next(&mut self) -> Option<DataPtr> {
    let DataPtr(p) = self.p;

    // Peek inside *p to see if we're at a header, and if so - step over it
    // using header arity. Otherwise step by 1 cell
    let val = unsafe { *p };
    let size = if primary::get_tag(val) == primary::TAG_HEADER {
      primary::header::get_arity(val) as isize
    } else {
      1isize
    };

    let next_p = unsafe { DataPtr(p.offset(size)) };

    let end = self.end;
    if next_p >= end {
      return None
    }

    self.p = next_p;
    Some(self.p)
  }
}
