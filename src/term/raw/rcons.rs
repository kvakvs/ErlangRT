use defs::Word;
use term::lterm::LTerm;

pub struct RawCons {
  p: *mut Word,
}

impl RawCons {
  pub fn from_pointer(p: *mut Word) -> RawCons {
    RawCons { p }
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
}
