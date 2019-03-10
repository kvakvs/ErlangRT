use crate::{
  defs::{BitDataReader, BitSize, ByteDataReader, ByteSize, WordSize},
  emulator::heap::Heap,
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      self,
      binary::{trait_interface::TBinary, writing, BinaryType},
      Binary,
    },
    lterm::Term,
  },
};

/// Defines operations with a binary on process heap.
/// Pointer to this can be directly casted from pointer to boxed::Binary
pub struct ProcessHeapBinary {
  pub bin_header: boxed::binary::Binary,
  pub size: BitSize,
  pub data: usize,
}

impl TBinary for ProcessHeapBinary {
  fn get_type(&self) -> BinaryType {
    BinaryType::ProcessHeap
  }

  fn get_byte_size(&self) -> ByteSize {
    self.size.get_byte_size_rounded_up()
  }

  fn get_bit_size(&self) -> BitSize {
    self.size
  }

  fn get_byte_reader(&self) -> Option<ByteDataReader> {
    let data = (&self.data) as *const usize as *const u8;
    let len = self.size.get_byte_size_rounded_up();
    Some(ByteDataReader::new(data, len.bytes()))
  }

  unsafe fn get_data_mut(&mut self) -> &mut [u8] {
    let data = (&self.data) as *const usize as *mut u8;
    let len = self.size.get_byte_size_rounded_up();
    core::slice::from_raw_parts_mut(data, len.bytes())
  }

  fn get_bit_reader(&self) -> BitDataReader {
    unimplemented!()
  }

  fn store(&mut self, data: &[u8]) -> RtResult<()> {
    if data.is_empty() {
      return Ok(());
    }

    let avail_size = self.size.get_byte_size_rounded_up();
    if avail_size.bytes() < data.len() {
      return Err(RtErr::ProcBinTooSmall(data.len(), avail_size));
    }

    let bin_bytes = unsafe { self.get_data_mut() };
    unsafe {
      core::ptr::copy_nonoverlapping(&data[0], bin_bytes.as_mut_ptr(), data.len());
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
  ) -> RtResult<()> {
    let data = self.get_data_mut();
    writing::put_integer(val, size, data, offset)
  }
}

impl ProcessHeapBinary {
  pub const ONHEAP_THRESHOLD: usize = 64;

  pub fn storage_size(size: BitSize) -> WordSize {
    let header_size = ByteSize::new(std::mem::size_of::<Self>());
    // The size is `ProcessHeapBinary` in words rounded up + storage bytes rounded up
    header_size.get_words_rounded_up() + size.get_words_rounded_up()
  }

  pub unsafe fn create_into(size: BitSize, hp: &mut Heap) -> RtResult<*mut TBinary> {
    // Size of header + data in words, to be allocated
    let storage_sz = Self::storage_size(size);
    let this = hp.alloc::<Self>(storage_sz, false)?;

    // Create and write the block header (Self)
    let bin_header = Binary::new(BinaryType::ProcessHeap, storage_sz);
    let new_self = Self {
      bin_header,
      size,
      data: 0,
    };
    core::ptr::write(this, new_self);

    Ok(this as *mut TBinary)
  }
}
