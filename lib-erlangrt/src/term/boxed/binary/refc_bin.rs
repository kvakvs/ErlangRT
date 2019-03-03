use crate::{
  defs::{ByteSize, Word},
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      binary::{
        binaryheap_bin::BinaryHeapBinary, bitsize::BitSize, trait_interface::TBinary,
        BinaryType,
      },
      Binary,
    },
    lterm::LTerm,
  },
};
use core::fmt;

/// Defines operations with reference to binary.
/// Pointer to this can be directly casted from pointer to boxed::Binary
pub struct ReferenceToBinary {
  pub bin_header: Binary,
  pub size: ByteSize,
  refc: Word,
  pub pointer: *mut BinaryHeapBinary,
}

impl ReferenceToBinary {
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
    unimplemented!()
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

  fn store(&mut self, _data: &[u8]) -> RtResult<()> {
    // TODO: Maybe should be possible? Assist with resolution into BinaryHeapBinary
    return Err(RtErr::CannotCopyIntoRefbin);
  }

  fn make_term(&self) -> LTerm {
    LTerm::make_boxed((&self.bin_header) as *const Binary)
  }

  fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "#refbin[{}]<<", self.size)?;
    panic!("notimpl: printing refbin to binary heap");
    // write!(f, ">>")
  }
}
