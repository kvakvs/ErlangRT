//! Module implements simple Erlang-style heap which holds Words (raw LTerms)
//! or other arbitrary data, all marked.
use term::lterm::LTerm;
use defs::Word;

/// Default heap size when loading a module
pub const DEFAULT_LIT_HEAP: Word = 1024;

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
}
