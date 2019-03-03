use crate::{
  defs::ByteSize,
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      self,
      binary::{
        trait_interface::{BitSize, TBinary},
        BinaryType,
      },
    },
    lterm::LTerm,
  },
};
use core::fmt;

/// Defines operations with a binary on the binary heap
/// Pointer to this can be directly casted from pointer to boxed::Binary
pub struct BinaryHeapBinary {
  pub bin_header: boxed::binary::Binary,
  pub size: ByteSize,
}

impl TBinary for BinaryHeapBinary {
  fn get_type(&self) -> BinaryType {
    BinaryType::BinaryHeap
  }

  fn get_size(&self) -> ByteSize {
    self.size
  }

  fn get_bit_size(&self) -> BitSize {
    unimplemented!()
  }

  fn get_data(&self) -> *const u8 {
    unimplemented!()
  }

  fn get_data_mut(&mut self) -> *mut u8 {
    unimplemented!()
  }

  fn store(&mut self, data: &[u8]) -> RtResult<()> {
    if data.is_empty() {
      return Ok(());
    }

    let avail_size = self.size.bytes();
    if avail_size < data.len() {
      return Err(RtErr::HeapBinTooSmall(data.len(), avail_size));
    }

    let bin_bytes = self.get_data_mut();
    unsafe {
      core::ptr::copy_nonoverlapping(&data[0], bin_bytes, data.len());
    }
    Ok(())
  }

  fn make_term(&self) -> LTerm {
    unimplemented!()
  }

  fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "#bhbin[{}]<<", self.size)?;
    panic!("notimpl: printing bin on binary heap");
    // write!(f, ">>")
  }
}
