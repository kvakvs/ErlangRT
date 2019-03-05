use crate::{
  defs::{ByteSize, Word, WordSize},
  emulator::heap::Heap,
  fail::RtResult,
  term::{
    boxed::{BoxHeader, boxtype},
    lterm::LTerm,
  },
};
use core::{mem::size_of, ptr};
use crate::term::boxed::trait_interface::TBoxed;
use crate::term::classify;
use crate::term::boxed::boxtype::BoxType;

/// Represents Pid box on heap.
pub struct ExternalPid {
  pub header: BoxHeader,
  pub node: LTerm,
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

  fn new(node: LTerm, id: Word) -> ExternalPid {
    let arity = ExternalPid::storage_size().words() - 1;
    ExternalPid {
      header: BoxHeader::new::<ExternalPid>(arity),
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
