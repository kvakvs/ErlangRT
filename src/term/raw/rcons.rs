use rt_defs::Word;
use term::lterm::*;

use std::ptr;


#[derive(Clone)]
pub enum ConsPtrMut { Ptr(*mut Word) }


pub enum ConsPtr { Ptr(*const Word) }


impl ConsPtrMut {

  #[inline]
  pub fn from_pointer(p: *mut Word) -> ConsPtrMut {
    ConsPtrMut::Ptr(p)
  }

  pub unsafe fn set_hd(&self, val: LTerm) {
    let ConsPtrMut::Ptr(p) = *self;
    *p = val.raw()
  }

  pub unsafe fn set_tl(&self, val: LTerm) {
    let ConsPtrMut::Ptr(p) = *self;
    *p.offset(1) = val.raw()
  }

  pub fn make_cons(&self) -> LTerm {
    let ConsPtrMut::Ptr(p) = *self;
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


impl ConsPtr {
  #[inline]
  pub fn from_pointer(p: *const Word) -> ConsPtr {
    assert_ne!(p, ptr::null());
    ConsPtr::Ptr(p)
  }

//  pub fn raw_pointer(&self) -> *const Word { self.p }

//  pub fn make_cons(&self) -> LTerm {
//    LTerm::make_cons(self.p)
//  }

  /// Peek into the cons, and get head value.
  pub unsafe fn hd(&self) -> LTerm {
    let ConsPtr::Ptr(p) = *self;
//    assert_ne!(p, ptr::null());
    LTerm::from_raw(*p)
  }

  /// Peek into the cons, and get tail value.
  pub unsafe fn tl(&self) -> LTerm {
    let ConsPtr::Ptr(p) = *self;
//    assert_ne!(p, ptr::null());
    LTerm::from_raw(*p.offset(1))
  }
}
