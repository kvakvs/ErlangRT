use term::boxed::{BoxTypeTag, BoxHeader};
use term::lterm::LTerm;
use emulator::heap::IHeap;
use rt_defs::{WORD_BYTES, Word};
use fail::Hopefully;

use core::ptr;
use std::mem;


/// Represents Pid box on heap.
pub struct RemotePid {
  pub header: BoxHeader,
  pub node: LTerm,
  pub id: Word,
}

impl RemotePid {
  const fn get_heap_size() -> Word {
    (mem::size_of::<RemotePid> + WORD_BYTES - 1) / WORD_BYTES
  }

  fn new(node: LTerm, id: Word) -> RemotePid {
    RemotePid {
      header: BoxHeader::new(BoxTypeTag::Pid),
      node,
      id
    }
  }

  /// Allocates
  pub fn create_into(hp: &mut IHeap,
                     node: LTerm,
                     id: Word) -> Hopefully<*mut BoxHeader> {
    // TODO do something with possible error from hp.heap_allocate
    let p = hp.heap_allocate(RemotePid::get_heap_size())?;
    unsafe { ptr::write(p, RemotePid::new(node, id)) }
    Ok(p)
  }
}
