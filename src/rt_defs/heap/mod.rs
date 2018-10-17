pub mod ptr;
pub mod iter;

use super::Word;
//use super::heap::iter::IHeapIterator;
//use super::heap::ptr::DataPtr;


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
