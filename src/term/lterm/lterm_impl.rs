//! Low level term library
//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance
//!
//! Do not import this file directly, use `use term::lterm::*;` instead.

use emulator::atom;
use emulator::heap::{IHeap};
use rt_defs::*;
use term::boxed;

use std::cmp::Ordering;
use std::fmt;
use std::ptr;
use fail::{Hopefully,Error};
use term::boxed::BoxTypeTag;


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
  pub fn raw(self) -> Word { self.value }


  #[inline]
  pub const fn make_atom(id: Word) -> LTerm {
    LTerm::make_from_tag_and_value(TermTag::Atom, id)
  }


  #[inline]
  pub const fn make_cp<T>(p: *const T) -> LTerm {
    let tagged_p = (p as Word) | (TermTag::CP as Word);
    LTerm::from_raw(tagged_p)
  }


  #[inline]
  pub const fn empty_tuple() -> LTerm {
    LTerm::make_special(SpecialTag::Const,
                        SpecialConst::EmptyTuple as Word)
  }


  #[inline]
  pub const fn empty_binary() -> LTerm {
    LTerm::make_special(SpecialTag::Const,
                        SpecialConst::EmptyBinary)
  }


  #[inline]
  pub const fn nil() -> LTerm {
    LTerm::make_special(SpecialTag::Const,
                        SpecialConst::EmptyList)
  }


  pub const fn make_from_tag_and_value<VType>(t: TermTag, v: VType) -> LTerm {
    LTerm::from_raw(v << TERM_TAG_BITS | (t as Word))
  }


  // TODO: Some safety checks maybe? But oh well
  pub const fn make_boxed<T>(p: *const T) -> LTerm {
    LTerm { value: p as Word }
  }


  /// Create a NON_VALUE.
  pub const fn non_value() -> LTerm {
    LTerm { value: 0 }
  }


  /// Check whether a value is a NON_VALUE.
  pub const fn is_non_value(self) -> bool {
    self.p.is_null()
  }


  /// Check whether a value is NOT a NON_VALUE.
  #[inline]
  pub fn is_value(self) -> bool {
    ! self.p.is_null()
  }


  /// Get tag bits from the p field as integer.
  #[inline]
  pub const fn get_term_tag(self) -> TermTag {
    self.raw() & TERM_TAG_MASK
  }


  /// Check whether tag bits of a value equal to TAG_BOXED=0
  pub const fn is_boxed(self) -> bool {
    self.get_term_tag() == TermTag::Boxed
  }


  pub fn get_box_ptr<T>(self) -> *const T {
    self.p as *const T
  }


  pub fn get_box_ptr_mut<T>(self) -> *mut T {
    self.p as *mut T
  }


  pub fn is_binary(self) -> bool {
    self.is_boxed() && self.p.t == BoxTypeTag::Binary
  }


  pub fn is_immediate(self) -> bool {
    self.get_term_tag() != TermTag::Boxed
  }


  /// Check whether the value is tagged as atom
  pub fn is_atom(self) -> bool {
    self.get_term_tag() == TermTag::Atom
  }


  pub fn is_local_pid(self) -> bool {
    self.get_term_tag() == TermTag::LocalPid
  }


  /// Return true if a value's tag will fit into a single word
  pub fn is_internal_immediate(self) -> bool {
    self.get_term_tag() == TermTag::Special
  }


  /// For non-pointer Term types get the encoded integer without tag bits
  pub const fn get_term_val_without_tag(self) -> Word {
    debug_assert!(self.get_term_tag() != TermTag::Boxed
        && self.get_term_tag() != TermTag::Cons);
    (self.p as Word) >> TERM_TAG_BITS
  }

  //
  // Construction
  //

  /// Any raw word becomes a term, possibly invalid
  pub fn from_raw(w: Word) -> LTerm {
    LTerm { value: w }
  }


  pub fn make_local_pid(pindex: Word) -> LTerm {
    LTerm::make_from_tag_and_value(TermTag::LocalPid, pindex)
  }


  pub fn make_remote_pid(hp: &mut IHeap,
                         node: LTerm,
                         pindex: Word) -> Hopefully<LTerm> {
    let rpid_ptr = boxed::RemotePid::create_into(hp, node, pindex)?;
    Ok(LTerm::make_boxed(rpid_ptr))
  }


  /// For a special-tagged term extract its special tag
  pub fn get_special_tag(self) -> SpecialTag {
    debug_assert_eq!(self.get_term_tag(), TermTag::Special);
    // cut away term tag bits and extract special tag
    ((self.value >> TERM_TAG_BITS) & TERM_SPECIAL_TAG_MASK) as SpecialTag
  }


  /// From a special-tagged term extract its value
  pub fn get_special_value(self) -> Word {
    debug_assert_eq!(self.get_term_tag(), TermTag::Special);
    // cut away term tag bits and special tag, extract the remaining value bits
    self.value >> (TERM_TAG_BITS + TERM_SPECIAL_TAG_BITS)
  }


  pub const fn make_special(special_t: SpecialTag, val: Word) -> LTerm {
    let special_v = val << TERM_SPECIAL_TAG_BITS | (special_t as Word);
    LTerm::make_from_tag_and_value(TermTag::Special, special_v)
  }


  pub fn make_xreg(n: Word) -> LTerm {
    LTerm::make_special(SpecialTag::RegX, n)
  }


  pub fn make_yreg(n: Word) -> LTerm {
    LTerm::make_special(SpecialTag::RegY, n)
  }

  pub fn make_fpreg(n: Word) -> LTerm {
    LTerm::make_special(SpecialTag::RegFP, n)
  }


  pub const fn is_cp(self) -> bool {
    debug_assert!(self.is_boxed());
    self.value & HIGHEST_BIT_CP == HIGHEST_BIT_CP
  }


  pub fn get_cp_ptr<T>(self) -> *const T {
    debug_assert_eq!(self.value & HIGHEST_BIT_CP, HIGHEST_BIT_CP);
    self.p as *const T
  }


  //
  // Lists/Cons cells
  //

  /// Check whether the value is a CONS pointer
  pub const fn is_cons(self) -> bool {
    self.get_term_tag() == TermTag::Cons
  }

  pub fn get_cons_ptr(self) -> *const boxed::Cons {
    debug_assert!(self.is_cons());
    (self.p & !TERM_TAG_MASK) as *const boxed::Cons
  }


  pub fn get_cons_ptr_mut(self) -> *mut boxed::Cons {
    debug_assert!(self.is_cons());
    (self.p & !TERM_TAG_MASK) as *mut boxed::Cons
  }


  /// Create a LTerm from pointer to Cons cell
  pub const fn make_cons(p: *const boxed::Cons) -> LTerm {
    LTerm { value: (p as Word) | (TermTag::Cons as Word) }
  }

  //
  // Small Integers
  //

  /// Check whether the value is a small integer
  pub const fn is_small(self) -> bool {
    self.get_term_tag() == TermTag::Small
  }


  pub const fn make_small_unsigned(val: Word) -> LTerm {
    LTerm::make_from_tag_and_value(TermTag::Small, val)
  }


  pub const fn make_small_signed(val: SWord) -> LTerm {
    LTerm::make_from_tag_and_value(TermTag::Small, val)
  }


  pub const fn small_fits<T>(val: T) -> bool {
    val >= SMALLEST_SMALL && val <= LARGEST_SMALL
  }


  pub const fn get_small_signed(self) -> SWord {
    (self.value as SWord) >> TERM_TAG_BITS
  }


  pub const fn get_small_unsigned(self) -> Word {
    self.get_term_val_without_tag()
  }

  //
  // Formatting helpers
  //

  fn format_special(self, f: &mut fmt::Formatter) -> fmt::Result {
    if self == LTerm::nil() {
      write!(f, "[]")
    } else if self.is_non_value() {
      write!(f, "NON_VALUE")
    } else if self == LTerm::empty_binary() {
      write!(f, "<<>>")
    } else if self == LTerm::empty_tuple() {
      write!(f, "{{}}")
    } else {
      write!(f, "Special(0x{:x})", self.get_special_value())
    }
//          immediate::TAG_IMM2_IMM3 => {
//            let v3 = immediate::get_imm3_value(v);
//
//            match immediate::get_imm3_tag(v) {
//              immediate::TAG_IMM3_XREG => write!(f, "X({})", v3),
//              immediate::TAG_IMM3_YREG => write!(f, "Y({})", v3),
//              immediate::TAG_IMM3_FPREG => write!(f, "FP({})", v3),
//              immediate::TAG_IMM3_OPCODE => write!(f, "Opcode({})", v3),
//              _ => panic!("Immediate3 tag must be in range 0..3")
//            }
//          }
//          _ => panic!("Immediate2 tag must be in range 0..3")
//        },
//      _ => panic!("Immediate1 tag must be in range 0..3")
//    }
  }


  /// Attempt to display contents of a tagged header word and the words which
  /// follow it. Arg `p` if not null is used to fetch the following memory words
  /// and display more detail.
  fn format_header(value_at_ptr: LTerm,
                   val_ptr: *const Word,
                   f: &mut fmt::Formatter) -> fmt::Result {
    let arity = boxed::headerword_to_arity(value_at_ptr);
    let h_tag = boxed::headerword_to_boxtype(value_at_ptr);

    match h_tag {
      boxed::BoxTypeTag::Binary => write!(f, "Bin"),
      boxed::BoxTypeTag::Tuple => {
        unsafe { LTerm::format_tuple(val_ptr, f) }
      },
      boxed::BoxTypeTag::Closure => write!(f, "Fun"),
      boxed::BoxTypeTag::Float => write!(f, "Float"),
      boxed::BoxTypeTag::ExternalPid => write!(f, "ExtPid"),
      boxed::BoxTypeTag::ExternalPort => write!(f, "ExtPort"),
      boxed::BoxTypeTag::ExternalRef => write!(f, "ExtRef"),

      _ => panic!("Unexpected header tag {}", h_tag)
    }
  }


  /// Given `p`, a pointer to tuple header word, format tuple contents.
  unsafe fn format_tuple(p: *const Word, f: &mut fmt::Formatter) -> fmt::Result {
    let tptr = boxed::Tuple::from_pointer(p);

    write!(f, "{{")?;

    let arity = tptr.arity();
    for i in 0..arity {
      write!(f, "{}", tptr.get_element_base0(i))?;
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
      write!(f, "{}", raw_cons.hd())?;
      let tl = raw_cons.tl();
      if tl.is_nil() {
        // Proper list ends here, do not show the tail
        break;
      } else if tl.is_cons() {
        // List continues, print a comma and follow the tail
        write!(f, ", ")?;
        raw_cons = tl.get_cons_ptr();
      } else {
        // Improper list, show tail
        write!(f, "| {}", tl)?;
        break
      }
    }
    write!(f, "]")
  }


  pub unsafe fn format_cons_ascii(self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\"")?;

    let mut raw_cons = self.get_cons_ptr();
    loop {
      write!(f, "{}", raw_cons.hd().get_small_unsigned() as u8 as char)?;
      let tl = raw_cons.tl();
      if tl.is_nil() {
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
  // Atoms ==============================
  //

  pub fn atom_index(self) -> Word {
    debug_assert!(self.is_atom());
    return self.get_term_val_without_tag();
  }
}


// Printing low_level Terms as "{}"
impl fmt::Display for LTerm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.get_term_tag() {
      TermTag::Boxed => unsafe {
        let p = self.box_ptr();
        if LTerm::is_cp() {
          write!(f, "CP({:p})", self.cp_get_ptr())
        } else {
          LTerm::format_header(*p, p, f)
        }
      },

      TermTag::Cons => unsafe {
        if self.cons_is_ascii_string() {
          self.format_cons_ascii(f)
        } else {
          self.format_cons(f)
        }
      },
      TermTag::Small => write!(f, "{}", self.get_small_signed()),
      TermTag::Special => self.format_special(f),
      TermTag::LocalPid => {
        write!(f, "LocalPid({})", self.get_term_val_without_tag())
      },
      TermTag::LocalPort => {
        write!(f, "LocalPort({})", self.get_term_val_without_tag())
      },
      TermTag::Atom => {
        match atom::to_str(*self) {
          Ok(s) => write!(f, "'{}'", s),
          Err(_e) => write!(f, "Atom?"),
        }
      },
      TermTag::Header => {
        write!(f, "Header(")?;
        LTerm::format_header(*self, ptr::null(), f)?;
        write!(f, ")")
      },

      _ => panic!("Primary tag {} not recognized", self.get_term_tag())
    }
  }
} // trait Display


//
// Testing section
//

//#[cfg(test)]
//mod tests {
//  use std::ptr;
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
  err: Error
) -> Hopefully<*const T> {
  if !t.is_boxed() { return Err(Error::TermIsNotABoxed) }
  let cptr = t.get_box_ptr() as *const T;
  if unsafe { cptr.header.t != box_type } {
    return Err(err)
  }
  Ok(cptr)
}


pub unsafe fn helper_get_mut_from_boxed_term<T>(
  t: LTerm,
  box_type: BoxTypeTag,
  err: Error
) -> Hopefully<*mut T> {
  debug_assert!(t.is_boxed());
  let cptr = t.get_box_ptr() as *mut T;
  unsafe { debug_assert_eq!(cptr.header.t, box_type) }
  Ok(cptr)
}
