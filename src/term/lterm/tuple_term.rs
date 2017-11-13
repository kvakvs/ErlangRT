use term::raw::{TuplePtr, TuplePtrMut};
use term::primary;
use term::immediate;
use defs::Word;


pub trait TupleTerm {
  /// Get a proxy object for read-only accesing the cons contents.
  unsafe fn raw_tuple(&self) -> TuplePtr;

  /// Get a proxy object for looking and modifying cons contents.
  unsafe fn raw_tuple_mut(&self) -> TuplePtrMut;

  /// Create an empty tuple value.
  fn empty_tuple() -> super::LTerm;

  /// Check whether a value is an empty tuple.
  fn is_empty_tuple(&self) -> bool;
}


impl TupleTerm for super::LTerm {

  /// Get a proxy object for read-only accesing the cons contents.
  unsafe fn raw_tuple(&self) -> TuplePtr {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_HEADER);
    assert_eq!(primary::header::get_tag(v),
               primary::header::TAG_HEADER_TUPLE);
    let boxp = primary::pointer(v);
    TuplePtr::from_pointer(boxp)
  }


  /// Get a proxy object for looking and modifying cons contents.
  unsafe fn raw_tuple_mut(&self) -> TuplePtrMut {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_HEADER);
    assert_eq!(primary::header::get_tag(v),
               primary::header::TAG_HEADER_TUPLE);
    let boxp = primary::pointer_mut(v);
    TuplePtrMut::from_pointer(boxp)
  }


  /// Create an empty tuple value.
  #[inline]
  fn empty_tuple() -> super::LTerm {
    super::LTerm { value: immediate::IMM2_SPECIAL_EMPTY_TUPLE_RAW }
  }

  /// Check whether a value is an empty tuple.
  #[inline]
  fn is_empty_tuple(&self) -> bool {
    self.value == immediate::IMM2_SPECIAL_EMPTY_TUPLE_RAW
  }
}


#[inline]
pub fn make_tuple_header(arity: Word) -> super::LTerm {
  super::LTerm { value: primary::header::make_tuple_header_raw(arity) }
}
