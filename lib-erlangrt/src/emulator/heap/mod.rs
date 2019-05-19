pub mod catch;
pub mod copy_term;
pub mod dump;
pub mod flat_heap;
pub mod gene_heap;
pub mod heap_trait;
pub mod iter;

use crate::{
  defs::WordSize,
  emulator::heap::{gene_heap::GenerationalHeap, heap_trait::THeap},
  fail::RtResult,
  term::boxed,
};

// pub type Heap = FlatHeap;
pub type Heap = GenerationalHeap;

/// Specifies the intended use of the heap
pub enum Designation {
  ProcessHeap,
  ModuleLiterals,
  BinaryHeap,
  // Used to store command line args on startup
  ProgramArgumentsHeap,
  // Heap of smallest size to be destroyed after it is swapped with the real one
  TransientDestructible,
}

/// Allocate 2 cells `[Head | Tail]` of raw cons cell, and return the pointer.
#[inline]
pub fn allocate_cons(hp: &mut THeap) -> RtResult<*mut boxed::Cons> {
  heap_alloc::<boxed::Cons>(hp, WordSize::new(2), false)
}

#[inline]
pub fn heap_alloc<T>(hp: &mut THeap, sz: WordSize, nil_init: bool) -> RtResult<*mut T> {
  match hp.alloc(sz, nil_init) {
    Ok(x) => Ok(x as *mut T),
    Err(y) => Err(y),
  }
}
