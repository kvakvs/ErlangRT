use crate::{
  defs::{Word, WordSize},
  emulator::heap::Heap,
  fail::RtResult,
  term::{
    boxed::{BoxHeader, BOXTYPETAG_EXTERNALPID},
    lterm::LTerm,
  },
};
use core::{mem::size_of, ptr};
use crate::defs::ByteSize;


/// Represents Pid box on heap.
pub struct ExternalPid {
  pub header: BoxHeader,
  pub node: LTerm,
  pub id: Word,
}

impl ExternalPid {
  const fn storage_size() -> WordSize {
    ByteSize::new(size_of::<ExternalPid>()).words_rounded_up()
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
