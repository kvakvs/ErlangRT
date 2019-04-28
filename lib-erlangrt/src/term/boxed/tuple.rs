use crate::{
  defs::{ByteSize, Word, WordSize},
  emulator::heap::heap_trait::THeap,
  fail::RtResult,
  term::{
    boxed::{
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
      BoxHeader,
    },
    classify,
    value::Term,
  },
};
use core::{fmt, ptr};

/// A fixed-size array which stores everything in its allocated memory on
/// process heap.
pub struct Tuple {
  header: BoxHeader,
  /// First data word is stored here. If a tuple is 0 elements, it cannot be
  /// created and an immediate `Term::empty_tuple()` should be used instead.
  pub data0: Term,
}

impl TBoxed for Tuple {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_TUPLE
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_TUPLE
  }

  //  fn inplace_map(&mut self, mapfn: &InplaceMapFn) {
  //    let this_p = self as *mut Tuple;
  //
  //    unsafe {
  //      let count = (*this_p).get_arity();
  //      let data = &mut (*this_p).data0 as *mut Term;
  //
  //      for i in 0..count {
  //        let val = ptr::read(data.add(i));
  //        ptr::write(data.add(i), mapfn(this_p as *mut BoxHeader, val));
  //      }
  //    }
  //  }
}

impl Tuple {
  /// Size of a tuple in memory with the header word (used for allocations)
  #[inline]
  const fn storage_size(arity: usize) -> WordSize {
    // Minus one because data0 in tuple already consumes one word
    let self_size = ByteSize::new(core::mem::size_of::<Self>()).get_words_rounded_up();
    WordSize::new(self_size.words + arity - 1)
  }

  fn new(arity: usize) -> Tuple {
    assert_ne!(arity, 0, "Can't create tuple of arity 0 on heap");
    Tuple {
      header: BoxHeader::new::<Tuple>(Self::storage_size(arity)),
      data0: Term::nil(),
    }
  }

  #[inline]
  pub fn get_arity(&self) -> usize {
    self.header.get_storage_size() - BoxHeader::storage_size().words
  }

  /// Allocate `size+1` cells and form a tuple in memory, return the pointer.
  pub fn create_into(hp: &mut THeap, arity: usize) -> RtResult<*mut Tuple> {
    let n = Self::storage_size(arity);
    let p = hp.alloc::<Tuple>(n, false)?;
    unsafe {
      ptr::write(p, Tuple::new(arity));
    }
    Ok(p)
  }

  // Write tuple's i-th element (base 0) as a raw term value
  pub unsafe fn set_element_raw(&mut self, index: usize, val: Word) {
    self.header.ensure_valid();
    debug_assert!(index < self.get_arity());

    let data = &mut self.data0 as *mut Term as *mut Word;
    ptr::write(data.add(index), val)
  }

  // Write tuple's i-th element (base 0)
  pub unsafe fn set_element(&mut self, index: usize, val: Term) {
    self.header.ensure_valid();
    debug_assert!(index < self.get_arity());

    // Take i-th word after the tuple header
    let data = &mut self.data0 as *mut Term;
    ptr::write(data.add(index), val)
  }

  // Read tuple's i-th element (base 0)
  pub unsafe fn get_element(&self, index: usize) -> Term {
    self.header.ensure_valid();
    debug_assert!(index < self.get_arity());

    let data = &self.data0 as *const Term;
    ptr::read(data.add(index))
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
