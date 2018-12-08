//! Low level term library
//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance
//!
//! Do not import this file directly, use `use term::lterm::*;` instead.

use crate::emulator::atom;
use crate::emulator::heap::Heap;
use crate::fail::{Error, RtResult};
use crate::rt_defs::*;
use crate::term::boxed;
use crate::term::boxed::{BoxHeader, BoxTypeTag};
use core::cmp::Ordering;
use core::fmt;
use core::isize;
use core::ptr;

//
// Structure of term:
// [ Value or a pointer ] [ TAG_* value 3 bits ]
//

pub const TERM_TAG_BITS: Word = 3;
pub const TERM_TAG_MASK: Word = (1 << TERM_TAG_BITS) - 1;

/// Max value for a positive small integer packed into immediate2 low level
/// Term. Assume word size minus 4 bits for imm1 tag and 1 for sign
pub const SMALLEST_SMALL: SWord = isize::MIN >> TERM_TAG_BITS;
pub const LARGEST_SMALL: SWord = isize::MAX >> TERM_TAG_BITS;

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct TermTag(pub Word);

pub const TERMTAG_BOXED: TermTag = TermTag(0);
pub const TERMTAG_HEADER: TermTag = TermTag(1);
pub const TERMTAG_CONS: TermTag = TermTag(2);
// From here and below, values are immediate (fit into a single word)
pub const TERMTAG_SMALL: TermTag = TermTag(3);
pub const TERMTAG_ATOM: TermTag = TermTag(4);
pub const TERMTAG_LOCALPID: TermTag = TermTag(5);
pub const TERMTAG_LOCALPORT: TermTag = TermTag(6);
pub const TERMTAG_SPECIAL: TermTag = TermTag(7);

//
// Structure of SPECIAL values,
// they are plethora of term types requiring fewer bits or useful in other ways
// [ special value ] [ VAL_SPECIAL_... 3 bits ] [ TAG_SPECIAL 3 bits ]
//
pub const TERM_SPECIAL_TAG_BITS: Word = 3;
pub const TERM_SPECIAL_TAG_MASK: Word = (1 << TERM_SPECIAL_TAG_BITS) - 1;

#[derive(Eq, PartialEq, Debug)]
pub struct SpecialTag(pub Word);

// special constants such as NIL, empty tuple, binary etc
pub const SPECIALTAG_CONST: SpecialTag = SpecialTag(0);
pub const SPECIALTAG_REGX: SpecialTag = SpecialTag(1);
pub const SPECIALTAG_REGY: SpecialTag = SpecialTag(2);
pub const SPECIALTAG_REGFP: SpecialTag = SpecialTag(3);
// decorates opcodes for easier code walking
pub const SPECIALTAG_OPCODE: SpecialTag = SpecialTag(4);


pub struct SpecialConst(pub Word);

pub const SPECIALCONST_EMPTYTUPLE: SpecialConst = SpecialConst(0);
pub const SPECIALCONST_EMPTYLIST: SpecialConst = SpecialConst(1);
pub const SPECIALCONST_EMPTYBINARY: SpecialConst = SpecialConst(2);


/// A low-level term is either a pointer to memory term or an Immediate with
/// leading bits defining its type (see TAG_* consts below).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct LTerm {
  value: Word, // Contains a pointer or an integer
}


impl Ord for LTerm {
  fn cmp(&self, other: &LTerm) -> Ordering {
    self.value.cmp(&other.value)
  }
}


impl PartialOrd for LTerm {
  fn partial_cmp(&self, other: &LTerm) -> Option<Ordering> {
    Some(self.value.cmp(&other.value))
  }
}


// TODO: Remove deadcode directive later and fix
#[allow(dead_code)]
impl LTerm {
  /// Retrieve the raw value of a `LTerm` as Word, including tag bits
  /// and everything.
  #[inline]
  pub const fn raw(self) -> Word {
    self.value
  }


  #[inline]
  pub const fn make_atom(id: Word) -> LTerm {
    LTerm::make_from_tag_and_value(TERMTAG_ATOM, id)
  }


