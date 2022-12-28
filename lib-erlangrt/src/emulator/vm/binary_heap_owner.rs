use crate::{
  defs::WordSize,
  emulator::heap::{Designation, Heap, THeap, THeapOwner},
  fail::RtResult,
};

pub struct BinaryHeapOwner {
  heap: Heap,
}

impl BinaryHeapOwner {
  pub fn new() -> Self {
    Self {
      heap: Heap::new(Designation::BinaryHeap),
    }
  }
}

impl THeapOwner for BinaryHeapOwner {
  fn ensure_heap(&mut self, _need: WordSize) -> RtResult<()> {
    unimplemented!()
  }

  #[inline]
  fn get_heap(&self) -> &dyn THeap {
    &self.heap as &dyn THeap
  }

  #[inline]
  fn get_heap_mut(&mut self) -> &mut dyn THeap {
    // &self.heap as &mut THeap
    let heap_ref = &mut self.heap;
    heap_ref as &mut dyn THeap
  }
}
