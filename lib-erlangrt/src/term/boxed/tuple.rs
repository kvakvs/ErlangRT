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
//  /// First data word is stored here. If a tuple is 0 elements, it cannot be
//  /// created and an immediate `LTerm::empty_tuple()` should be used instead.
//  data0: LTerm,
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
  const fn storage_size(arity: usize) -> WordSize {
    // Minus one because data0 in tuple already consumes one word
    let self_size = ByteSize::new(core::mem::size_of::<Self>()).get_words_rounded_up();
    WordSize::new(self_size.words() + arity)
  }

  fn new(arity: usize) -> Tuple {
    assert_ne!(arity, 0, "Can't create tuple of arity 0 on heap");
    Tuple {
      header: BoxHeader::new::<Tuple>(Self::storage_size(arity).words()),
      // data0: LTerm::non_value(),
    }
  }

  #[inline]
  pub fn get_arity(&self) -> usize {
    self.header.get_arity() - BoxHeader::storage_size().words()
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

  // Write tuple's i-th element (base 0) as a raw term value
  pub unsafe fn set_element_raw(&mut self, index: usize, val: Word) {
    self.header.ensure_valid();
    debug_assert!(index < self.get_arity());

    let data = (self as *mut Self).add(1) as *mut LTerm as *mut Word;
    core::ptr::write(data.add(index), val)
  }

  // Write tuple's i-th element (base 0)
  pub unsafe fn set_element(&mut self, index: usize, val: LTerm) {
    self.header.ensure_valid();
    debug_assert!(index < self.get_arity());

    // Take i-th word after the tuple header
    let data = (self as *mut Self).add(1) as *mut LTerm;
    core::ptr::write(data.add(index), val)
  }

  // Read tuple's i-th element (base 0)
  pub unsafe fn get_element(&self, index: usize) -> LTerm {
    self.header.ensure_valid();
    debug_assert!(index < self.get_arity());

    let data = (self as *const Self).add(1) as *const LTerm;
    core::ptr::read(data.add(index))
  }

  /// Format tuple contents
  pub unsafe fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.header.ensure_valid();
    write!(f, "{{")?;

    let arity = self.get_arity();
    for i in 0..arity {
      if i > 0 {
        write!(f, ", ")?
      }
      write!(f, "{}", self.get_element(i))?;
    }
    write!(f, "}}")
  }
}
