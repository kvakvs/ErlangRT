//! Module implements simple Erlang-style heap which holds Words (raw LTerms)
//! or other arbitrary data, all marked.
use term::lterm::LTerm;
use defs::Word;
use term::raw::{RawCons, RawTuple, RawBignum};

use num;

/// Default heap size for constants (literals) when loading a module.
pub const DEFAULT_LIT_HEAP: Word = 1024;
/// Default heap size when spawning a process.
pub const DEFAULT_PROC_HEAP: Word = 300;


/// A heap structure which grows upwards with allocations. Cannot expand
/// implicitly and will return error when capacity is exceeded. Organize a
/// garbage collect call to get more memory TODO: gc on heap
pub struct Heap {
  data: Vec<Word>,
}


impl Heap {
  pub fn new(capacity: Word) -> Heap {
    Heap{
      data: Vec::with_capacity(capacity),
    }
  }


  /// Expand heap to host `n` words of data
  pub fn allocate(&mut self, n: Word) -> Option<*mut Word> {
    let pos = self.data.len();
    // Explicitly forbid expanding without a GC, fail if capacity is exceeded
    if pos + n >= self.data.capacity() {
      return None
    }
    // Assume we can grow the data without reallocating
    self.data.resize(pos + n, LTerm::nil().raw());
    Some(&mut self.data[pos] as *mut Word)
  }


  /// Allocate 2 cells `[Head | Tail]` of raw cons cell, and return the pointer.
  pub fn allocate_cons(&mut self) -> Option<RawCons> {
    match self.allocate(2) {
      Some(p) => Some(RawCons::from_pointer(p)),
      None => None
    }
  }


  /// Allocate `size+1` cells and form a tuple in memory, return the pointer.
  pub fn allocate_tuple(&mut self, size: Word) -> Option<RawTuple> {
    match self.allocate(RawTuple::storage_size(size)) {
      Some(p) => unsafe { Some(RawTuple::create_at(p, size)) },
      None => None
    }
  }


  /// Allocate words on heap enough to store bignum digits and copy the given
  /// bignum to memory, return the pointer.
  pub fn allocate_big(&mut self, big: &num::BigInt) -> Option<RawBignum> {
    match self.allocate(RawBignum::storage_size(&big)) {
      Some(p) => unsafe { Some(RawBignum::create_at(p, &big)) },
      None => None
    }
  }
}
