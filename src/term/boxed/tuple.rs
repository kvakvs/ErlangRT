use emulator::heap::Heap;
use emulator::heap::IHeap;
use fail::Error;
use fail::Hopefully;
use term::boxed::{BoxHeader, BoxTypeTag};
use term::boxed;

/// A fixed-size array which stores everything in its allocated memory on
/// process heap.
pub struct Tuple {
  header: BoxHeader,
}

impl Tuple {
  /// Size of a tuple in memory with the header word (used for allocations)
  #[inline]
  pub const fn storage_size(arity: Word) -> Word {
    arity + BoxHeader::storage_size()
  }


  pub fn get_arity(self) -> Word {
    self.header.t
  }


  /// Allocate `size+1` cells and form a tuple in memory, return the pointer.
  pub fn create_into(hp: &mut Heap, arity: Word) -> Hopefully<*mut Tuple> {
    let n = boxed::Tuple::storage_size(arity);
    //let p = hp.heap_allocate(n, false)?;
    boxed::Tuple::create_into(hp, arity)
  }


  pub fn from_pointer(p: *const Word) -> Hopefully<*const Tuple> {
    let tp = p as *const Tuple;
    if tp.header.get_tag() != BoxTypeTag::Tuple {
      return Err(Error::BoxedIsNotATuple)
    }
    Ok(tp)
  }
}
