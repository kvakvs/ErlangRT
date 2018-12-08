use crate::emulator::heap::Heap;
use crate::fail::RtResult;
use crate::rt_defs::storage_bytes_to_words;
use crate::term::boxed::{BoxHeader, BOXTYPETAG_FLOAT};

use core::ptr;


#[allow(dead_code)]
pub struct Float {
  header: BoxHeader,
  pub value: f64,
}

impl Float {
  #[allow(dead_code)]
  const fn storage_size() -> usize {
    storage_bytes_to_words(core::mem::size_of::<Float>())
  }


  #[allow(dead_code)]
  fn new(value: f64) -> Float {
    Float {
      header: BoxHeader::new(
        BOXTYPETAG_FLOAT,
        storage_bytes_to_words(core::mem::size_of::<Float>()),
      ),
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
