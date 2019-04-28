// use crate::emulator::heap::Designation;
use crate::{
  defs::{sizes::WordSize, Word},
  emulator::heap::iter,
  fail::RtResult,
  term::value::Term,
};

/// Trait defines shared API which all heap implementations must expose
pub trait THeap {
  fn alloc(&mut self, sz: WordSize, nil_init: bool) -> RtResult<*mut Word>;

  // Data access
  //

  fn get_y(&self, index: Word) -> RtResult<Term>;
  fn get_y_unchecked(&self, index: Word) -> Term;
  fn set_y(&mut self, index: Word, val: Term) -> RtResult<()>;

  // Heap & Stack memory management
  //
  /// Take `cp` from stack top and deallocate `n+1` words of stack.
  fn stack_deallocate(&mut self, n: usize) -> Term;

  /// Express the intent to allocate `size` words on the heap, which may either
  /// include an attempt to GC, or incur a heap fragment allocation.
  /// Does not immediately allocate.
  fn allocate_intent(&mut self, size: WordSize, live: usize) -> RtResult<()>;

  fn heap_check_available(&self, need: WordSize) -> bool;
  fn stack_check_available(&self, need: WordSize) -> bool;
  fn stack_alloc_unchecked(&mut self, need: WordSize, fill_nil: bool);
  fn stack_depth(&self) -> usize;

  /// Push a Term to stack without checking. Call `stack_have(1)` beforehand.
  fn stack_push_lterm_unchecked(&mut self, val: Term);

  // Iteration
  //

  unsafe fn heap_iter(&self) -> iter::HeapIterator;
  fn belongs_to_heap(&self, p: *const Word) -> bool;
  //  fn get_heap_start_ptr(&self) -> *const Word;
  //  fn get_heap_top_ptr(&self) -> *const Word;
  //  fn get_heap_begin_ptr_mut(&mut self) -> *mut Word;
  //  unsafe fn get_stack_start_ptr(&self) -> *const Word;
  //  unsafe fn get_end_ptr(&self) -> *const Word;
  //  unsafe fn get_stack_top_ptr(&self) -> *const Word;

  // Debug and printing
  //
  fn stack_dump(&self);
}
