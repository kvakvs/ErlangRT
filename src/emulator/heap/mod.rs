pub mod dump;
pub mod heap_impl;
pub mod iter;
pub mod ptr;

use rt_defs::Word;


/// Default heap size for constants (literals) when loading a module.
pub const DEFAULT_LIT_HEAP: Word = 8192;
/// Default heap size when spawning a process. (default: 300)
pub const DEFAULT_PROC_HEAP: Word = 8192;


pub use emulator::heap::heap_impl::{Heap, allocate_cons, heap_iter};



#[derive(Debug)]
pub enum HeapError {
  /// Very bad, no more memory to grow.
  OutOfMemory,
  /// No space left in heap. GC requested.
  HeapIsFull,
  /// Attempt to index outside of the current stack.
  StackIndexRange,
}


/// Trait represents an object which can do heap operations.
pub trait IHeap {
  /// How many words do we have before it will require GC/growth.
  fn heap_capacity(&self) -> usize;

  /// How many words are used.
  fn htop(&self) -> usize;

  /// This is used by heap walkers such as "dump.rs"
  fn heap_begin(&self) -> *const Word;

  fn heap_begin_mut(&mut self) -> *mut Word;

  /// This is used by heap walkers such as "dump.rs"
  unsafe fn heap_end(&self) -> *const Word;

  /// Expand heap to host `n` words of data
  fn heap_allocate(&mut self, n: Word, init_nil: bool) -> Result<*mut Word, HeapError>;

  //  /// Allocate words on heap enough to store bignum digits and copy the given
  //  /// bignum to memory, return the pointer.
  //  pub fn allocate_big(&mut self, big: &num::BigInt) -> Result<BignumPtr, HeapError> {

  // /// Create a constant iterator for walking the heap.
  // /// This is used by heap walkers such as "dump.rs"
  //unsafe fn heap_iter(&self) -> IHeapIterator<DataPtr>;
  //unsafe fn heap_iter<IteratorType>(&self) -> IteratorType;

  fn heap_have(&self, need: Word) -> bool;
}

/// A heap iterator. Not very `std::iter::Iterator` compatible but simple.
pub trait IHeapIterator<PtrType> {
  unsafe fn next(&mut self) -> Option<PtrType>;
}
