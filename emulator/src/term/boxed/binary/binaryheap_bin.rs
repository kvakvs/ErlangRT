use crate::{defs::ByteSize, term::boxed};

/// Defines operations with a binary on the binary heap
/// Pointer to this can be directly casted from pointer to boxed::Binary
pub struct BinaryHeapBinary {
  pub bin: boxed::binary::Binary,
  pub size: ByteSize,
}