  #[inline]
  pub fn make_cp<T>(p: *const T) -> LTerm {
    assert_eq!(p as Word & TERM_TAG_MASK, 0); // must be aligned to 8
    let tagged_p = (p as Word) | HIGHEST_BIT_CP;
    LTerm::from_raw(tagged_p)
  }


  #[inline]
  pub const fn empty_tuple() -> LTerm {
    LTerm::make_special(SPECIALTAG_CONST, SPECIALCONST_EMPTYTUPLE.0)
  }


  #[inline]
  pub const fn empty_binary() -> LTerm {
    LTerm::make_special(SPECIALTAG_CONST, SPECIALCONST_EMPTYBINARY.0)
  }


  #[inline]
  pub const fn nil() -> LTerm {
    LTerm::make_special(SPECIALTAG_CONST, SPECIALCONST_EMPTYLIST.0)
  }


  pub const fn make_from_tag_and_value(t: TermTag, v: Word) -> LTerm {
    LTerm::from_raw(v << TERM_TAG_BITS | t.0)
  }

  pub const fn make_from_tag_and_signed_value(t: TermTag, v: SWord) -> LTerm {
    LTerm::from_raw((v << TERM_TAG_BITS | (t.0 as SWord)) as Word)
  }


  // TODO: Some safety checks maybe? But oh well
  #[inline]
  pub fn make_boxed<T>(p: *const T) -> LTerm {
    LTerm { value: p as Word }
  }


  /// Create a NON_VALUE.
  pub const fn non_value() -> LTerm {
    LTerm { value: 0 }
  }


  /// Check whether a value is a NON_VALUE.
  pub const fn is_non_value(self) -> bool {
    self.value == 0
  }


  /// Check whether a value is NOT a NON_VALUE.
  pub const fn is_value(self) -> bool {
    !self.is_non_value()
  }


  /// Get tag bits from the p field as integer.
  #[inline]
  pub const fn get_term_tag(self) -> TermTag {
    TermTag(self.raw() & TERM_TAG_MASK)
  }


  /// Check whether tag bits of a value equal to TAG_BOXED=0
  #[inline]
  pub fn is_boxed(self) -> bool {
    self.get_term_tag() == TERMTAG_BOXED
  }


  #[inline]
  pub fn get_box_ptr<T>(self) -> *const T {
    assert!(self.is_boxed());
    self.value as *const T
  }


  #[inline]
  pub fn get_box_ptr_mut<T>(self) -> *mut T {
    assert!(self.is_boxed());
    self.value as *mut T
  }


  pub fn get_box_ptr_safe<T>(self) -> RtResult<*const T> {
    if !self.is_boxed() {
      return Err(Error::TermIsNotABoxed);
    }
    Ok(self.value as *const T)
  }


  pub fn get_box_ptr_safe_mut<T>(self) -> RtResult<*mut T> {
    if !self.is_boxed() {
      return Err(Error::TermIsNotABoxed);
    }
    Ok(self.value as *mut T)
  }


