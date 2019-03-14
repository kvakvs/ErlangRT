//! Module defines pointer types for readonly code and mutable code.

use crate::{defs::Word, emulator::module::VersionedModuleName, term::value::*};
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
/// In debug build additional mark bits `TERMTAG_SPECIAL`, and then
/// `SPECIALTAG_OPCODE` are added to this word and additional check is done
/// here in `CodePtr`.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub struct CodePtr {
  p: *const Word,
}

impl fmt::Display for CodePtr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CodePtr(0x{:x})", self.p as Word)
  }
}

impl CodePtr {
  #[allow(dead_code)]
  pub fn unsafe_new(p: *const Word) -> Self {
    Self { p }
  }

  #[inline]
  pub fn get_pointer(self) -> *const Word {
    self.p as *const Word
  }

  #[inline]
  pub fn from_cp(cp: Term) -> CodePtr {
    CodePtr::from_ptr(cp.get_cp_ptr())
  }

  #[inline]
  fn assert_location_is_opcode(p0: *const Word) {
    let p = p0 as *const Term;
    unsafe {
      // An extra unsafe safety check, this will fail if codeptr points to
      // a random garbage. Or may be a null.
      debug_assert!(
        p.is_null() || (*p).is_special(),
        "A CodePtr must be null or point to a Special-tagged term (opcode)"
      );
    }
  }

  #[inline]
  pub fn from_ptr(p0: *const Word) -> CodePtr {
    Self::assert_location_is_opcode(p0);
    CodePtr {
      p: p0 as *const Word,
    }
  }

  #[inline]
  pub fn null() -> CodePtr {
    CodePtr {
      p: core::ptr::null(),
    }
  }

  /// Convert to tagged CP integer
  #[inline]
  pub fn to_cp_term(self) -> Term {
    Term::make_cp(self.p)
  }

  #[inline]
  pub fn is_null(self) -> bool {
    self.p.is_null()
  }

  pub fn belongs_to(self, slice: &[Word]) -> bool {
    let cbegin = &slice[0] as *const Word;
    let cend = unsafe { cbegin.add(slice.len()) };
    let p = self.get_pointer();
    p >= cbegin && p < cend
  }

  /// Shift the pointer forward or back (no checking of contents)
  #[inline]
  pub fn offset(&mut self, n: isize) {
    self.p = unsafe { self.p.offset(n) };
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
