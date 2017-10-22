//! Module defines pointer types for readonly code and mutable code.
use defs::Word;
use defs::TAG_CP;
use term::immediate;
use term::lterm::LTerm;

use std::fmt;

/// Pointer to code location, can only be created to point to some opcode
/// (instruction begin), and never to the data. During VM execution iterates
/// over args too, and no extra checks are made.
///
/// In debug build additional mark bits `Imm3::OPCODE` are added to this word
/// and additional check is done here in `CodePtr`.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum CodePtr { Ptr(*const Word) }


impl fmt::Display for CodePtr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let CodePtr::Ptr(p) = *self;
    write!(f, "CodePtr(0x{:x})", p as Word)
  }
}


impl CodePtr {

  #[inline]
  pub fn get_ptr(&self) -> *const Word {
    let CodePtr::Ptr(p) = *self;
    p
  }


  #[inline]
  pub fn from_cp(cp: LTerm) -> CodePtr {
    CodePtr::from_ptr(cp.cp_get_ptr())
  }

  #[cfg(debug_assertions)]
  #[inline]
  pub fn from_ptr(p: *const Word) -> CodePtr {
    unsafe {
      // An extra unsafe safety check, this will fail if codeptr points to
      // a random garbage
      assert!(immediate::is_immediate3(*p),
              "A CodePtr must always point to an imm3 tagged opcode");
    }
    CodePtr::Ptr(p)
  }

  #[cfg(not(debug_assertions))]
  pub fn from_ptr(p: *const Word) -> CodePtr {
    CodePtr::Ptr(p)
  }


  #[inline]
  pub fn null() -> CodePtr {
    CodePtr::Ptr(::std::ptr::null())
  }


  /// Convert to tagged CP integer
  #[inline]
  pub fn to_cp(&self) -> Word {
    let CodePtr::Ptr(p) = *self;
    let p1 = p as Word;
    p1 | TAG_CP
  }


  #[inline]
  pub fn offset(&self, n: isize) -> CodePtr {
    let CodePtr::Ptr(p) = *self;
    let new_p = unsafe { p.offset(n) };
    CodePtr::Ptr(new_p)
  }


  #[inline]
  pub fn is_null(&self) -> bool {
    let CodePtr::Ptr(p) = *self;
    p.is_null()
  }


//  #[inline]
//  pub fn is_not_null(&self) -> bool { ! self.is_null() }
}

/// A mutable code pointer for walking the code and modifying the values.
/// See `emulator::code::iter` for iterators.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum CodePtrMut { Ptr(*mut Word) }

impl CodePtrMut {

  /// Read word at the code pointer.
  pub unsafe fn read_0(&self) -> Word {
    let CodePtrMut::Ptr(p) = *self;
    *p
  }


  /// Read `n`-th word from code pointer.
  pub unsafe fn read_n(&self, n: isize) -> Word {
    let CodePtrMut::Ptr(p) = *self;
    *(p.offset(n))
  }


  /// Write `n`-th word at the code pointer.
  pub unsafe fn write_n(&self, n: isize, val: Word) {
    let CodePtrMut::Ptr(p) = *self;
    *(p.offset(n)) = val
  }
}
