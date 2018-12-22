//! Module defines pointer types for readonly code and mutable code.

use crate::{defs::Word, emulator::module::VersionedModuleName, term::lterm::*};
use core::fmt;

/// A cross-module code pointer tied to a specific module of a specific version.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct VersionedCodePtr {
  pub versioned_name: VersionedModuleName,
  pub ptr: CodePtr,
}

#[allow(dead_code)]
impl VersionedCodePtr {
  pub fn new(name: VersionedModuleName, ptr: CodePtr) -> VersionedCodePtr {
    VersionedCodePtr {
      versioned_name: name,
      ptr,
    }
  }

  //  #[inline]
  //  pub fn code_ptr(self: VersionedCodePtr, code_server: &CodeServer) -> CodePtr {
  //     TODO: assumes the result will contain the value and not panic instead
  //    code_server.lookup_far_pointer(self).unwrap()
  //  }
}

/// Pointer to code location, can only be created to point to some opcode
/// (instruction begin), and never to the data. During VM execution iterates
/// over args too, and no extra checks are made.
///
/// In debug build additional mark bits `Imm3::OPCODE` are added to this word
/// and additional check is done here in `CodePtr`.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub struct CodePtr(*const Word);

impl fmt::Display for CodePtr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CodePtr(0x{:x})", self.get() as Word)
  }
}

impl CodePtr {
  pub fn new<T>(p: *const T) -> CodePtr {
    // assert_ne!(p as Word, 0xcea);
    CodePtr(p as *const Word)
  }

  #[inline]
  pub fn get(self) -> *const Word {
    let CodePtr(p) = self;
    p
  }

  #[inline]
  pub fn from_cp(cp: LTerm) -> CodePtr {
    CodePtr::from_ptr(cp.get_cp_ptr())
  }

  #[cfg(debug_assertions)]
  #[inline]
  pub fn from_ptr(p: *const LTerm) -> CodePtr {
    unsafe {
      // An extra unsafe safety check, this will fail if codeptr points to
      // a random garbage. Or may be a null.
      assert!(
        p.is_null() || (*p).get_term_tag() == TERMTAG_SPECIAL,
        "A CodePtr must be null or point to an imm3 tagged opcode"
      );
    }
    CodePtr::new(p)
  }

  #[cfg(not(debug_assertions))]
  pub fn from_ptr(p: *const Word) -> CodePtr {
    CodePtr(p)
  }

  #[inline]
  pub fn null() -> CodePtr {
    CodePtr::new::<Word>(::core::ptr::null())
  }

  /// Convert to tagged CP integer
  #[inline]
  pub fn to_cp(self) -> Word {
    LTerm::make_cp(self.get()).raw()
  }

  //  #[inline]
  //  pub fn offset(&self, n: isize) -> CodePtr {
  //    let CodePtr(p) = *self;
  //    let new_p = unsafe { p.offset(n) };
  //    CodePtr(new_p)
  //  }

  #[inline]
  pub fn is_null(self) -> bool {
    self.get().is_null()
  }

  //  #[inline]
  //  pub fn is_not_null(&self) -> bool { ! self.is_null() }

  pub fn belongs_to(self, slice: &[Word]) -> bool {
    let cbegin = &slice[0] as *const Word;
    let cend = unsafe { cbegin.add(slice.len()) };
    let p = self.get();
    p >= cbegin && p < cend
  }
}

/// A mutable code pointer for walking the code and modifying the values.
/// See `emulator::code::iter` for iterators.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct CodePtrMut(pub *mut Word);

impl CodePtrMut {
  /// Quick access to the contained pointer.
  #[inline]
  pub fn ptr(self) -> *const Word {
    let CodePtrMut(p) = self;
    p
  }

  /// Read word at the code pointer.
  #[inline]
  #[allow(dead_code)]
  pub unsafe fn read_0(self) -> Word {
    let CodePtrMut(p) = self;
    *p
  }

  /// Read `n`-th word from code pointer.
  #[inline]
  pub unsafe fn read_n(self, n: usize) -> Word {
    let CodePtrMut(p) = self;
    core::ptr::read(p.add(n))
  }

  /// Write `n`-th word at the code pointer.
  #[inline]
  pub unsafe fn write_n(self, n: usize, val: Word) {
    let CodePtrMut(p) = self;
    core::ptr::write(p.add(n), val)
  }
}
