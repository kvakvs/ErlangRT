// use crate::emulator::heap::Designation;
use crate::{
  defs::{sizes::WordSize, Word},
  emulator::heap::{catch::NextCatchResult, iter},
  fail::RtResult,
  term::value::Term,
};

#[derive(Eq, PartialEq)]
pub enum AllocInit {
  Nil,
  Uninitialized,
}

/// Trait defines shared API which all heap implementations must expose
pub trait THeap {
  fn alloc(&mut self, sz: WordSize, fill: AllocInit) -> RtResult<*mut Word>;

  // Stack access
  //

  fn get_y(&self, index: usize) -> RtResult<Term>;
  fn get_y_unchecked(&self, index: usize) -> Term;
  fn set_y(&mut self, index: usize, val: Term) -> RtResult<()>;

  // Heap & Stack memory management
  //

  /// Take `cp` from stack top and deallocate `n+1` words of stack.
  fn stack_deallocate(&mut self, n: usize) -> Term;

  // / Express the intent to allocate `size` words on the heap, which may either
  // / include an attempt to GC, or incur a heap fragment allocation.
  // / Does not immediately allocate.
  // fn allocate_intent(&mut self, size: WordSize, live: usize) -> RtResult<()>;
  // fn allocate_intent_no_gc(&mut self, size: WordSize) -> RtResult<()>;

  fn heap_check_available(&self, need: WordSize) -> bool;
  fn stack_check_available(&self, need: WordSize) -> bool;
  fn stack_alloc(&mut self, need: WordSize, extra: WordSize, fill: AllocInit);
  fn stack_depth(&self) -> usize;

  /// Push a Term to stack without checking. Call `stack_have(1)` beforehand.
  fn stack_push_lterm_unchecked(&mut self, val: Term);
  fn drop_stack_words(&mut self, n_drop: usize);
  unsafe fn unroll_stack_until_catch(&self) -> Option<NextCatchResult>;

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
