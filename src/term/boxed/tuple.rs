use emulator::heap::Heap;
use fail::Error;
use fail::RtResult;
use rt_defs::Word;
use term::boxed;
use term::boxed::{BoxHeader, BOXTYPETAG_TUPLE};
use term::lterm::LTerm;

//use core::ptr;


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


  fn new(arity: usize) -> Tuple {
    Tuple {
      header: BoxHeader::new(BOXTYPETAG_TUPLE, arity),
    }
  }


  pub unsafe fn get_arity(this: *const Tuple) -> Word {
    (*this).header.get_arity()
  }


  /// Allocate `size+1` cells and form a tuple in memory, return the pointer.
  pub fn create_into(hp: &mut Heap, arity: Word) -> RtResult<*mut Tuple> {
    let n = boxed::Tuple::storage_size(arity);
    let p = hp.alloc::<Tuple>(n, false)?;
    unsafe {
      core::ptr::write(p, Tuple::new(arity));
    }
    Ok(p)
  }


  /// Convert any p into *const Tuple + checking the header word to be Tule
  pub unsafe fn from_pointer<T>(p: *const T) -> RtResult<*const Tuple> {
    let tp = p as *const Tuple;
    if (*tp).header.get_tag() != BOXTYPETAG_TUPLE {
      return Err(Error::BoxedIsNotATuple);
    }
    Ok(tp)
  }


  /// Convert any p into *mut Tuple + checking the header word to be Tule
  pub unsafe fn from_pointer_mut<T>(p: *mut T) -> RtResult<*mut Tuple> {
    let tp = p as *mut Tuple;
    if (*tp).header.get_tag() != BOXTYPETAG_TUPLE {
      return Err(Error::BoxedIsNotATuple);
    }
    Ok(tp)
  }


  pub unsafe fn set_raw_word_base0(this: *mut Tuple, index: Word, val: Word) {
    debug_assert!(index < Tuple::get_arity(this));
    let p = this as *mut Word;
    *p.offset(index as isize + 1) = val
  }


  pub unsafe fn set_element_base0(this: *mut Tuple, i: Word, val: LTerm) {
    // Take i-th word after the tuple header
    let word_ptr = this.add(1) as *mut Word;
    core::ptr::write(word_ptr.add(i), val.raw())
  }


  pub unsafe fn get_element_base0(this: *const Tuple, i: Word) -> LTerm {
    let word_ptr = this.add(1) as *const LTerm;
    core::ptr::read(word_ptr.add(i))
  }
}
