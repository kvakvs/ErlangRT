use crate::{
  emulator::heap::THeap,
  fail::{RtErr, RtResult},
  term::{boxed, Term},
};

impl Term {
  /// Check whether a value is a small integer, a big integer or a float.
  pub fn is_number(self) -> bool {
    self.is_small()
      || self.is_boxed_of_(|t| {
        t == boxed::BOXTYPETAG_BIGINTEGER || t == boxed::BOXTYPETAG_FLOAT
      })
  }

  /// Constructor to create a float on heap. May fail if the heap is full or
  /// something else might happen.
  pub fn make_float(hp: &mut THeap, val: f64) -> RtResult<Self> {
    let pf = unsafe { boxed::Float::create_into(hp, val)? };
    Ok(Self::make_boxed(pf))
  }

  /// Check whether a term is boxed and then whether it points to a word of
  /// memory tagged as float
  pub fn is_float(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_FLOAT)
  }

  pub fn get_float(self) -> RtResult<f64> {
    if !self.is_boxed() {
      return Err(RtErr::TermIsNotABoxed);
    }
    let _p = self.get_box_ptr::<boxed::BoxHeader>();
    unimplemented!("float box")
  }

  /// Returns float value, performs no extra checks. The caller is responsible
  /// for the value being a boxed float.
  #[inline]
  pub unsafe fn get_float_unchecked(self) -> f64 {
    let p = self.get_box_ptr::<boxed::Float>();
    (*p).value
  }
}
