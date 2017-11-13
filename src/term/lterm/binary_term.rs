use term::immediate;
use term::primary;
//use defs::Word;


pub trait BinaryTerm {
  fn is_binary(&self) -> bool;
  fn is_empty_binary(&self) -> bool;
}


impl BinaryTerm for super::LTerm {

  unsafe fn is_binary(&self) -> bool {
    if self.is_empty_binary() { return true }
    if !self.is_box() { return false }

    let p = self.box_ptr();
    let box_tag = primary::header::get_tag(*p);
    //box_tag == primary::header::TAG_HEADER_HEAPOBJ
    panic!("TODO: Organize check via heapobj in a nice way, generalize")
  }


  /// Check whether a value is an empty binary.
  #[inline]
  fn is_empty_binary(&self) -> bool {
    self.value == immediate::IMM2_SPECIAL_EMPTY_BIN_RAW
  }
}


/// Create an empty binary value.
#[inline]
pub fn empty_binary() -> super::LTerm {
  super::LTerm { value: immediate::IMM2_SPECIAL_EMPTY_BIN_RAW }
}
