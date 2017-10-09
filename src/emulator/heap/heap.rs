use defs::Word;
use emulator::heap::iter;
use fail::{Error, Hopefully};
use term;
use term::lterm::LTerm;
use term::raw::rtuple;
use term::raw::{RawConsMut, RawTupleMut, RawBignum};

use num;
use alloc::raw_vec::RawVec;
use std::fmt;


#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum DataPtr { Ptr(*const Word) }

//#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
//pub enum DataPtrMut { Ptr(*mut Word) }


/// A heap structure which grows upwards with allocations. Cannot expand
/// implicitly and will return error when capacity is exceeded. Organize a
/// garbage collect call to get more memory TODO: gc on heap
pub struct Heap {
  data: RawVec<Word>,
  /// Heap top, begins at 0 and grows up towards the stack top `stop`.
  htop: Word,
  /// Stack top, begins at the end of capacity and grows down.
  stop: Word,
  /// Stack end, marks end of heap also.
  send: Word,
}


impl fmt::Debug for Heap {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Heap{{ cap: {}, used: {} }}", self.capacity(), self.htop())
  }
}


impl Heap {
  pub fn new(capacity: Word) -> Heap {
    assert!(capacity > 0);
    Heap {
      data: RawVec::with_capacity(capacity),
      htop: 0,
      stop: capacity,
      send: capacity,
    }
  }


  /// How many words do we have before it will require GC/growth.
  pub fn capacity(&self) -> usize {
    self.data.cap()
  }


  /// How many words are used.
  pub fn htop(&self) -> usize {
    self.htop
  }


  pub fn begin(&self) -> *const Word {
    self.data.ptr() as *const Word
  }


  pub unsafe fn end(&self) -> *const Word {
    let p = self.data.ptr() as *const Word;
    p.offset(self.htop as isize)
  }


  /// Expand heap to host `n` words of data
  pub fn allocate(&mut self, n: Word) -> Hopefully<*mut Word> {
    let pos = self.htop;
    // Explicitly forbid expanding without a GC, fail if capacity is exceeded
    if pos + n >= self.stop {
      return Err(Error::HeapIsFull)
    }

    // Assume we can grow the data without reallocating
    let raw_nil = LTerm::nil().raw();
    //    self.data.resize(pos + n, raw_nil);
    let new_chunk = unsafe {
      self.data.ptr().offset(self.htop as isize)
    };
    unsafe {
      for i in 0..n {
        *new_chunk.offset(i as isize) = raw_nil
      }
    }
    self.htop += n;

    Ok(new_chunk)
  }


  /// Allocate 2 cells `[Head | Tail]` of raw cons cell, and return the pointer.
  pub fn allocate_cons(&mut self) -> Hopefully<RawConsMut> {
    match self.allocate(2) {
      Ok(p) => Ok(RawConsMut::from_pointer(p)),
      Err(e) => Err(e) // repack inner Err into outer Err
    }
  }


  /// Allocate `size+1` cells and form a tuple in memory, return the pointer.
  pub fn allocate_tuple(&mut self, size: Word) -> Hopefully<RawTupleMut> {
    match self.allocate(rtuple::storage_size(size)) {
      Ok(p) => unsafe { Ok(RawTupleMut::create_at(p, size)) },
      Err(e) => Err(e) // repack inner Err into outer Err
    }
  }


  /// Allocate words on heap enough to store bignum digits and copy the given
  /// bignum to memory, return the pointer.
  pub fn allocate_big(&mut self, big: &num::BigInt) -> Hopefully<RawBignum> {
    match self.allocate(RawBignum::storage_size(&big)) {
      Ok(p) => unsafe { Ok(RawBignum::create_at(p, &big)) },
      Err(e) => Err(e) // repack inner Err into outer Err
    }
  }


  /// Create a constant iterator for walking the heap.
  pub unsafe fn iter(&self) -> iter::HeapIterator {
    let last = self.htop as isize;
    let begin = self.data.ptr() as *const Word;
    iter::HeapIterator::new(DataPtr::Ptr(begin),
                            DataPtr::Ptr(begin.offset(last)))
  }

  //
  // Stack Operations
  //

  pub fn stack_alloc(&mut self, need: Word) -> Hopefully<()> {
    // Check if heap top is too close to stack top, then fail
    if self.htop + need > self.stop {
      return Err(Error::HeapIsFull)
    }
    self.stop -= need;

    // Clear the new cells
    let raw_nil = term::immediate::IMM2_SPECIAL_NIL_RAW;
    unsafe {
      let p = self.data.ptr().offset(self.stop as isize);
      for y in 0..need {
        *p.offset(y as isize) = raw_nil
      }
    }
    Ok(())
  }


  // TODO: Add unsafe push without range checks (batch check+multiple push)
  pub fn stack_push(&mut self, val: Word) -> Hopefully<()> {
    if self.htop + 1 > self.stop {
      return Err(Error::HeapIsFull)
    }
    self.stop -= 1;

    unsafe {
      let p = self.data.ptr().offset(self.stop as isize);
      *p = val
    }

    Ok(())
  }
}

