use crate::{
  defs::{BitSize, ByteSize},
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      binary::{trait_interface::TBinary, BinaryType},
      Binary,
    },
    lterm::LTerm,
  },
};

/// Defines operations with a binary on the binary heap
/// Pointer to this can be directly casted from pointer to boxed::Binary
pub struct BinaryHeapBinary {
  pub bin_header: Binary,
  pub size: BitSize,
  pub data: usize, // first 8 (or 4) bytes of data begin here
}

impl TBinary for BinaryHeapBinary {
  fn get_type(&self) -> BinaryType {
    BinaryType::BinaryHeap
  }

  fn get_byte_size(&self) -> ByteSize {
    self.size.get_bytes_rounded_up()
  }

  fn get_bit_size(&self) -> BitSize {
    self.size
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

    let avail_size = self.size.get_bytes_rounded_up();
    if avail_size.bytes() < data.len() {
      return Err(RtErr::HeapBinTooSmall(data.len(), avail_size));
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
}
