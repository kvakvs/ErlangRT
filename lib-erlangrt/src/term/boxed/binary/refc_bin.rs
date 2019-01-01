use crate::{
  defs::{ByteSize, Word},
  term::boxed::{self, binary::binaryheap_bin::BinaryHeapBinary},
};

/// Defines operations with reference to binary.
/// Pointer to this can be directly casted from pointer to boxed::Binary
pub struct ReferenceToBinary {
  pub bin: boxed::binary::Binary,
  pub size: ByteSize,
  refc: Word,
  pub pointer: *mut BinaryHeapBinary,
}

impl ReferenceToBinary {
  #[allow(dead_code)]
  pub unsafe fn on_destroy(this: *mut ReferenceToBinary) {
    if (*this).refc > 0 {
      (*this).refc -= 1;
      return;
    }
  }
}
