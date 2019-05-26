use crate::{
  defs::WordSize,
  emulator::heap::{gc_trait::NullGc, heap_incremental::IncrementalHeap},
  fail::RtResult,
  term::boxed,
};

pub mod catch;
pub mod copy_term;
pub mod dump;
pub mod gc_copying;
pub mod gc_trait;
pub mod heap_incremental;
pub mod iter;

mod heap_owner_trait;
mod heap_trait;
pub use heap_owner_trait::THeapOwner;
pub use heap_trait::{AllocInit, THeap};

pub type Heap = IncrementalHeap<NullGc>;

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
  heap_alloc::<boxed::Cons>(hp, WordSize::new(2), AllocInit::Uninitialized)
}

#[inline]
pub fn heap_alloc<T>(hp: &mut THeap, sz: WordSize, fill: AllocInit) -> RtResult<*mut T> {
  match hp.alloc(sz, fill) {
    Ok(x) => Ok(x as *mut T),
    Err(y) => Err(y),
  }
}
