use crate::{
  defs::{BitReader, BitSize, ByteReader, ByteSize, Word, WordSize},
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      binary::{binaryheap_bin::BinaryHeapBinary, trait_interface::TBinary, BinaryType},
      Binary,
    },
    Term,
  },
};

/// Defines operations with reference to binary.
/// Pointer to this can be directly casted from pointer to boxed::Binary
pub struct ReferenceToBinary {
  pub bin_header: Binary,
  pub size: BitSize,
  refc: Word,
  pub pointer: *mut BinaryHeapBinary,
}

impl ReferenceToBinary {
  pub fn storage_size() -> WordSize {
    let header_size = ByteSize::new(std::mem::size_of::<Self>());
    header_size.get_words_rounded_up()
  }

  #[allow(dead_code)]
  pub unsafe fn on_destroy(this: *mut ReferenceToBinary) {
    if (*this).refc > 0 {
      (*this).refc -= 1;
      return;
    }
  }
}

impl TBinary for ReferenceToBinary {
  fn get_type(&self) -> BinaryType {
    BinaryType::RefToBinaryHeap
  }

  fn get_byte_size(&self) -> ByteSize {
    self.size.get_byte_size_rounded_up()
  }

  fn get_bit_size(&self) -> BitSize {
    self.size
  }

  fn get_byte_reader(&self) -> Option<ByteReader> {
    unimplemented!()
  }

  unsafe fn get_data_mut(&mut self) -> &mut [u8] {
    unimplemented!()
  }

  unsafe fn get_data(&self) -> &[u8] {
    unimplemented!()
  }

  fn get_bit_reader(&self) -> BitReader {
    unimplemented!()
  }

  fn store(&mut self, _data: &[u8]) -> RtResult<()> {
    // TODO: Maybe should be possible? Assist with resolution into BinaryHeapBinary
    return Err(RtErr::CannotCopyIntoRefbin);
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
    let p = self.pointer as *mut TBinary;
    (*p).put_integer(val, size, offset, flags)
  }
}
