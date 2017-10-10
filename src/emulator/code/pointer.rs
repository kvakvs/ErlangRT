//! Module defines pointer types for readonly code and mutable code.
use defs::Word;
use defs::TAG_CP;

#[derive(Copy, Clone)]
pub enum CodePtr { Ptr(*const Word) }

impl CodePtr {
  pub fn null() -> CodePtr {
    CodePtr::Ptr(0 as *const Word)
  }


  /// Convert to tagged CP integer
  pub fn to_cp(&self) -> Word {
    let CodePtr::Ptr(p) = *self;
    let p1 = p as Word;
    p1 | TAG_CP
  }


  pub fn offset(&self, n: isize) -> CodePtr {
    let CodePtr::Ptr(p) = *self;
    let new_p = unsafe { p.offset(n) };
    CodePtr::Ptr(new_p)
  }

  //  pub fn is_null(&self) -> bool {
  //    let CodePtr::Ptr(p) = *self;
  //    p == 0 as *const Word
  //  }
}

//pub enum CodePtrMut { Ptr(*mut Word) }
