use crate::{
  defs::{BitDataReader, BitSize, ByteDataReader, ByteSize, WordSize},
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
  pub offset: BitSize,
  pub size: BitSize,
  // TODO: Make sure this is recognized as a term during the GC
  pub orig: *const TBinary,
}

impl BinarySlice {
  /// Return size in words for on-heap creation
  pub fn storage_size() -> WordSize {
    let header_size = ByteSize::new(std::mem::size_of::<Self>());

    // The size of `BinarySlice` in words rounded up, no extra storage
    header_size.get_words_rounded_up()
  }

  pub unsafe fn create_into(
    orig: *const TBinary,
    offset: BitSize,
    size: BitSize,
    hp: &mut Heap,
  ) -> RtResult<*const TBinary> {
    if size.bit_count == 0 {
      // Return binary {} immediate special instead!
      return Err(RtErr::CreatingZeroSizedSlice);
    }

    // Size of header + data in words, to be allocated
    let storage_sz = Self::storage_size();
    let this = hp.alloc::<Self>(storage_sz, false)?;

    let bin_header = Binary::new(BinaryType::Slice, storage_sz);

    // Create and write the block header (Self)
    let new_self = Self {
      bin_header,
      offset,
      size,
      orig,
    };
    core::ptr::write(this, new_self);

    Ok(this as *mut TBinary)
  }
}

impl TBinary for BinarySlice {
  fn get_type(&self) -> BinaryType {
    BinaryType::Slice
  }

  fn get_byte_size(&self) -> ByteSize {
    self.size.get_byte_size_rounded_up()
  }

  fn get_bit_size(&self) -> BitSize {
    self.size
  }

  fn get_byte_reader(&self) -> Option<ByteDataReader> {
    if self.offset.get_last_byte_bits() == 0 {
      // The offset is byte-aligned, we can actually return a faster byte-reader
      match unsafe { (*self.orig).get_byte_reader() } {
        Some(r) => unsafe {
          Some(r.set_byte_offset(self.offset.get_byte_size_rounded_down()))
        },
        None => None
      }
    } else {
      None
    }
  }

  unsafe fn get_data_mut(&mut self) -> &mut [u8] {
    // Can not use mutable access on slice
    core::slice::from_raw_parts_mut(core::ptr::null_mut(), 0)
  }

  fn get_bit_reader(&self) -> BitDataReader {
    unsafe { (*self.orig).get_bit_reader() }
  }

  fn store(&mut self, _data: &[u8]) -> RtResult<()> {
    return Err(RtErr::CannotCopyIntoBinSlice);
  }

  fn make_term(&self) -> LTerm {
    LTerm::make_boxed(&self.bin_header)
  }
}
