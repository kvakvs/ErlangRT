use term::mterm::{BoxType, BoxHeader};
use term::lterm::LTerm;
use emulator::heap::IHeap;
use rt_defs::{WORD_BYTES, Word};

use core::ptr;
use std::mem;


/// Represents Pid box on heap.
pub struct MRemotePid {
  pub header: BoxHeader,
  pub node: LTerm,
  pub id: Word,
}

impl MRemotePid {
  const fn get_heap_size() -> Word {
    (mem::size_of::<MRemotePid> + WORD_BYTES - 1) / WORD_BYTES
  }

  fn new(id: Word) -> MRemotePid {
    MRemotePid {
      header: BoxHeader::new(BoxType::Pid),
      id
    }
  }

  /// Allocates
  pub fn create_in(hp: &mut IHeap, id: Word) -> *mut BoxHeader {
    // TODO do something with possible error from hp.heap_allocate
    let p = hp.heap_allocate(MRemotePid::get_heap_size()).unwrap();
    unsafe { ptr::write(p, MRemotePid::new(id)) }
    p
  }
}
