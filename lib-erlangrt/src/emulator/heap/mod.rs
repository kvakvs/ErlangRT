pub mod catch;
pub mod copy_term;
pub mod dump;
pub mod flat_heap;
pub mod iter;

use crate::{
  defs::WordSize, emulator::heap::flat_heap::FlatHeap, fail::RtResult, term::boxed,
};

pub type Heap = FlatHeap;

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
pub fn allocate_cons(hp: &mut Heap) -> RtResult<*mut boxed::Cons> {
  hp.alloc::<boxed::Cons>(WordSize::new(2), false)
}
