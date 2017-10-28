use defs::Word;
use emulator::heap::iter;
use fail::{Error, Hopefully};
use term::lterm::LTerm;
use term::raw::rtuple;
use term::raw::{ConsPtrMut, TuplePtrMut};

//use num;
use std::fmt;


#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum DataPtr { Ptr(*const Word) }

//#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
//pub enum DataPtrMut { Ptr(*mut Word) }


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
    write!(f, "Heap{{ cap: {}, used: {} }}", self.capacity(), self.htop())
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


  /// How many words do we have before it will require GC/growth.
  pub fn capacity(&self) -> usize {
    self.data.capacity()
  }


  /// How many words are used.
  pub fn htop(&self) -> usize {
    self.htop
  }


  // This is used by heap walkers such as "dump.rs"
  #[allow(dead_code)]
  pub fn begin(&self) -> *const Word {
    &self.data[0] as *const Word
  }


  #[allow(dead_code)]
  pub fn begin_mut(&mut self) -> *mut Word {
    &mut self.data[0] as *mut Word
  }


  // This is used by heap walkers such as "dump.rs"
  #[allow(dead_code)]
  pub unsafe fn end(&self) -> *const Word {
    let p = self.begin();
    p.offset(self.htop as isize)
  }


  /// Expand heap to host `n` words of data
  pub fn allocate(&mut self, n: Word, init_nil: bool) -> Hopefully<*mut Word> {
    let pos = self.htop;
    // Explicitly forbid expanding without a GC, fail if capacity is exceeded
    if pos + n >= self.stop {
      return Err(Error::HeapIsFull);
    }

    // Assume we can grow the data without reallocating
    let raw_nil = LTerm::nil().raw();
    let new_chunk = unsafe {
      self.begin_mut().offset(self.htop as isize)
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


  /// Allocate 2 cells `[Head | Tail]` of raw cons cell, and return the pointer.
  pub fn allocate_cons(&mut self) -> Hopefully<ConsPtrMut> {
    match self.allocate(2, false) {
      Ok(p) => Ok(ConsPtrMut::from_pointer(p)),
      Err(e) => Err(e) // repack inner Err into outer Err
    }
  }


  /// Allocate `size+1` cells and form a tuple in memory, return the pointer.
  pub fn allocate_tuple(&mut self, size: Word) -> Hopefully<TuplePtrMut> {
    match self.allocate(rtuple::storage_size(size), false) {
      Ok(p) => unsafe { Ok(TuplePtrMut::create_at(p, size)) },
      Err(e) => Err(e) // repack inner Err into outer Err
    }
  }


  //  /// Allocate words on heap enough to store bignum digits and copy the given
  //  /// bignum to memory, return the pointer.
  //  pub fn allocate_big(&mut self, big: &num::BigInt) -> Hopefully<BignumPtr> {
  //    match self.allocate(BignumPtr::storage_size(big)) {
  //      Ok(p) => unsafe { Ok(BignumPtr::create_at(p, big)) },
  //      Err(e) => Err(e) // repack inner Err into outer Err
  //    }
  //  }


  /// Create a constant iterator for walking the heap.
  // This is used by heap walkers such as "dump.rs"
  #[allow(dead_code)]
  pub unsafe fn iter(&self) -> iter::HeapIterator {
    let last = self.htop as isize;
    let begin = self.begin() as *const Word;
    iter::HeapIterator::new(DataPtr::Ptr(begin),
                            DataPtr::Ptr(begin.offset(last)))
  }


  //
  // Stack Operations
  //

  #[inline]
  pub fn have(&self, need: Word) -> bool {
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
  pub fn stack_alloc_unchecked(&mut self, need: Word) {
    self.stop -= need;

    // Clear the new cells
    let raw_nil = LTerm::nil().raw();
    unsafe {
      let p = self.begin_mut().offset(self.stop as isize);
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


  /// Push a value to stack without checking. Call `stack_have(1)` beforehand.
  pub fn stack_push_unchecked(&mut self, val: Word) {
    self.stop -= 1;
    unsafe {
      let p = self.begin_mut().offset(self.stop as isize);
      *p = val
    }
  }


  pub fn stack_set_y(&mut self, index: Word, val: LTerm) -> Hopefully<()> {
    if self.send - self.stop > index + 1 {
      return Err(Error::StackIndexRange);
    }
    let pos = index as isize + self.stop as isize + 1;
    unsafe {
      let p = self.begin_mut().offset(pos);
      *p = val.raw()
    }
    Ok(())
  }


  pub fn stack_get_y(&self, index: Word) -> Hopefully<LTerm> {
    if self.send - self.stop > index + 1 {
      return Err(Error::StackIndexRange);
    }
    let pos = index as isize + self.stop as isize + 1;
    unsafe {
      let p = self.begin().offset(pos);
      Ok(LTerm::from_raw(*p))
    }
  }


  pub fn stack_depth(&self) -> Word { self.send - self.stop }


  /// Take `cp` from stack top and deallocate `n+1` words of stack.
  pub fn stack_deallocate(&mut self, n: Word) -> LTerm {
    assert!(self.stop + n + 1 <= self.send,
            "Failed to dealloc {}+1 words (s_top {}, s_end {})",
            n, self.stop, self.send);
    let cp = LTerm::from_raw(self.data[self.stop]);
    assert!(cp.is_cp());
    self.stop += n + 1;
    cp
  }
}

