use crate::{
  defs::sizes::ByteSize,
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

/// Another type of binary. Refers to a slice in another binary.
pub struct BinarySlice {
  pub bin_header: boxed::binary::Binary,
  pub offset: usize,
  pub size: ByteSize,
  // TODO: Make sure this is detected when garbage collected
  pub orig: *const TBinary,
}

impl TBinary for BinarySlice {
  fn get_type(&self) -> BinaryType {
    unimplemented!()
  }

  fn get_size(&self) -> ByteSize {
    unimplemented!()
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
    return Err(RtErr::CannotCopyIntoBinSlice);
  }

  fn make_term(&self) -> LTerm {
    unimplemented!()
  }

  fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "#subbin[{}]<<", self.size)?;
    write!(f, "...>>")
  }
}
