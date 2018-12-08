use crate::{
  emulator::heap::Heap,
  fail::RtResult,
  rt_defs::{storage_bytes_to_words, WordSize},
  term::boxed::{BoxHeader, BOXTYPETAG_FLOAT},
};
use core::ptr;


#[allow(dead_code)]
pub struct Float {
  header: BoxHeader,
  pub value: f64,
}

impl Float {
  #[allow(dead_code)]
  const fn storage_size() -> WordSize {
    storage_bytes_to_words(core::mem::size_of::<Float>())
  }


  #[allow(dead_code)]
  fn new(value: f64) -> Float {
    let storage_size = storage_bytes_to_words(core::mem::size_of::<Float>());
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
