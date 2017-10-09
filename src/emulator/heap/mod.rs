pub mod iter;
pub mod dump;

use defs::Word;
use term::lterm::LTerm;
use term::raw::{RawConsMut, RawTupleMut, RawBignum};

use num;
use std::boxed::Box;
use std::fmt;

/// Default heap size for constants (literals) when loading a module.
pub const DEFAULT_LIT_HEAP: Word = 1024;
/// Default heap size when spawning a process.
pub const DEFAULT_PROC_HEAP: Word = 300;


#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum DataPtr { Ptr(*const Word) }

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum DataPtrMut { Ptr(*mut Word) }


/// A heap structure which grows upwards with allocations. Cannot expand
/// implicitly and will return error when capacity is exceeded. Organize a
/// garbage collect call to get more memory TODO: gc on heap
pub struct Heap {
  data: Box<Vec<Word>>,
}


impl fmt::Debug for Heap {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Heap{{ cap: {}, used: {} }}", self.capacity(), self.used())
  }
}


impl Heap {
  pub fn new(capacity: Word) -> Heap {
    Heap{
      data: Box::new(Vec::with_capacity(capacity)),
    }
  }


  /// How many words do we have before it will require GC/growth.
  pub fn capacity(&self) -> usize {
    self.data.capacity()
  }


  /// How many words are used.
  pub fn used(&self) -> usize {
    self.data.len()
  }


  fn begin(&self) -> *const Word {
    &self.data[0] as *const Word
  }


  unsafe fn end(&self) -> *const Word {
    let p = &self.data[0] as *const Word;
    p.offset(self.data.len() as isize)
  }


  /// Expand heap to host `n` words of data
  pub fn allocate(&mut self, n: Word) -> Option<*mut Word> {
    let pos = self.data.len();
    // Explicitly forbid expanding without a GC, fail if capacity is exceeded
    if pos + n >= self.data.capacity() {
      return None
    }

    // Assume we can grow the data without reallocating
    let raw_nil = LTerm::nil().raw();
    self.data.resize(pos + n, raw_nil);
//    for _i in 0..n { self.data.push(raw_nil) }

    Some(&mut self.data[pos] as *mut Word)
  }


  /// Allocate 2 cells `[Head | Tail]` of raw cons cell, and return the pointer.
  pub fn allocate_cons(&mut self) -> Option<RawConsMut> {
    match self.allocate(2) {
      Some(p) => Some(RawConsMut::from_pointer(p)),
      None => None
    }
  }


  /// Allocate `size+1` cells and form a tuple in memory, return the pointer.
  pub fn allocate_tuple(&mut self, size: Word) -> Option<RawTupleMut> {
    match self.allocate(RawTupleMut::storage_size(size)) {
      Some(p) => unsafe { Some(RawTupleMut::create_at(p, size)) },
      None => None
    }
  }


  /// Allocate words on heap enough to store bignum digits and copy the given
  /// bignum to memory, return the pointer.
  pub fn allocate_big(&mut self, big: &num::BigInt) -> Option<RawBignum> {
    match self.allocate(RawBignum::storage_size(&big)) {
      Some(p) => unsafe { Some(RawBignum::create_at(p, &big)) },
      None => None
    }
  }


  /// Create a constant iterator for walking the heap.
  pub unsafe fn iter(&self) -> iter::HeapIterator {
    let last = self.data.len() as isize;
    let begin = &self.data[0] as *const Word;
    iter::HeapIterator::new(DataPtr::Ptr(begin),
                            DataPtr::Ptr(begin.offset(last)))
  }
}

