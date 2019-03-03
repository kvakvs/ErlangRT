use crate::{
  defs::{sizes::WordSize, ByteSize},
  emulator::heap::Heap,
  fail::RtResult,
  term::boxed::{self, binary::BinaryType, Binary},
};

/// Defines operations with a binary on process heap.
/// Pointer to this can be directly casted from pointer to boxed::Binary
pub struct ProcessHeapBinary {
  pub bin_header: boxed::binary::Binary,
  pub size: ByteSize,
}

impl ProcessHeapBinary {
  pub const ONHEAP_THRESHOLD: usize = 64;

  pub fn storage_size(size: ByteSize) -> WordSize {
    let header_size = ByteSize::new(std::mem::size_of::<Self>());
    // The size is `ProcessHeapBinary` in words rounded up + storage bytes rounded up
    WordSize::new(
      header_size.words_rounded_up().words() + size.words_rounded_up().words(),
    )
  }

  pub unsafe fn create_into(hp: &mut Heap, size: ByteSize) -> RtResult<*mut Binary> {
    // Size of header + data in words, to be allocated
    let storage_sz = Self::storage_size(size);
    let this = hp.alloc::<Self>(storage_sz, false)?;

    // Create and write the block header (Self)
    let bin_header = Binary::new(BinaryType::ProcessHeap, storage_sz);
    let new_self = Self { bin_header, size };
    core::ptr::write(this, new_self);

    Ok(this as *mut Binary)
  }
}