  pub fn is_binary(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_BINARY)
  }


  pub fn is_immediate(self) -> bool {
    self.get_term_tag() != TERMTAG_BOXED
  }


  /// Check whether the value is tagged as atom
  pub fn is_atom(self) -> bool {
    self.get_term_tag() == TERMTAG_ATOM
  }


  pub fn is_local_pid(self) -> bool {
    self.get_term_tag() == TERMTAG_LOCALPID
  }


  /// Check whether a lterm is boxed and then whether it points to a word of
  /// memory tagged as external pid
  pub fn is_external_pid(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_EXTERNALPID)
  }


  #[inline]
  fn is_boxed_of_type(self, t: BoxTypeTag) -> bool {
    if !self.is_boxed() {
      return false;
    }
    let p = self.get_box_ptr::<BoxHeader>();
    unsafe { (*p).get_tag() == t }
  }


  /// Return true if a value's tag will fit into a single word
  pub fn is_internal_immediate(self) -> bool {
    self.get_term_tag() == TERMTAG_SPECIAL
  }


  /// For non-pointer Term types get the encoded integer without tag bits
  #[inline]
  pub fn get_term_val_without_tag(self) -> Word {
    debug_assert!(self.get_term_tag() != TERMTAG_BOXED && self.get_term_tag() != TERMTAG_CONS);
    self.value >> TERM_TAG_BITS
  }

  //
  // Construction
  //

  /// Any raw word becomes a term, possibly invalid
  pub const fn from_raw(w: Word) -> LTerm {
    LTerm { value: w }
  }


  pub fn make_local_pid(pindex: Word) -> LTerm {
    LTerm::make_from_tag_and_value(TERMTAG_LOCALPID, pindex)
  }


  pub fn make_remote_pid(hp: &mut Heap, node: LTerm, pindex: Word) -> RtResult<LTerm> {
    let rpid_ptr = boxed::ExternalPid::create_into(hp, node, pindex)?;
    Ok(LTerm::make_boxed(rpid_ptr))
  }


  /// For a special-tagged term extract its special tag
  pub fn get_special_tag(self) -> SpecialTag {
    debug_assert_eq!(self.get_term_tag(), TERMTAG_SPECIAL);
    // cut away term tag bits and extract special tag
    SpecialTag((self.value >> TERM_TAG_BITS) & TERM_SPECIAL_TAG_MASK)
  }


  /// From a special-tagged term extract its value
  pub fn get_special_value(self) -> Word {
    debug_assert_eq!(self.get_term_tag(), TERMTAG_SPECIAL);
    // cut away term tag bits and special tag, extract the remaining value bits
    self.value >> (TERM_TAG_BITS + TERM_SPECIAL_TAG_BITS)
  }


  pub const fn make_special(special_t: SpecialTag, val: Word) -> LTerm {
    LTerm::make_from_tag_and_value(TERMTAG_SPECIAL, val << TERM_SPECIAL_TAG_BITS | special_t.0)
  }


  pub fn make_xreg(n: Word) -> LTerm {
    LTerm::make_special(SPECIALTAG_REGX, n)
  }


  pub fn make_yreg(n: Word) -> LTerm {
    LTerm::make_special(SPECIALTAG_REGY, n)
  }

  pub fn make_fpreg(n: Word) -> LTerm {
    LTerm::make_special(SPECIALTAG_REGFP, n)
  }


  #[inline]
  pub fn is_cp(self) -> bool {
    debug_assert!(self.is_boxed());
    self.value & HIGHEST_BIT_CP == HIGHEST_BIT_CP
  }


  pub fn get_cp_ptr<T>(self) -> *const T {
    debug_assert_eq!(self.value & HIGHEST_BIT_CP, HIGHEST_BIT_CP);
    (self.value & (HIGHEST_BIT_CP - 1)) as *const T
  }


  //
  // Tuples =========================
  //

  pub fn is_tuple(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_TUPLE)
  }

  //
  // Lists/Cons cells =========================
  //

  #[inline]
  pub fn is_list(self) -> bool {
    self.is_cons() || self == LTerm::nil()
  }

  /// Check whether the value is a CONS pointer
  #[inline]
  pub fn is_cons(self) -> bool {
    self.get_term_tag() == TERMTAG_CONS
  }


  #[inline]
  pub fn get_cons_ptr(self) -> *const boxed::Cons {
    debug_assert!(self.is_cons());
    (self.value & (!TERM_TAG_MASK)) as *const boxed::Cons
  }


  pub fn get_cons_ptr_mut(self) -> *mut boxed::Cons {
    debug_assert!(self.is_cons());
    (self.value & (!TERM_TAG_MASK)) as *mut boxed::Cons
  }


  /// Create a LTerm from pointer to Cons cell. Pass a pointer to `LTerm` or
  /// a pointer to `boxed::Cons`.
  #[inline]
  pub fn make_cons<T>(p: *const T) -> LTerm {
    LTerm {
      value: (p as Word) | TERMTAG_CONS.0,
    }
  }


  unsafe fn cons_is_ascii_string(&self) -> bool {
    // TODO: List iterator
    let mut cons_p = self.get_cons_ptr();
    loop {
      let hd = (*cons_p).hd();
      if !hd.is_small() {
        return false;
      }

      let hd_value = hd.get_small_signed();
      if hd_value < 32 || hd_value >= 127 {
        return false;
      }

      let tl = (*cons_p).tl();
      if !tl.is_cons() {
        // NIL [] tail is required for a true string
        return tl == LTerm::nil();
      }
      cons_p = tl.get_cons_ptr();
    }
  }

  //
  // Small Integers =========================
  //

  /// Check whether the value is a small integer
  pub fn is_small(self) -> bool {
    self.get_term_tag() == TERMTAG_SMALL
  }


  pub const fn make_small_unsigned(val: Word) -> LTerm {
    LTerm::make_from_tag_and_value(TERMTAG_SMALL, val)
  }


  pub const fn make_small_signed(val: SWord) -> LTerm {
    LTerm::make_from_tag_and_signed_value(TERMTAG_SMALL, val)
  }


  #[inline]
  pub fn small_fits(val: isize) -> bool {
    val >= SMALLEST_SMALL && val <= LARGEST_SMALL
  }


  pub const fn get_small_signed(self) -> SWord {
    (self.value as SWord) >> TERM_TAG_BITS
  }


  #[inline]
  pub fn get_small_unsigned(self) -> Word {
    self.get_term_val_without_tag()
  }

  //
  // Big Integers ==============================
  //

  /// Check whether a lterm is boxed and then whether it points to a word of
  /// memory tagged as float
  pub fn is_big_int(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_BIGINTEGER)
  }


  //
  // Atoms ==============================
  //

  pub fn atom_index(self) -> Word {
    debug_assert!(self.is_atom());
    return self.get_term_val_without_tag();
  }


  //
  // Float ==============================
  //

  /// Check whether a lterm is boxed and then whether it points to a word of
  /// memory tagged as float
  pub fn is_float(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_FLOAT)
  }

  pub fn get_f64(self) -> RtResult<f64> {
    if !self.is_boxed() {
      return Err(Error::TermIsNotABoxed);
    }
    let _p = self.get_box_ptr::<BoxHeader>();
    panic!("notimpl: float box")
  }


  /// Returns float value, performs no extra checks. The caller is responsible
  /// for the value being a boxed float.
  #[inline]
  pub unsafe fn get_f64_unsafe(self) -> f64 {
    let p = self.get_box_ptr::<boxed::Float>();
    (*p).value
  }

  //
  // Formatting helpers
  //

  fn format_special(self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.get_special_tag() {
      SPECIALTAG_CONST => {
        if self == LTerm::nil() {
          return write!(f, "[]");
        } else if self.is_non_value() {
          return write!(f, "NON_VALUE");
        } else if self == LTerm::empty_binary() {
          return write!(f, "<<>>");
        } else if self == LTerm::empty_tuple() {
          return write!(f, "{{}}");
        }
      }
      SPECIALTAG_REGX => return write!(f, "X{}", self.get_special_value()),
      SPECIALTAG_REGY => return write!(f, "Y{}", self.get_special_value()),
      SPECIALTAG_REGFP => return write!(f, "F{}", self.get_special_value()),
      SPECIALTAG_OPCODE => return write!(f, "Opcode({})", self.get_special_value()),
      _ => {}
    }
    write!(
      f,
      "Special(0x{:x}; 0x{:x})",
      self.get_special_tag().0,
      self.get_special_value()
    )
  }


  /// Attempt to display contents of a tagged header word and the words which
  /// follow it. Arg `p` if not null is used to fetch the following memory words
  /// and display more detail.
  fn format_header(
    value_at_ptr: LTerm,
    val_ptr: *const Word,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    //    let arity = boxed::headerword_to_arity(value_at_ptr.raw());
    let h_tag = boxed::headerword_to_boxtype(value_at_ptr.raw());

    match h_tag {
      boxed::BOXTYPETAG_BINARY => write!(f, "Bin"),
      boxed::BOXTYPETAG_TUPLE => unsafe { LTerm::format_tuple(val_ptr, f) },
      boxed::BOXTYPETAG_CLOSURE => write!(f, "Fun"),
      boxed::BOXTYPETAG_FLOAT => write!(f, "Float"),
      boxed::BOXTYPETAG_EXTERNALPID => write!(f, "ExtPid"),
      boxed::BOXTYPETAG_EXTERNALPORT => write!(f, "ExtPort"),
      boxed::BOXTYPETAG_EXTERNALREF => write!(f, "ExtRef"),

      _ => panic!("Unexpected header tag {:?}", h_tag),
    }
  }


  /// Given `p`, a pointer to tuple header word, format tuple contents.
  unsafe fn format_tuple(p: *const Word, f: &mut fmt::Formatter) -> fmt::Result {
    let tptr = match boxed::Tuple::from_pointer(p) {
      Ok(x) => x,
      Err(e) => return write!(f, "<err formatting tuple: {:?}>", e),
    };

    write!(f, "{{")?;

    let arity = boxed::Tuple::get_arity(tptr);
    for i in 0..arity {
      write!(f, "{}", boxed::Tuple::get_element_base0(tptr, i))?;
      if i < arity - 1 {
        write!(f, ", ")?
      }
    }
    write!(f, "}}")
  }


  pub unsafe fn format_cons(self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[")?;

    let mut raw_cons = self.get_cons_ptr();
    loop {
      write!(f, "{}", (*raw_cons).hd())?;
      let tl = (*raw_cons).tl();
      if tl == LTerm::nil() {
        // Proper list ends here, do not show the tail
        break;
      } else if tl.is_cons() {
        // List continues, print a comma and follow the tail
        write!(f, ", ")?;
        raw_cons = tl.get_cons_ptr();
      } else {
        // Improper list, show tail
        write!(f, "| {}", tl)?;
        break;
      }
    }
    write!(f, "]")
  }


  pub unsafe fn format_cons_ascii(self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\"")?;

    let mut raw_cons = self.get_cons_ptr();
    loop {
      write!(f, "{}", (*raw_cons).hd().get_small_unsigned() as u8 as char)?;
      let tl = (*raw_cons).tl();
      if tl == LTerm::nil() {
        // Proper list ends here, do not show the tail
        break;
      } else if tl.is_cons() {
        // List continues, follow the tail
        raw_cons = tl.get_cons_ptr();
      } else {
        // Improper list, must not happen because we checked for proper NIL
        // tail in cons_is_ascii_string. Let's do some panic!
        panic!("Printing an improper list as ASCII string")
      }
    }
    write!(f, "\"")
  }


  /// Raw compare two term values.
  pub fn is_same(a: LTerm, b: LTerm) -> bool {
    a.raw() == b.raw()
  }

  //
  // PORT ===========
  //
  /// Check whether a value is any kind of port.
  pub fn is_port(&self) -> bool {
    self.is_local_port() || self.is_external_port()
  }

  pub fn is_local_port(&self) -> bool {
    false
  }

  pub fn is_external_port(&self) -> bool {
    false
  }

  //
  // MAP ===============
  //
  /// Check whether a value is a map.
  pub fn is_map(&self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_MAP)
  }

  /// Check whether a value is a small map < 32 elements (Flat). Does NOT check
  /// that the value is a map (assert!) assuming that the caller has checked
  /// it by now.
  pub fn is_flat_map(&self) -> bool {
    false
  }

  /// Check whether a value is a hash map >= 32 elements (HAMT). Does NOT check
  /// that the value is a map (assert!) assuming that the caller has checked
  /// it by now.
  pub fn is_hash_map(&self) -> bool {
    false
  }

  pub fn map_size(&self) -> usize {
    0
  }

  //
  // EXPORT ==================
  //
  /// Check whether a value is a boxed export (M:F/Arity triple).
  pub fn is_export(&self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_EXPORT)
  }

  //
  // FUN / CLOSURE ==================
  //

  /// Check whether a value is a boxed fun (a closure).
  pub fn is_fun(&self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_CLOSURE)
  }

  //
  // REFERENCE
  //
  /// Check whether a value is any kind of reference.
  pub fn is_ref(&self) -> bool {
    self.is_local_ref() || self.is_external_ref()
  }

  pub fn is_local_ref(&self) -> bool {
    false
  }

  pub fn is_external_ref(&self) -> bool {
    false
  }

}


