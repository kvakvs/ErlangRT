use emulator::heap::iter;
//use rt_defs::heap::iter::IHeapIterator;
use rt_defs::heap::ptr::DataPtr;
use rt_defs::heap::{IHeap, HeapError};
use rt_defs::stack::IStack;
use rt_defs::Word;
use term::lterm::*;
use term::raw::*;

use std::fmt;


/// A heap structure which grows upwards with allocations. Cannot expand
/// implicitly and will return error when capacity is exceeded. Organize a
/// garbage collect call to get more memory TODO: gc on heap
pub struct Heap {
  data: Vec<Word>,
  /// Heap top, begins at 0 and grows up towards the stack top `stop`.
  htop: Word,
  /// Stack top, begins at the end of capacity and grows down.
  stop: Word,
  /// Stack end, marks end of heap also.
  send: Word,
}


impl fmt::Debug for Heap {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Heap{{ cap: {}, used: {} }}", self.heap_capacity(), self.htop())
  }
}


impl Heap {
  pub fn new(capacity: Word) -> Heap {
    assert!(capacity > 0);
    let mut h = Heap {
      data: Vec::with_capacity(capacity),
      htop: 0,
      stop: capacity,
      send: capacity,
    };
    unsafe { h.data.set_len(capacity) };
    h
  }
}


impl IHeap for Heap {
  /// How many words do we have before it will require GC/growth.
  fn heap_capacity(&self) -> usize {
    self.data.capacity()
  }


  /// How many words are used.
  fn htop(&self) -> usize {
    self.htop
  }


  /// This is used by heap walkers such as "dump.rs"
  fn heap_begin(&self) -> *const Word {
    &self.data[0] as *const Word
  }


  fn heap_begin_mut(&mut self) -> *mut Word {
    &mut self.data[0] as *mut Word
  }


  /// This is used by heap walkers such as "dump.rs"
  unsafe fn heap_end(&self) -> *const Word {
    let p = self.heap_begin();
    p.offset(self.htop as isize)
  }


  /// Expand heap to host `n` words of data
  fn heap_allocate(&mut self, n: Word, init_nil: bool)
    -> Result<*mut Word, HeapError>
  {
    let pos = self.htop;
    // Explicitly forbid expanding without a GC, fail if capacity is exceeded
    if pos + n >= self.stop {
      return Err(HeapError::HeapIsFull);
    }

    // Assume we can grow the data without reallocating
    let raw_nil = nil().raw();
    let new_chunk = unsafe {
      self.heap_begin_mut().offset(self.htop as isize)
    };

    if init_nil {
      unsafe {
        for i in 0..n {
          *new_chunk.offset(i as isize) = raw_nil
        }
      }
    }

    self.htop += n;

    Ok(new_chunk)
  }


  //  /// Allocate words on heap enough to store bignum digits and copy the given
  //  /// bignum to memory, return the pointer.
  //  pub fn allocate_big(&mut self, big: &num::BigInt) -> Hopefully<BignumPtr> {
  //    match self.allocate(BignumPtr::storage_size(big)) {
  //      Ok(p) => unsafe { Ok(BignumPtr::create_at(p, big)) },
  //      Err(e) => Err(e) // repack inner Err into outer Err
  //    }
  //  }


  #[inline]
  fn heap_have(&self, need: Word) -> bool {
    self.htop + need <= self.stop
  }
}


/// Create a constant iterator for walking the heap.
/// This is used by heap walkers such as "dump.rs"
pub unsafe fn heap_iter(hp: &Heap) -> iter::HeapIterator {
  let last = hp.htop as isize;
  let begin = hp.heap_begin() as *const Word;
  iter::HeapIterator::new(DataPtr(begin),
                          DataPtr(begin.offset(last)))
}


impl IStack<LTerm> for Heap {
  #[inline]
  fn stack_have(&self, need: Word) -> bool {
    self.htop + need <= self.stop
  }


  //  pub fn stack_alloc(&mut self, need: Word) -> Hopefully<()> {
  //    // Check if heap top is too close to stack top, then fail
  //    if !self.stack_have(need) {
  //      return Err(Error::HeapIsFull)
  //    }
  //    self.stack_alloc_unchecked(need);
  //    Ok(())
  //  }


  /// Allocate stack cells without checking. Call `stack_have(n)` beforehand.
  fn stack_alloc_unchecked(&mut self, need: Word) {
    self.stop -= need;

    // Clear the new cells
    let raw_nil = nil().raw();
    unsafe {
      let p = self.heap_begin_mut().offset(self.stop as isize);
      for y in 0..need {
        *p.offset(y as isize) = raw_nil
      }
    }
  }


  // TODO: Add unsafe push without range checks (batch check+multiple push)
  //  pub fn stack_push(&mut self, val: Word) -> Hopefully<()> {
  //    if !self.stack_have(1) {
  //      return Err(Error::HeapIsFull)
  //    }
  //    self.stack_push_unchecked(val);
  //    Ok(())
  //  }


  //#[allow(dead_code)]
  fn stack_info(&self) {
    println!("Stack (s_top {}, s_end {})",
             self.stop, self.send)
  }


  /// Push a value to stack without checking. Call `stack_have(1)` beforehand.
  fn stack_push_unchecked(&mut self, val: Word) {
    self.stop -= 1;
    self.data[self.stop] = val;
  }


  /// Check whether `y+1`-th element can be found in stack
  #[inline]
  fn stack_have_y(&self, y: Word) -> bool {
    self.send - self.stop >= y + 1
  }


  fn stack_set_y(&mut self, index: Word, val: LTerm) -> Result<(), HeapError> {
    if !self.stack_have_y(index) {
      return Err(HeapError::StackIndexRange);
    }
    self.data[index + self.stop + 1] = val.raw();
    Ok(())
  }


  fn stack_get_y(&self, index: Word) -> Result<LTerm, HeapError> {
    if !self.stack_have_y(index) {
      return Err(HeapError::StackIndexRange);
    }
    let pos = index + self.stop + 1;
    Ok(LTerm::from_raw(self.data[pos]))
  }


  fn stack_depth(&self) -> Word { self.send - self.stop }


  /// Take `cp` from stack top and deallocate `n+1` words of stack.
  fn stack_deallocate(&mut self, n: Word) -> LTerm {
    assert!(self.stop + n + 1 <= self.send,
            "Failed to dealloc {}+1 words (s_top {}, s_end {})",
            n, self.stop, self.send);
    let cp = LTerm::from_raw(self.data[self.stop]);
    assert!(cp.is_cp());
    self.stop += n + 1;
    cp
  }
}


/// Allocate 2 cells `[Head | Tail]` of raw cons cell, and return the pointer.
pub fn allocate_cons(hp: &mut Heap)
  -> Result<rcons::PtrMut, HeapError>
{
  match hp.heap_allocate(2, false) {
    Ok(p) => Ok(rcons::PtrMut::from_pointer(p)),
    Err(e) => Err(e) // repack inner Err into outer Err
  }
}


/// Allocate `size+1` cells and form a tuple in memory, return the pointer.
pub fn allocate_tuple(hp: &mut Heap, size: Word)
  -> Result<rtuple::PtrMut, HeapError>
{
  match hp.heap_allocate(rtuple::storage_size(size), false) {
    Ok(p) => unsafe { Ok(rtuple::PtrMut::create_at(p, size)) },
    Err(e) => Err(e) // repack inner Err into outer Err
  }
}
