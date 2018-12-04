use emulator::heap::Heap;
use fail::Hopefully;
use rt_defs::{Word, storage_bytes_to_words};
use term::boxed::{BoxHeader, BOXTYPETAG_EXTERNALPID};
use term::lterm::LTerm;

use core::ptr;
use std::mem;


/// Represents Pid box on heap.
pub struct ExternalPid {
  pub header: BoxHeader,
  pub node: LTerm,
  pub id: Word,
}

impl ExternalPid {
  const fn storage_size() -> Word {
    storage_bytes_to_words(mem::size_of::<ExternalPid>)
  }

  fn new(node: LTerm, id: Word) -> ExternalPid {
    ExternalPid {
      header: BoxHeader::new(BOXTYPETAG_EXTERNALPID,
                             ExternalPid::storage_size() - 1),
      node,
      id,
    }
  }

  /// Allocates
  pub fn create_into(hp: &mut Heap,
                     node: LTerm,
                     id: Word) -> Hopefully<*mut BoxHeader> {
    // TODO do something with possible error from hp.heap_allocate
    let p = hp.heap_allocate(ExternalPid::storage_size())?;
    unsafe { ptr::write(p, ExternalPid::new(node, id)) }
    Ok(p)
  }
}
