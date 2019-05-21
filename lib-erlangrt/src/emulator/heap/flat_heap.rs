#![allow(dead_code)]
use crate::{
  defs::{Word, WordSize},
  emulator::heap::{catch::NextCatchResult, heap_trait::THeap, iter, Designation},
  fail::{RtErr, RtResult},
  term::value::Term,
};
use colored::Colorize;
use core::fmt;

/// Default heap size for constants (literals) when loading a module.
const DEFAULT_LIT_HEAP: usize = 8192;

/// Default heap size when spawning a process. (default: 300)
const DEFAULT_PROC_HEAP: usize = 16384;
const BINARY_HEAP_CAPACITY: usize = 65536; // 64k*8 = 512kb

/// A heap structure which grows upwards with allocations. Cannot expand
/// implicitly and will return error when capacity is exceeded. Organize a
/// garbage collect call to get more memory TODO: gc on heap
pub struct FlatHeap {
  data: Vec<Word>,
  /// Heap top, begins at 0 and grows up towards the `stack_top`.
  heap_top: usize,
  /// Stack top, begins at the end of capacity and grows down.
  stack_top: usize,
  /// Marks end of the stack and also end of the heap.
  capacity: usize,
}

impl FlatHeap {}

impl fmt::Debug for FlatHeap {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "FlatHeap{{ cap: {}, used: {} }}",
      self.get_heap_max_capacity(),
      self.get_heap_used_words()
    )
  }
}

impl THeap for FlatHeap {
  fn alloc(&mut self, n: WordSize, init_nil: bool) -> RtResult<*mut Word> {
    let pos = self.heap_top;
    let n_words = n.words;
    // Explicitly forbid expanding without a GC, fail if capacity is exceeded
    if pos + n_words >= self.stack_top {
      // return Err(Error::HeapIsFull);
      panic!(
        "Heap is full requested={} have={}",
        n,
        self.get_heap_available()
      );
    }

    // Assume we can grow the data without reallocating
    let raw_nil = Term::nil().raw();
    let new_chunk = unsafe { self.get_heap_begin_ptr_mut().add(self.heap_top) };

    if init_nil {
      unsafe {
        for i in 0..n_words {
          new_chunk.add(i).write(raw_nil)
        }
      }
    }

    self.heap_top += n_words;

    Ok(new_chunk)
  }

  fn get_y(&self, index: Word) -> RtResult<Term> {
    if !self.stack_have_y(index) {
      println!(
        "Stack value requested y{}, depth={}",
        index,
        self.stack_depth()
      );
      return Err(RtErr::StackIndexRange(index));
    }
    let pos = index + self.stack_top + 1;
    let result = Term::from_raw(self.data[pos]);
    debug_assert!(result.is_value(), "Should never get a #Nonvalue<> from y[]");
    Ok(result)
  }

  #[inline]
  fn get_y_unchecked(&self, index: Word) -> Term {
    let pos = index + self.stack_top + 1;
    Term::from_raw(self.data[pos])
  }

  /// Set stack value (`index`th from stack top) to `val`.
  fn set_y(&mut self, index: Word, val: Term) -> RtResult<()> {
    debug_assert!(val.is_value(), "Should never set y[] to a #Nonvalue<>");
    if !self.stack_have_y(index) {
      return Err(RtErr::StackIndexRange(index));
    }
    if cfg!(feature = "trace_stack_changes") {
      println!("{}{} = {}", "set y".green(), index, val);
    }
    self.data[index + self.stack_top + 1] = val.raw();
    Ok(())
  }

  /// Take `cp` from stack top and deallocate `n+1` words of stack.
  fn stack_deallocate(&mut self, n: usize) -> Term {
    assert!(
      self.stack_top + n < self.capacity,
      "Failed to dealloc {}+1 words (s_top {}, s_end {})",
      n,
      self.stack_top,
      self.capacity
    );

    let cp = Term::from_raw(self.data[self.stack_top]);
    assert!(
      cp.is_cp(),
      "Dealloc expected a CP value on stack top, got {}",
      cp
    );
    self.stack_top += n + 1;
    cp
  }

  /// Express the intent to allocate `size` words on the heap, which may either
  /// include an attempt to GC, or incur a heap fragment allocation.
  /// Does not immediately allocate.
  fn allocate_intent(&mut self, size: WordSize, _live: usize) -> RtResult<()> {
    if self.heap_check_available(size) {
      return Ok(());
    }
    Err(RtErr::HeapIsFull("heap::allocate_intent"))
  }

  fn allocate_intent_no_gc(&mut self, _size: WordSize) -> RtResult<()> {
    Ok(())
  }

  #[inline]
  fn heap_check_available(&self, need: WordSize) -> bool {
    self.heap_top + need.words <= self.stack_top
  }

  #[inline]
  fn stack_check_available(&self, need: WordSize) -> bool {
    self.heap_top + need.words <= self.stack_top
  }

  /// Allocate stack cells without checking. Call `stack_have(n)` beforehand.
  fn stack_alloc_unchecked(&mut self, need: WordSize, fill_nil: bool) {
    if need.words == 0 {
      return;
    }
    self.stack_top -= need.words;

    // Clear the new cells
    let raw_nil = Term::nil().raw();
    unsafe {
      let p = self.get_heap_begin_ptr_mut().add(self.stack_top);

      if fill_nil {
        for y in 0..need.words {
          p.add(y).write(raw_nil)
        }
      }
    }
  }

