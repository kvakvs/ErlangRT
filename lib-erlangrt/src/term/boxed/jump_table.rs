use crate::{
  defs::{ByteSize, WordSize},
  emulator::heap::{AllocInit, THeap},
  fail::RtResult,
  term::{
    boxed::{
      self,
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
      BoxHeader,
    },
    classify,
    value::Term,
  },
};
use core::fmt;

/// An array of sorted pairs, which like a tuple stores the array in its memory
pub struct JumpTable {
  header: BoxHeader,
  /// First data word is stored here
  val0: Term,
}

impl TBoxed for JumpTable {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_SPECIAL
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_JUMP_TABLE
  }

  //  fn inplace_map(&mut self, mapfn: &InplaceMapFn) {
  //    let this_p = self as *mut JumpTable;
  //    unsafe {
  //      let count = (*this_p).get_count();
  //      for i in 0..count {
  //        let (val, loc) = (*this_p).get_pair(i);
  //        (*this_p).set_pair(
  //          i,
  //          mapfn(this_p as *mut BoxHeader, val),
  //          mapfn(this_p as *mut BoxHeader, loc),
  //        );
  //      }
  //    }
  //  }
}

impl JumpTable {
  /// Size of a tuple in memory with the header word (used for allocations)
  #[inline]
  const fn storage_size(n_pairs: usize) -> WordSize {
    // Minus one because data0 in tuple already consumes one word
    let self_size = ByteSize::new(core::mem::size_of::<Self>()).get_words_rounded_up();
    WordSize::new(self_size.words - 1 + n_pairs * 2)
  }

  fn new(n_pairs: usize) -> JumpTable {
    assert_ne!(n_pairs, 0, "Can't create an empty jump table");
    Self {
      header: BoxHeader::new::<Self>(Self::storage_size(n_pairs)),
      val0: Term::non_value(),
    }
  }

  #[inline]
  pub fn get_count(&self) -> usize {
    (self.header.get_storage_size() - BoxHeader::storage_size().words) / 2
  }

  /// Allocate `n_pairs`*2 cells and form a tuple-like structure
  pub fn create_into(hp: &mut THeap, n_pairs: usize) -> RtResult<*mut Self> {
    let storage_size = Self::storage_size(n_pairs);
    let p = hp.alloc(storage_size, AllocInit::Uninitialized)? as *mut Self;
    unsafe {
      p.write(Self::new(n_pairs));
    }
    Ok(p)
  }

  // Write i-th pair (base 0)
  pub unsafe fn set_pair(&mut self, index: usize, val: Term, location: Term) {
    self.header.ensure_valid();
    debug_assert!(index < self.get_count());

    let data = &mut self.val0 as *mut Term;
    data.add(index * 2).write(val);
    data.add(index * 2 + 1).write(location);
  }

  // Write value component in the i-th pair (base 0)
  pub unsafe fn set_value(&mut self, index: usize, val: Term) {
    self.header.ensure_valid();
    debug_assert!(index < self.get_count());

    let data = &mut self.val0 as *mut Term;
    data.add(index * 2).write(val)
  }

  // Write location component in the i-th pair (base 0)
  pub unsafe fn set_location(&mut self, index: usize, val: Term) {
    self.header.ensure_valid();
    debug_assert!(index < self.get_count());

    let data = &mut self.val0 as *mut Term;
    data.add(index * 2 + 1).write(val)
  }

  // Read tuple's i-th element (base 0)
  pub unsafe fn get_pair(&self, index: usize) -> (Term, Term) {
    self.header.ensure_valid();
    debug_assert!(index < self.get_count());

    let data = &self.val0 as *const Term;
    let val = data.add(index * 2).read();
    let location = data.add(index * 2 + 1).read();
    (val, location)
  }

  // Read tuple's i-th element location (base 0)
  pub unsafe fn get_location(&self, index: usize) -> Term {
    self.header.ensure_valid();
    debug_assert!(index < self.get_count());

    let data = &self.val0 as *const Term;
    data.add(index * 2 + 1).read()
  }

  /// Format jump table contents
  pub unsafe fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.header.ensure_valid();
    write!(f, "#JumpTab<")?;

    let count = self.get_count();
    for i in 0..count {
      if i > 0 {
        write!(f, ", ")?
      }
      let (val, loc) = self.get_pair(i);
      write!(f, "{} -> {}", val, loc)?;
    }
    write!(f, ">")
  }

  pub fn inplace_map_t<T>(&mut self, mut mapfn: T)
  where
    T: FnMut(*mut boxed::JumpTable, Term) -> Term,
  {
    let this_p = self as *mut JumpTable;
    unsafe {
      let count = (*this_p).get_count();
      for i in 0..count {
        let (val, loc) = (*this_p).get_pair(i);
        (*this_p).set_pair(i, mapfn(this_p, val), mapfn(this_p, loc));
      }
    }
  }
}
