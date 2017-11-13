//! Functions to manipulate an LTerm as a float. Part of LTerm impl.

//use defs::Word;
//use term::immediate;
use term::primary;
use term::lterm::boxed_term::BoxedTerm;


pub trait FloatTerm {
  unsafe fn is_float(&self) -> bool;
}


impl FloatTerm for super::LTerm {

  /// Check whether a value contains a pointer to a float box. Unsafe (i.e.
  /// will dereference the box pointer).
  unsafe fn is_float(&self) -> bool {
    // For a value to be float it must be a box, which points to heap word with
    // primary header bits having value `TAG_HEADER_FLOAT` and primary tag bits
    // having value `primary::TAG_HEADER`.
    if !self.is_box() {
      return false
    }
    let p = self.box_ptr();
    let box_tag = primary::header::get_tag(*p);
    box_tag == primary::header::TAG_HEADER_FLOAT
  }

}
