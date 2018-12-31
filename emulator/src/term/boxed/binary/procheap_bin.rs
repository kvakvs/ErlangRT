use crate::{defs::ByteSize, term::boxed};

/// Defines operations with a binary on process heap.
/// Pointer to this can be directly casted from pointer to boxed::Binary
pub struct ProcessHeapBinary {
  pub bin: boxed::binary::Binary,
  pub size: ByteSize,
}

impl ProcessHeapBinary {
  pub const ONHEAP_THRESHOLD: usize = 64;
}
