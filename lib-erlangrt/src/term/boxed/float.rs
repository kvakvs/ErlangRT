use crate::{
  defs::{ByteSize, WordSize},
  emulator::heap::Heap,
  fail::RtResult,
  term::{
    boxed::{
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
      BoxHeader,
    },
    classify,
  },
};
use core::{mem::size_of, ptr};

#[allow(dead_code)]
pub struct Float {
  header: BoxHeader,
  pub value: f64,
}

impl TBoxed for Float {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_NUMBER
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_FLOAT
  }
}

impl Float {
  #[allow(dead_code)]
  const fn storage_size() -> WordSize {
    ByteSize::new(core::mem::size_of::<Self>()).get_words_rounded_up()
  }

  #[allow(dead_code)]
  fn new(value: f64) -> Self {
    let storage_size = ByteSize::new(size_of::<Self>()).get_words_rounded_up();
    Self {
      header: BoxHeader::new::<Self>(storage_size),
      value,
    }
  }

  #[allow(dead_code)]
  pub unsafe fn create_into(hp: &mut Heap, value: f64) -> RtResult<*mut Self> {
    let n_words = Self::storage_size();
    let this = hp.alloc::<Self>(n_words, false)?;

    ptr::write(this, Self::new(value));

    Ok(this)
  }
}
