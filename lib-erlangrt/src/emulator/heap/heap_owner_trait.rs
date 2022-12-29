use super::heap_trait::THeap;
use crate::{defs::SizeWords, fail::RtResult};

/// Trait must be implemented by heap owners, such as processes (which own their
/// own heaps, the code server (which owns literal heaps for the loaded modules)
/// or the VM (which owns the binary heap).
///
/// The job of THeapOwner is to ensure that the heap size is available and take
/// measures to expand the space (call the GC on its own heap).
pub trait THeapOwner {
  /// We do not pass `live` here because of all heap owners only processes have
  /// registers which can be live. And they can always look into their runtime
  /// context if they need that value.
  fn ensure_heap(&mut self, need: SizeWords) -> RtResult<()>;
  /// For read-only relations with my owned heap
  fn get_heap(&self) -> &dyn THeap;
  /// For read-write relations with my owned heap
  fn get_heap_mut(&mut self) -> &mut dyn THeap;
}
