use crate::{
  defs::{sizes::WordSize, ByteSize},
  emulator::heap::Heap,
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      self,
      binary::{
        trait_interface::{BitSize, TBinary},
        BinaryType,
      },
      Binary,
    },
    lterm::LTerm,
  },
};
use core::fmt;

/// Defines operations with a binary on process heap.
/// Pointer to this can be directly casted from pointer to boxed::Binary
pub struct ProcessHeapBinary {
  pub bin_header: boxed::binary::Binary,
  pub size: ByteSize,
  pub data: usize,
}

impl TBinary for ProcessHeapBinary {
  fn get_type(&self) -> BinaryType {
    unimplemented!()
  }

  fn get_size(&self) -> ByteSize {
    self.size
  }

  fn get_bit_size(&self) -> BitSize {
    unimplemented!()
  }

  fn get_data(&self) -> *const u8 {
    (&self.data) as *const usize as *const u8
  }

  fn get_data_mut(&mut self) -> *mut u8 {
    (&self.data) as *const usize as *mut u8
  }

  fn store(&mut self, data: &[u8]) -> RtResult<()> {
    if data.is_empty() {
      return Ok(());
    }

    let avail_size = self.size.bytes();
    if avail_size < data.len() {
      return Err(RtErr::ProcBinTooSmall(data.len(), avail_size));
    }

    let bin_bytes = self.get_data_mut();
    unsafe {
      core::ptr::copy_nonoverlapping(&data[0], bin_bytes, data.len());
    }
    Ok(())
  }

  fn make_term(&self) -> LTerm {
    LTerm::make_boxed((&self.bin_header) as *const Binary)
  }

  fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "#procbin[{}]<<", self.size)?;

    let n_bytes = self.size.bytes();
    let datap = self.get_data();

    for i in 0..n_bytes {
      if i > 0 {
        write!(f, ", ")?;
      }
      let b = unsafe { core::ptr::read(datap.add(i)) };
      write!(f, "{}", b)?;
    }
    write!(f, ">>")
  }
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

  pub unsafe fn create_into(hp: &mut Heap, size: ByteSize) -> RtResult<*mut TBinary> {
    // Size of header + data in words, to be allocated
    let storage_sz = Self::storage_size(size);
    let this = hp.alloc::<Self>(storage_sz, false)?;

    // Create and write the block header (Self)
    let bin_header = Binary::new(BinaryType::ProcessHeap, storage_sz);
    let new_self = Self { bin_header, size, data: 0 };
    core::ptr::write(this, new_self);

    Ok(this as *mut TBinary)
  }
}
