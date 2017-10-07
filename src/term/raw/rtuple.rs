use defs::Word;
use term::lterm::LTerm;
use term::primary;

/// Represents raw layout of tuple in memory.
pub struct RawTuple {
  p: *mut Word,
}

impl RawTuple {
  /// Given a pointer initialize a tuple header here, hence unsafe. Return a
  /// `RawTuple` wrapper.
  pub unsafe fn create_at(p: *mut Word, arity: Word) -> RawTuple {
    *p = primary::make_header_raw(arity);
    RawTuple { p }
  }

  /// Size of a tuple in memory with the header word (used for allocations).
  pub fn word_size(size: Word) -> Word { size + 1 }

  /// Given a pointer to an already initialized tuple, just return a wrapper.
  pub fn from_pointer(p: *mut Word, arity: Word) -> RawTuple {
    RawTuple { p }
  }

  pub unsafe fn arity(&self) -> Word {
    primary::get_value(*self.p)
  }

  /// Zero-based set element function
  pub unsafe fn set_element_base0(&self, i: Word, val: LTerm) {
    assert!(i < self.arity());
    *self.p.offset(i as isize + 1) = val.raw()
  }

  pub unsafe fn get_element(&self, i: Word) -> LTerm {
    LTerm::from_raw(*self.p.offset(i as isize + 1))
  }

  /// Box the `self.p` pointer into `LTerm`.
  pub fn make_tuple(&self) -> LTerm {
    LTerm::make_box(self.p)
  }
}
