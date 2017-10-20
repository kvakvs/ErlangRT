//! `RawTuple` and `RawTupleMut` define pointer which refers to a `HeapTuple` 
//! on heap.

use defs::Word;
use term::lterm::LTerm;
use term::primary;

/// Size of a tuple in memory with the header word (used for allocations).
#[inline]
pub fn storage_size(size: Word) -> Word { size + 1 }


/// Represents a pointer to raw tuple in mutable memory.
pub enum TuplePtrMut { Ptr(*mut Word) }


impl TuplePtrMut {
  /// Given a pointer initialize a tuple header here, hence unsafe. Return a
  /// `RawTuple` wrapper.
  #[inline]
  pub unsafe fn create_at(p: *mut Word, arity: Word) -> TuplePtrMut {
    *p = primary::header::make_tuple_header_raw(arity);
    TuplePtrMut::from_pointer(p)
  }


  /// Given a pointer to an already initialized tuple, just return a wrapper.
  #[inline]
  pub fn from_pointer(p: *mut Word) -> TuplePtrMut {
    TuplePtrMut::Ptr(p)
  }


  pub unsafe fn arity(&self) -> Word {
    let TuplePtrMut::Ptr(p) = *self;
    primary::get_value(*p)
  }


  /// Zero-based set element function
  #[inline]
  pub unsafe fn set_element_base0(&self, i: Word, val: LTerm) {
    assert!(i < self.arity());
    let TuplePtrMut::Ptr(p) = *self;
    *p.offset(i as isize + 1) = val.raw()
  }


  /// Zero-based set element function
  #[inline]
  pub unsafe fn set_raw_word_base0(&self, i: Word, val: Word) {
    assert!(i < self.arity());
    let TuplePtrMut::Ptr(p) = *self;
    *p.offset(i as isize + 1) = val
  }


  //  pub unsafe fn get_element(&self, i: Word) -> LTerm {
//    LTerm::from_raw(*self.p.offset(i as isize + 1))
//  }


  /// Box the `self.p` pointer into `LTerm`.
  pub fn make_tuple(&self) -> LTerm {
    let TuplePtrMut::Ptr(p) = *self;
    LTerm::make_box(p)
  }
}


/// Represents raw layout of tuple in read-only memory.
pub enum TuplePtr { Ptr(*const Word) }


impl TuplePtr {
  /// Given a pointer to an already initialized tuple, just return a wrapper.
  pub fn from_pointer(p: *const Word) -> TuplePtr {
    TuplePtr::Ptr(p as *const Word)
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
