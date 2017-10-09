use defs::Word;
use term::lterm::LTerm;

pub struct RawConsMut {
  p: *mut Word,
}

impl RawConsMut {
  pub fn from_pointer(p: *mut Word) -> RawConsMut {
    RawConsMut { p }
  }

  pub unsafe fn set_hd(&self, val: LTerm) {
    *self.p = val.raw()
  }

  pub unsafe fn set_tl(&self, val: LTerm) {
    *self.p.offset(1) = val.raw()
  }

  pub fn make_cons(&self) -> LTerm {
    LTerm::make_cons(self.p)
  }

//  /// Peek into the cons, and get head value.
//  pub unsafe fn hd(&self) -> LTerm {
//    LTerm::from_raw(*self.p)
//  }

//  /// Peek into the cons, and get tail value.
//  pub unsafe fn tl(&self) -> LTerm {
//    LTerm::from_raw(*(self.p.offset(1)))
//  }
}


pub struct RawCons {
  p: *const Word,
}


impl RawCons {
  pub fn from_pointer(p: *const Word) -> RawCons {
    RawCons { p }
  }

  pub fn raw_pointer(&self) -> *const Word { self.p }

//  pub fn make_cons(&self) -> LTerm {
//    LTerm::make_cons(self.p)
//  }

  /// Peek into the cons, and get head value.
  pub unsafe fn hd(&self) -> LTerm {
    LTerm::from_raw(*self.p)
  }

  /// Peek into the cons, and get tail value.
  pub unsafe fn tl(&self) -> LTerm {
    LTerm::from_raw(*(self.p.offset(1)))
  }
}
