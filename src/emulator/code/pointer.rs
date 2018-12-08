//! Module defines pointer types for readonly code and mutable code.

use crate::emulator::code_srv::module_id::VersionedModuleId;
use crate::emulator::code_srv::CodeServer;
use crate::rt_defs::Word;
use crate::term::lterm::*;

use core::fmt;


/// A cross-module code pointer tied to a specific module of a specific version.
/// Versions are maintained by the Code Server.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct FarCodePointer {
  pub mod_id: VersionedModuleId,
  pub offset: usize,
}


#[allow(dead_code)]
impl FarCodePointer {
  pub fn new(mod_id: &VersionedModuleId, offset: usize) -> FarCodePointer {
    FarCodePointer {
      mod_id: *mod_id,
      offset,
    }
  }

  #[inline]
  pub fn code_ptr(self: FarCodePointer, code_server: &CodeServer) -> CodePtr {
    // TODO: assumes the result will contain the value and not panic instead
    code_server.lookup_far_pointer(self).unwrap()
  }
}


/// Pointer to code location, can only be created to point to some opcode
/// (instruction begin), and never to the data. During VM execution iterates
/// over args too, and no extra checks are made.
///
/// In debug build additional mark bits `Imm3::OPCODE` are added to this word
/// and additional check is done here in `CodePtr`.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
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
  pub fn is_null(&self) -> bool {
    self.get().is_null()
  }


  //  #[inline]
  //  pub fn is_not_null(&self) -> bool { ! self.is_null() }

  pub fn belongs_to(self, slice: &[Word]) -> bool {
    let cbegin = &slice[0] as *const Word;
    let cend = unsafe { cbegin.offset(slice.len() as isize) };
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
  pub unsafe fn read_n(self, n: isize) -> Word {
    let CodePtrMut(p) = self;
    *(p.offset(n))
  }


  /// Write `n`-th word at the code pointer.
  #[inline]
  pub unsafe fn write_n(self, n: isize, val: Word) {
    let CodePtrMut(p) = self;
    *(p.offset(n)) = val
  }
}
