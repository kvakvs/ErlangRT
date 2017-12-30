//! `RawTuple` and `RawTupleMut` define pointer which refers to a `HeapTuple` 
//! on heap.

use rt_defs::Word;
use term::lterm::*;
use term::primary;
use term::primary::header;

/// Size of a tuple in memory with the header word (used for allocations).
#[inline]
pub fn storage_size(size: Word) -> Word { size + 1 }


/// Represents a pointer to raw tuple in mutable memory.
pub struct PtrMut(*mut Word);


impl PtrMut {
  /// Given a pointer initialize a tuple header here, hence unsafe. Return a
  /// `RawTuple` wrapper.
  pub unsafe fn create_at(p: *mut Word, arity: Word) -> PtrMut {
    *p = primary::header::make_tuple_header_raw(arity);
    PtrMut::from_pointer(p)
  }


  /// Given a pointer to an already initialized tuple, just return a wrapper.
  #[inline]
  pub fn from_pointer(p: *mut Word) -> PtrMut {
    PtrMut(p)
  }


  pub unsafe fn arity(&self) -> Word {
    let PtrMut(p) = *self;
    primary::get_value(*p)
  }


  /// Zero-based set element function
  #[inline]
  pub unsafe fn set_element_base0(&self, i: Word, val: LTerm) {
    assert!(i < self.arity());
    let PtrMut(p) = *self;
    *p.offset(i as isize + 1) = val.raw()
  }


  /// Zero-based set element function
  #[inline]
  pub unsafe fn set_raw_word_base0(&self, i: Word, val: Word) {
    assert!(i < self.arity());
    let PtrMut(p) = *self;
    *p.offset(i as isize + 1) = val
  }


  pub unsafe fn get_element_base0(&self, i: Word) -> LTerm {
    let PtrMut(p) = *self;
    LTerm::from_raw(*p.offset(i as isize + 1))
  }


  /// Box the `self.p` pointer into `LTerm`.
  pub fn make_term(&self) -> LTerm {
    let PtrMut(p) = *self;
    make_box(p)
  }
}


/// Represents raw layout of tuple in read-only memory.
pub struct Ptr(*const Word);


impl Ptr {
  /// Given a pointer to an already initialized tuple, just return a wrapper.
  pub fn from_pointer(p: *const Word) -> Ptr {
    Ptr(p as *const Word)
  }


  pub unsafe fn arity(&self) -> Word {
    let Ptr(p) = *self;
    header::get_arity(*p)
  }


  pub unsafe fn get_element_base0(&self, i: Word) -> LTerm {
    let Ptr(p) = *self;
    LTerm::from_raw(*p.offset(i as isize + 1))
  }


//  /// Box the `self.p` pointer into `LTerm`.
//  pub fn make_tuple(&self) -> LTerm {
//    LTerm::make_box(self.p)
//  }
}
