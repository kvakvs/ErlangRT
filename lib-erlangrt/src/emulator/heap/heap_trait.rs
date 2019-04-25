// use crate::emulator::heap::Designation;
use crate::{
  defs::{sizes::WordSize, Word},
  emulator::heap::iter,
  fail::RtResult,
};

/// Trait defines shared API which all heap implementations must expose
pub trait THeap {
  fn alloc(&mut self, sz: WordSize, nil_init: bool) -> RtResult<*mut Word>;

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
}
