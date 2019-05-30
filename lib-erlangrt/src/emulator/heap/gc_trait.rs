//! Trait for Garbage Collector
//! GC can only be compatible with the heap type it is designed for.
use crate::{
  emulator::heap::{heap_trait::THeap, *},
  fail::RtResult,
  term::heap_walker::*,
};

pub trait TGc {
  fn new() -> Self;

  /// GC takes a range of mutable-term-ranges
  fn garbage_collect(
    heap: &THeap,
    walker: HeapWalker,
    roots: Box<TRootIterator>,
  ) -> RtResult<()>;
}

/// Null GC does nothing, and instead panics
#[allow(dead_code)]
pub struct NullGc {}

impl TGc for NullGc {
  fn new() -> Self {
    Self {}
  }

  fn garbage_collect(_heap: &THeap, walker: HeapWalker,_roots: Box<TRootIterator>) -> RtResult<()> {
    unimplemented!("NullGC is not designed to collect any garbage")
  }
}
