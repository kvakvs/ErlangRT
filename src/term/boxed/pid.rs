use crate::{
  emulator::heap::Heap,
  fail::RtResult,
  rt_defs::{storage_bytes_to_words, Word, WordSize},
  term::{
    boxed::{BoxHeader, BOXTYPETAG_EXTERNALPID},
    lterm::LTerm,
  },
};
use core::ptr;


/// Represents Pid box on heap.
pub struct ExternalPid {
  pub header: BoxHeader,
  pub node: LTerm,
  pub id: Word,
}

impl ExternalPid {
  const fn storage_size() -> WordSize {
    storage_bytes_to_words(std::mem::size_of::<ExternalPid>())
  }

  fn new(node: LTerm, id: Word) -> ExternalPid {
    let arity = ExternalPid::storage_size().words() - 1;
    ExternalPid {
      header: BoxHeader::new(BOXTYPETAG_EXTERNALPID, arity),
      node,
      id,
    }
  }

  /// Allocates
  pub fn create_into(hp: &mut Heap, node: LTerm, id: Word) -> RtResult<*mut BoxHeader> {
    // TODO do something with possible error from hp.heap_allocate
    let p = hp.alloc::<ExternalPid>(ExternalPid::storage_size(), false)?;
    unsafe { ptr::write(p, ExternalPid::new(node, id)) }
    Ok(p as *mut BoxHeader)
  }
}
