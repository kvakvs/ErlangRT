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
    lterm::LTerm,
  },
};
use core::fmt;

/// A fixed-size array which stores everything in its allocated memory on
/// process heap.
pub struct Tuple {
  header: BoxHeader,
  /// First data word is stored here. If a tuple is 0 elements, it cannot be
  /// created and an immediate `LTerm::empty_tuple()` should be used instead.
  data0: LTerm,
}

impl TBoxed for Tuple {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_TUPLE
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_TUPLE
  }
}

impl Tuple {
  /// Size of a tuple in memory with the header word (used for allocations)
  #[inline]
  const fn storage_size(arity: Word) -> WordSize {
    // Minus one because data0 in tuple already consumes one word
    let self_size = ByteSize::new(core::mem::size_of::<Self>()).get_words_rounded_up();
    WordSize::new(self_size.words() - 1 + arity)
  }

  fn new(arity: usize) -> Tuple {
    Tuple {
      header: BoxHeader::new::<Tuple>(Self::storage_size(arity).words()),
      data0: LTerm::non_value(),
    }
  }

  pub fn get_arity(&self) -> usize {
    self.header.get_arity()
  }

  /// Allocate `size+1` cells and form a tuple in memory, return the pointer.
  pub fn create_into(hp: &mut Heap, arity: usize) -> RtResult<*mut Tuple> {
    let n = Self::storage_size(arity);
    let p = hp.alloc::<Tuple>(n, false)?;
    unsafe {
      core::ptr::write(p, Tuple::new(arity));
    }
    Ok(p)
  }

  /// Convert any p into *const Tuple + checking the header word to be a Tuple
  //  pub unsafe fn from_pointer<T>(p: *const T) -> RtResult<*const Tuple> {
  //    let tp = p as *const Tuple;
  //    if (*tp).header.get_tag() != BOXTYPETAG_TUPLE {
  //      return Err(RtErr::BoxedIsNotATuple);
  //    }
  //    Ok(tp)
  //  }

  /// Convert any p into *mut Tuple + checking the header word to be Tule
  //  pub unsafe fn from_pointer_mut<T>(p: *mut T) -> RtResult<*mut Tuple> {
  //    let tp = p as *mut Tuple;
  //    if (*tp).header.get_tag() != BOXTYPETAG_TUPLE {
  //      return Err(RtErr::BoxedIsNotATuple);
  //    }
  //    Ok(tp)
  //  }

  // Write tuple's i-th element (base 0) as a raw term value
  pub unsafe fn set_raw_word_base0(&mut self, index: usize, val: Word) {
    debug_assert!(index < self.get_arity());

    let data = (&mut self.data0) as *mut LTerm as *mut Word;
    core::ptr::write(data.add(index), val)
  }

  // Write tuple's i-th element (base 0)
  #[inline]
  pub unsafe fn set_element_base0(&mut self, i: Word, val: LTerm) {
    debug_assert!(i < self.get_arity());

    // Take i-th word after the tuple header
    let data = (&mut self.data0) as *mut LTerm;
    core::ptr::write(data.add(i), val)
  }

  // Read tuple's i-th element (base 0)
  #[inline]
  pub unsafe fn get_element_base0(&self, i: usize) -> LTerm {
    debug_assert!(i < self.get_arity());

    let data = &self.data0 as *const LTerm;
    core::ptr::read(data.add(i))
  }

  /// Format tuple contents
  pub fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{")?;

    let arity = self.get_arity();
    for i in 0..arity {
      if i > 0 {
        write!(f, ", ")?
      }
      unsafe { write!(f, "{}", self.get_element_base0(i))? };
    }
    write!(f, "}}")
  }
}
