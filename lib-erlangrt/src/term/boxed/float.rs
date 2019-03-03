use crate::{
  defs::{ByteSize, WordSize},
  emulator::heap::Heap,
  fail::RtResult,
  term::boxed::{BoxHeader, BOXTYPETAG_FLOAT},
};
use core::{mem::size_of, ptr};

#[allow(dead_code)]
pub struct Float {
  header: BoxHeader,
  pub value: f64,
}

impl Float {
  #[allow(dead_code)]
  const fn storage_size() -> WordSize {
    ByteSize::new(core::mem::size_of::<Float>()).get_words_rounded_up()
  }

  #[allow(dead_code)]
  fn new(value: f64) -> Float {
    let storage_size = ByteSize::new(size_of::<Float>()).get_words_rounded_up();
    Float {
      header: BoxHeader::new(BOXTYPETAG_FLOAT, storage_size.words()),
      value,
    }
  }

  #[allow(dead_code)]
  pub unsafe fn create_into(hp: &mut Heap, value: f64) -> RtResult<*mut Float> {
    let n_words = Float::storage_size();
    let this = hp.alloc::<Float>(n_words, false)?;

    ptr::write(this, Float::new(value));

    Ok(this)
  }
}
