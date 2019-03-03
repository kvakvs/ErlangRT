use crate::{
  defs::{self, BitSize, ByteSize, WordSize},
  emulator::heap::Heap,
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      binary::{trait_interface::TBinary, BinaryType},
      Binary,
    },
    lterm::LTerm,
  },
};

/// Another type of binary. Refers to a slice in another binary.
pub struct BinarySlice {
  pub bin_header: Binary,
  pub offset: usize,
  pub size: BitSize,
  // TODO: Make sure this is recognized as a term during the GC
  pub orig: *const TBinary,
}

impl BinarySlice {
  /// Return size in words for on-heap creation
  pub fn storage_size(size: BitSize) -> WordSize {
    let header_size = BitSize::with_unit(std::mem::size_of::<Self>(), defs::BYTE_BITS);
    // The size is `ProcessHeapBinary` in words rounded up + storage bytes rounded up
    WordSize::new(
      header_size.get_words_rounded_up().words() + size.get_words_rounded_up().words(),
    )
  }

  pub unsafe fn create_into(
    orig: *const TBinary,
    size: BitSize,
    hp: &mut Heap,
  ) -> RtResult<*const TBinary> {
    if size.bit_count == 0 {
      // Return binary {} immediate special instead!
      return Err(RtErr::CreatingZeroSizedSlice);
    }

    // Size of header + data in words, to be allocated
    let storage_sz = Self::storage_size(size);
    let this = hp.alloc::<Self>(storage_sz, false)?;

    // Create and write the block header (Self)
    let bin_header = Binary::new(BinaryType::ProcessHeap, storage_sz);
    let new_self = Self {
      bin_header,
      offset: 0,
      size,
      orig,
    };
    core::ptr::write(this, new_self);

    Ok(this as *mut TBinary)
  }
}

impl TBinary for BinarySlice {
  fn get_type(&self) -> BinaryType {
    unimplemented!()
  }

  fn get_byte_size(&self) -> ByteSize {
    self.size.get_bytes_rounded_up()
  }

  fn get_bit_size(&self) -> BitSize {
    self.size
  }

  fn get_data(&self) -> *const u8 {
    unimplemented!()
  }

  fn get_data_mut(&mut self) -> *mut u8 {
    unimplemented!()
  }

  fn store(&mut self, _data: &[u8]) -> RtResult<()> {
    return Err(RtErr::CannotCopyIntoBinSlice);
  }

  fn make_term(&self) -> LTerm {
    LTerm::make_boxed((&self.bin_header) as *const Binary)
  }
}
