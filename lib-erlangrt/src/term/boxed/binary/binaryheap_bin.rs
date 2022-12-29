use crate::{
  defs::{BitReader, BitSize, ByteReader, SizeBytes, SizeWords},
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      binary::{bits_paste, trait_interface::TBinary, BinaryType},
      Binary,
    },
    Term,
  },
};
use core::ptr;

/// Defines operations with a binary on the binary heap
/// Pointer to this can be directly casted from pointer to boxed::Binary
pub struct BinaryHeapBinary {
  pub bin_header: Binary,
  pub size: BitSize,
  pub data: usize, // first 8 (or 4) bytes of data begin here
}

impl BinaryHeapBinary {
  #[allow(dead_code)]
  pub fn storage_size(size: BitSize) -> SizeWords {
    let header_size = SizeBytes::new(std::mem::size_of::<Self>());
    // The size is `BinaryHeapBinary` in words rounded up + storage bytes rounded up
    header_size.get_words_rounded_up() + size.get_words_rounded_up()
  }
}

impl TBinary for BinaryHeapBinary {
  fn get_type(&self) -> BinaryType {
    BinaryType::BinaryHeap
  }

  fn get_byte_size(&self) -> SizeBytes {
    self.size.get_byte_size_rounded_up()
  }

  fn get_bit_size(&self) -> BitSize {
    self.size
  }

  fn get_byte_reader(&self) -> Option<ByteReader> {
    let data = (&self.data) as *const usize as *const u8;
    let len = self.size.get_byte_size_rounded_up();
    Some(ByteReader::new(data, len.bytes()))
  }

  unsafe fn get_data_mut(&mut self) -> &mut [u8] {
    let data = (&self.data) as *const usize as *mut u8;
    let len = self.size.get_byte_size_rounded_up();
    core::slice::from_raw_parts_mut(data, len.bytes())
  }

  unsafe fn get_data(&self) -> &[u8] {
    let data = (&self.data) as *const usize as *const u8;
    let len = self.size.get_byte_size_rounded_up();
    core::slice::from_raw_parts(data, len.bytes())
  }

  fn get_bit_reader(&self) -> BitReader {
    unimplemented!()
  }

  fn store(&mut self, data: &[u8]) -> RtResult<()> {
    if data.is_empty() {
      return Ok(());
    }

    let avail_size = self.size.get_byte_size_rounded_up();
    if avail_size.bytes() < data.len() {
      return Err(RtErr::HeapBinTooSmall(data.len(), avail_size));
    }

    let bin_bytes = unsafe { self.get_data_mut() };
    unsafe {
      ptr::copy_nonoverlapping(&data[0], bin_bytes.as_mut_ptr(), data.len());
    }
    Ok(())
  }

  fn make_term(&self) -> Term {
    Term::make_boxed((&self.bin_header) as *const Binary)
  }

  unsafe fn put_integer(
    &mut self,
    val: Term,
    size: BitSize,
    offset: BitSize,
    flags: crate::beam::opcodes::BsFlags,
  ) -> RtResult<()> {
    let data = self.get_data_mut();
    bits_paste::put_integer(val, size, data, offset, flags)
  }
}
