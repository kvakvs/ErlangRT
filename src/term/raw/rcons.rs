use defs::Word;
use term::lterm::LTerm;

pub struct ConsPtrMut {
  p: *mut Word,
}

impl ConsPtrMut {
  pub fn from_pointer(p: *mut Word) -> ConsPtrMut {
    ConsPtrMut { p }
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


pub struct ConsPtr {
  p: *const Word,
}


impl ConsPtr {
  pub fn from_pointer(p: *const Word) -> ConsPtr {
    ConsPtr { p }
  }

//  pub fn raw_pointer(&self) -> *const Word { self.p }

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