// Printing low_level Terms as "{}"
impl fmt::Display for LTerm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.get_term_tag() {
      TERMTAG_BOXED => unsafe {
        if self.is_cp() {
          write!(f, "CP({:p})", self.get_cp_ptr::<Word>())
        } else {
          let p = self.get_box_ptr();
          LTerm::format_header(*p, p as *const Word, f)
        }
      },

      TERMTAG_CONS => unsafe {
        if self.cons_is_ascii_string() {
          self.format_cons_ascii(f)
        } else {
          self.format_cons(f)
        }
      },
      TERMTAG_SMALL => write!(f, "{}", self.get_small_signed()),
      TERMTAG_SPECIAL => self.format_special(f),
      TERMTAG_LOCALPID => write!(f, "LocalPid({})", self.get_term_val_without_tag()),
      TERMTAG_LOCALPORT => write!(f, "LocalPort({})", self.get_term_val_without_tag()),
      TERMTAG_ATOM => match atom::to_str(*self) {
        Ok(s) => write!(f, "'{}'", s),
        Err(_e) => write!(f, "Atom?"),
      },
      TERMTAG_HEADER => {
        write!(f, "Header(")?;
        LTerm::format_header(*self, ptr::null(), f)?;
        write!(f, ")")
      }

      _ => panic!("Primary tag {:?} not recognized", self.get_term_tag()),
    }
  }
} // trait Display


