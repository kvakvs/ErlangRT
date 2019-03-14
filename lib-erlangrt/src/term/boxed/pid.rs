use crate::{
  defs::{ByteSize, Word, WordSize},
  emulator::heap::Heap,
  fail::RtResult,
  term::{
    boxed::{
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
      BoxHeader,
    },
    classify,
    lterm::Term,
  },
};
use core::{mem::size_of, ptr};

/// Represents Pid box on heap.
pub struct ExternalPid {
  pub header: BoxHeader,
  pub node: Term,
  pub id: Word,
}

impl TBoxed for ExternalPid {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_PID
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_EXTERNALPID
  }
}

impl ExternalPid {
  const fn storage_size() -> WordSize {
    ByteSize::new(size_of::<ExternalPid>()).get_words_rounded_up()
  }

  fn new(node: Term, id: Word) -> ExternalPid {
    let storage_size = ExternalPid::storage_size() - WordSize::one();
    ExternalPid {
      header: BoxHeader::new::<ExternalPid>(storage_size),
      node,
      id,
    }
  }

  /// Allocates
  pub fn create_into(hp: &mut Heap, node: Term, id: Word) -> RtResult<*mut BoxHeader> {
    // TODO do something with possible error from hp.heap_allocate
    let p = hp.alloc::<ExternalPid>(ExternalPid::storage_size(), false)?;
    unsafe { ptr::write(p, ExternalPid::new(node, id)) }
    Ok(p as *mut BoxHeader)
  }
}