  fn stack_depth(&self) -> usize {
    self.capacity - self.stack_top
  }


  /// Push a Term to stack without checking. Call `stack_have(1)` beforehand.
  #[inline]
  fn stack_push_lterm_unchecked(&mut self, val: Term) {
    if cfg!(feature = "trace_stack_changes") {
      println!("{} {}", "push (unchecked)".green(), val);
    }
    self.stack_top -= 1;
    self.data[self.stack_top] = val.raw();
  }

  /// Sets the stack top.
  /// Arg: new_stack_top - offset from the heap end
  fn drop_stack_words(&mut self, n_drop: usize) {
    println!("drop_stack_words {}", n_drop);
    assert!(self.stack_top + n_drop < self.capacity);
    self.stack_top += n_drop;
  }

  /// Go through stack values searching for a stored CP, skip if it does not
  /// point to a catch instruction.
  /// Returns: the location stored on stack
  unsafe fn unroll_stack_until_catch(&self) -> Option<NextCatchResult> {
    let mut ptr: *const Word = self.get_stack_top_ptr();
    let stack_start: *const Word = self.get_stack_start_ptr();
    // Counter how many stack cells to drop
    let mut stack_drop = 0usize;

    loop {
      if ptr >= stack_start {
        return None;
      }
      // Hope we found a CP on stack (good!)
      let term_at_ptr = Term::from_raw(ptr.read());

      if term_at_ptr.is_catch() {
        // Typical stack frame looks like:
        // >>-top-> CP Catch ...
        // Drop 1 less word than where the catch was found to preserve that CP
        return Some(NextCatchResult {
          loc: term_at_ptr.get_catch_ptr(),
          stack_drop: stack_drop - 1,
        });
      }
      ptr = ptr.add(1);
      stack_drop += 1;
    }
  }

  /// Create a constant iterator for walking the heap.
  /// This is used by heap walkers such as "dump.rs"
  unsafe fn heap_iter(&self) -> iter::HeapIterator {
    let last = self.heap_top as isize;
    let begin = self.get_heap_start_ptr() as *const Term;
    iter::HeapIterator::new(begin, begin.offset(last))
  }

  fn belongs_to_heap(&self, p: *const Word) -> bool {
    p < self.get_heap_start_ptr() || p >= self.get_heap_top_ptr()
  }

  fn stack_dump(&self) {
    if self.stack_depth() == 0 {
      println!("stack: empty");
      return;
    }

    let mut i = 0;
    let max_i = self.stack_depth() - 1;
    loop {
      if i >= max_i || i >= 10 {
        break;
      }
      println!("stack Y[{}] = {}", i, self.get_y_unchecked(i));
      i += 1;
    }
  }
}

// === === ===

impl FlatHeap {
  fn get_size_for(d: Designation) -> usize {
    match d {
      Designation::ProcessHeap => DEFAULT_PROC_HEAP,
      Designation::ModuleLiterals => DEFAULT_LIT_HEAP,
      Designation::BinaryHeap => BINARY_HEAP_CAPACITY,
      Designation::TransientDestructible => 1,
      Designation::ProgramArgumentsHeap => 512,
    }
  }

  pub fn new(designation: Designation) -> Self {
    let capacity = Self::get_size_for(designation);
    assert!(capacity > 0);
    let mut h = Self {
      data: Vec::with_capacity(capacity),
      heap_top: 0,
      stack_top: capacity,
      capacity,
    };
    unsafe { h.data.set_len(capacity) };
    h
  }

  /// How many words do we have before it will require GC/growth.
  #[inline]
  fn get_heap_max_capacity(&self) -> usize {
    self.data.capacity()
  }

  /// Heap usage stat.
  #[inline]
  fn get_heap_used_words(&self) -> usize {
    self.heap_top
  }

  #[inline]
  fn get_heap_available(&self) -> usize {
    self.stack_top - self.heap_top
  }

  #[inline]
  pub fn get_heap_start_ptr(&self) -> *const Word {
    self.data.as_ptr()
  }

  #[inline]
  fn get_heap_begin_ptr_mut(&mut self) -> *mut Word {
    self.data.as_mut_ptr()
  }

  /// Get pointer to end of the allocated heap (below the stack top).
  #[inline]
  pub fn get_heap_top_ptr(&self) -> *const Word {
    unsafe { self.get_heap_start_ptr().add(self.heap_top) }
  }

  /// Stack start is same as end of everything, pointer to the first word after
  /// the allocated memory, used as limit when iterating the stack.
  #[inline]
  unsafe fn get_stack_start_ptr(&self) -> *const Word {
    self.get_end_ptr()
  }

  #[inline]
  unsafe fn get_end_ptr(&self) -> *const Word {
    self.get_heap_start_ptr().add(self.capacity)
  }

  #[inline]
  unsafe fn get_stack_top_ptr(&self) -> *const Word {
    self.get_heap_start_ptr().add(self.stack_top)
  }

  #[allow(dead_code)]
  pub fn stack_info(&self) {
    println!("Stack (s_top {}, s_end {})", self.stack_top, self.capacity)
  }

  /// Check whether `y+1`-th element can be found in stack
  #[inline]
  pub fn stack_have_y(&self, y: Word) -> bool {
    self.capacity - self.stack_top >= y + 1
  }
}