//
// Testing section
//

//#[cfg(test)]
//mod tests {
//  use core::ptr;
//  use std::mem;
//
//  use rt_defs::*;
//  use super::*;
//  use term::lterm::aspect_smallint::*;
//
//  #[test]
//  fn test_nil_is_not_atom() {
//    // Some obscure bit mishandling made nil be recognized as atom
//    let n = LTerm::nil();
//    assert!(!n.is_atom(), "must not be an atom {} 0x{:x} imm2_pfx 0x{:x}, imm2atompfx 0x{:x}",
//            n, n.raw(), immediate::get_imm2_prefix(n.raw()),
//            immediate::IMM2_ATOM_PREFIX);
//  }
//
//  #[test]
//  fn test_term_size() {
//    assert_eq!(mem::size_of::<LTerm>(), WORD_BYTES);
//  }
//
//  #[test]
//  fn test_small_unsigned() {
//    let s1 = make_small_u(1);
//    assert_eq!(1, s1.get_small_unsigned());
//
//    let s2 = make_small_u(MAX_UNSIGNED_SMALL);
//    assert_eq!(MAX_UNSIGNED_SMALL, s2.get_small_unsigned());
//  }
//
//
//  #[test]
//  fn test_small_signed_1() {
//    let s2 = make_small_s(1);
//    let s2_out = s2.get_small_signed();
//    assert_eq!(1, s2_out, "Expect 1, have 0x{:x}", s2_out);
//
//    let s1 = make_small_s(-1);
//    let s1_out = s1.get_small_signed();
//    assert_eq!(-1, s1_out, "Expect -1, have 0x{:x}", s1_out);
//  }
//
//
//  #[test]
//  fn test_small_signed_limits() {
//    let s2 = make_small_s(MAX_POS_SMALL);
//    assert_eq!(MAX_POS_SMALL, s2.get_small_signed());
//
//    let s3 = make_small_s(MIN_NEG_SMALL);
//    assert_eq!(MIN_NEG_SMALL, s3.get_small_signed());
//  }
//
//
//  #[test]
//  fn test_cp() {
//    let s1 = make_cp(ptr::null());
//    assert_eq!(s1.cp_get_ptr(), ptr::null());
//  }
//}


pub unsafe fn helper_get_const_from_boxed_term<T>(
  t: LTerm,
  box_type: BoxTypeTag,
  err: Error,
) -> RtResult<*const T> {
  if !t.is_boxed() {
    return Err(Error::TermIsNotABoxed);
  }
  let cptr = t.get_box_ptr::<T>();
  let hptr = cptr as *const BoxHeader;
  if (*hptr).get_tag() != box_type {
    return Err(err);
  }
  Ok(cptr)
}


pub unsafe fn helper_get_mut_from_boxed_term<T>(
  t: LTerm,
  box_type: BoxTypeTag,
  _err: Error,
) -> RtResult<*mut T> {
  debug_assert!(t.is_boxed());
  let cptr = t.get_box_ptr_mut::<T>();
  let hptr = cptr as *const BoxHeader;
  debug_assert_eq!((*hptr).get_tag(), box_type);
  Ok(cptr)
}
