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
}

impl JumpTable {
  /// Size of a tuple in memory with the header word (used for allocations)
  #[inline]
  const fn storage_size(n_pairs: usize) -> WordSize {
    // Minus one because data0 in tuple already consumes one word
    let self_size =
      ByteSize::new(core::mem::size_of::<Self>()).get_words_rounded_up() - 1;
    WordSize::new(self_size.words() + n_pairs * 2)
  }

  fn new(n_pairs: usize) -> JumpTable {
    assert_ne!(n_pairs, 0, "Can't create an empty jump table");
    Self {
      header: BoxHeader::new::<Self>(Self::storage_size(n_pairs).words()),
      val0: Term::non_value(),
    }
  }

  #[inline]
  pub fn get_count(&self) -> usize {
    (self.header.get_arity() - BoxHeader::storage_size().words()) / 2
  }

  /// Allocate `n_pairs`*2 cells and form a tuple-like structure
  pub fn create_into(hp: &mut Heap, n_pairs: usize) -> RtResult<*mut Self> {
    let n = Self::storage_size(n_pairs);
    let p = hp.alloc::<Self>(n, false)?;
    unsafe {
      core::ptr::write(p, Self::new(n_pairs));
    }
    Ok(p)
  }

  // Write i-th pair (base 0)
  pub unsafe fn set_pair(&mut self, index: usize, val: Term, location: Term) {
    self.header.ensure_valid();
    debug_assert!(index < self.get_arity());

    // Take i-th word after the tuple header
    let data = &self.val0 as *mut Term;
    core::ptr::write(data.add(index), val)
  }

  // Read tuple's i-th element (base 0)
  pub unsafe fn get_pair(&self, index: usize) -> (Term, Term) {
    self.header.ensure_valid();
    debug_assert!(index < self.get_count());

    let data = &self.val0 as *const Term;
    let val = core::ptr::read(data.add(index));
    let location = core::ptr::read(data.add(index));
    (val, location)
  }

  /// Format jump table contents
  pub unsafe fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.header.ensure_valid();
    write!(f, "{{")?;

    let count = self.get_count();
    for i in 0..count {
      if i > 0 {
        write!(f, ", ")?
      }
      let (val, loc) = self.get_pair(i);
      write!(f, "{} -> {}", val, loc)?;
    }
    write!(f, "}}")
  }
}
