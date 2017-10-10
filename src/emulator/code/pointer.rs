//! Module defines pointer types for readonly code and mutable code.
use defs::Word;
use defs::TAG_CP;
use term::immediate;

/// Pointer to code location, can only be created to point to some opcode
/// (instruction begin), and never to the data. During VM execution iterates
/// over args too, and no extra checks are made.
///
/// In debug build additional mark bits `Imm3::OPCODE` are added to this word
/// and additional check is done here in `CodePtr`.
#[derive(Copy, Clone)]
pub enum CodePtr { Ptr(*const Word) }

impl CodePtr {

  #[cfg(debug_assertions)]
  pub fn from_ptr(p: *const Word) -> CodePtr {
    unsafe { assert!(immediate::is_immediate3(*p)); }
    CodePtr::Ptr(p)
  }

  #[cfg(not(debug_assertions))]
  pub fn from_ptr(p: *const Word) -> CodePtr {
    CodePtr::Ptr(p)
  }


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
