use crate::{
  defs::{BitReader, BitSize, ByteReader, ByteSize, WordSize},
  emulator::heap::{AllocInit, THeap},
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      binary::{trait_interface::TBinary, BinaryType},
      Binary,
    },
    Term,
  },
};
use core::ptr;

/// Another type of binary. Refers to a slice in another binary.
pub struct BinarySlice {
  pub bin_header: Binary,
  pub offset: BitSize,
  pub size: BitSize,
  // TODO: Make sure this is recognized as a term during the GC
  pub orig: *const dyn TBinary,
}

impl BinarySlice {
  /// Return size in words for on-heap creation
  pub fn storage_size() -> WordSize {
    let header_size = ByteSize::new(std::mem::size_of::<Self>());

    // The size of `BinarySlice` in words rounded up, no extra storage
    header_size.get_words_rounded_up()
  }

  pub unsafe fn create_into(
    orig: *const dyn TBinary,
    offset: BitSize,
    size: BitSize,
    hp: &mut dyn THeap,
  ) -> RtResult<*const dyn TBinary> {
    if size.bits == 0 {
      // Return binary {} immediate special instead!
      return Err(RtErr::CreatingZeroSizedSlice);
    }

    // Size of header + data in words, to be allocated
    let storage_sz = Self::storage_size();
    let this = hp.alloc(storage_sz, AllocInit::Uninitialized)? as *mut Self;

    let bin_header = Binary::new(BinaryType::Slice, storage_sz);

    // Create and write the block header (Self)
    let new_self = Self {
      bin_header,
      offset,
      size,
      orig,
    };
    this.write(new_self);

    Ok(this as *mut dyn TBinary)
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

  fn get_byte_reader(&self) -> Option<ByteReader> {
    if self.offset.get_last_byte_bits() == 0 {
      // The offset is byte-aligned, we can actually return a faster byte-reader
      match unsafe { (*self.orig).get_byte_reader() } {
        Some(r) => unsafe {
          Some(r.set_offset_and_size(
            self.offset.get_byte_size_rounded_down(),
            self.size.get_byte_size_rounded_down(),
          ))
        },
        None => None,
      }
    } else {
      None
    }
  }

  unsafe fn get_data_mut(&mut self) -> &mut [u8] {
    // Can not use mutable access on slice
    #[allow(clippy::invalid_null_ptr_usage)]
    core::slice::from_raw_parts_mut(ptr::null_mut(), 0)
  }

  unsafe fn get_data(&self) -> &[u8] {
    unimplemented!()
  }

  fn get_bit_reader(&self) -> BitReader {
    let r = unsafe { (*self.orig).get_bit_reader() };
    r.add_bit_offset(self.offset)
  }

  fn store(&mut self, _data: &[u8]) -> RtResult<()> {
    return Err(RtErr::CannotCopyIntoBinSlice);
  }

  fn make_term(&self) -> Term {
    Term::make_boxed(&self.bin_header)
  }

  unsafe fn put_integer(
    &mut self,
    _val: Term,
    _size: BitSize,
    _offset: BitSize,
    _flags: crate::beam::opcodes::BsFlags,
  ) -> Result<(), RtErr> {
    panic!("Can't put_integer into a binary slice")
  }
}
