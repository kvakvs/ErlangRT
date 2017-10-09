use defs::Word;
use term::lterm::LTerm;
use term::primary;

/// Size of a tuple in memory with the header word (used for allocations).
#[inline]
pub fn storage_size(size: Word) -> Word { size + 1 }

/// Represents raw layout of tuple in mutable memory.
pub struct RawTupleMut {
  p: *mut Word,
}


impl RawTupleMut {
  /// Given a pointer initialize a tuple header here, hence unsafe. Return a
  /// `RawTuple` wrapper.
  pub unsafe fn create_at(p: *mut Word, arity: Word) -> RawTupleMut {
    *p = primary::header::make_tuple_header_raw(arity);
    RawTupleMut { p }
  }


  /// Given a pointer to an already initialized tuple, just return a wrapper.
  pub fn from_pointer(p: *mut Word) -> RawTupleMut {
    RawTupleMut { p }
  }


  pub unsafe fn arity(&self) -> Word {
    primary::get_value(*self.p)
  }


  /// Zero-based set element function
  pub unsafe fn set_element_base0(&self, i: Word, val: LTerm) {
    assert!(i < self.arity());
    *self.p.offset(i as isize + 1) = val.raw()
  }


//  pub unsafe fn get_element(&self, i: Word) -> LTerm {
//    LTerm::from_raw(*self.p.offset(i as isize + 1))
//  }


  /// Box the `self.p` pointer into `LTerm`.
  pub fn make_tuple(&self) -> LTerm {
    LTerm::make_box(self.p)
  }
}


/// Represents raw layout of tuple in read-only memory.
pub struct RawTuple {
  p: *const Word,
}

impl RawTuple {
  /// Given a pointer to an already initialized tuple, just return a wrapper.
  pub fn from_pointer(p: *const Word) -> RawTuple {
    RawTuple { p }
  }


//  pub unsafe fn arity(&self) -> Word {
//    primary::get_value(*self.p)
//  }


//  pub unsafe fn get_element(&self, i: Word) -> LTerm {
//    LTerm::from_raw(*self.p.offset(i as isize + 1))
//  }


//  /// Box the `self.p` pointer into `LTerm`.
//  pub fn make_tuple(&self) -> LTerm {
//    LTerm::make_box(self.p)
//  }
}
