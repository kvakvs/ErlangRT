//! Trait for Garbage Collector
//! GC can only be compatible with the heap type it is designed for.
use crate::{emulator::heap::heap_trait::THeap, fail::RtResult, term::value::Term};

/// Mutable root is a mutable slice which will be read, collected and then
/// updated with new locations of the data.
pub type MutableRoot<'t> = &'t mut [Term];

pub trait TGc {
  fn new() -> Self;
  /// GC takes a range of mutable-term-ranges
  fn garbage_collect(heap: &THeap, roots: &[MutableRoot]) -> RtResult<()>;
}

/// Null GC does nothing, and instead panics
pub struct NullGc {}

impl TGc for NullGc {
  fn new() -> Self {
    Self {}
  }

  fn garbage_collect(heap: &THeap, roots: &[MutableRoot]) -> RtResult<()> {
    panic!("The heap is full and there is nothing i can do about it")
  }
}
