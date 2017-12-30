use rt_defs::Word;
use term::lterm::*;

use std::ptr;


#[derive(Clone)]
pub struct PtrMut(pub *mut Word);


pub struct Ptr(pub *const Word);


impl PtrMut {

  #[inline]
  pub fn from_pointer(p: *mut Word) -> PtrMut {
    PtrMut(p)
  }

  pub unsafe fn set_hd(&self, val: LTerm) {
    let PtrMut(p) = *self;
    *p = val.raw()
  }

  pub unsafe fn set_tl(&self, val: LTerm) {
    let PtrMut(p) = *self;
    *p.offset(1) = val.raw()
  }

  pub fn make_cons(&self) -> LTerm {
    let PtrMut(p) = *self;
    make_cons(p)
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


impl Ptr {
  #[inline]
  pub fn from_pointer(p: *const Word) -> Ptr {
    assert_ne!(p, ptr::null());
    Ptr(p)
  }

//  pub fn raw_pointer(&self) -> *const Word { self.p }

//  pub fn make_cons(&self) -> LTerm {
//    LTerm::make_cons(self.p)
//  }

  /// Peek into the cons, and get head value.
  pub unsafe fn hd(&self) -> LTerm {
    let Ptr(p) = *self;
//    assert_ne!(p, ptr::null());
    LTerm::from_raw(*p)
  }

  /// Peek into the cons, and get tail value.
  pub unsafe fn tl(&self) -> LTerm {
    let Ptr(p) = *self;
//    assert_ne!(p, ptr::null());
    LTerm::from_raw(*p.offset(1))
  }
}
