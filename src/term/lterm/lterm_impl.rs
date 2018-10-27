//! Low level term library
//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance
//!
//! Do not import this file directly, use `use term::lterm::*;` instead.

use emulator::atom;
use emulator::heap::{IHeap};
use rt_defs::*;
use super::super::lterm::*;
use term::immediate;
use term::boxed::{BoxHeader, BoxTypeTag};
use term::boxed;
use term::primary;
use term::raw::*;

use std::cmp::Ordering;
use std::fmt;
use std::ptr;
use fail::Hopefully;


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
    LTerm::make_from_tag_and_value(TAG_ATOM, id)
  }


  #[inline]
  pub const fn make_cp<T>(p: *const T) -> LTerm {
    let tagged_p = (p as Word) | TAG_CP;
    LTerm::from_raw(tagged_p)
  }


  #[inline]
  pub const fn empty_tuple() -> LTerm {
    LTerm::make_from_tag_and_value(TAG_SPECIAL, VAL_SPECIAL_EMPTY_TUPLE)
  }


  #[inline]
  pub const fn empty_binary() -> LTerm {
    LTerm::make_from_tag_and_value(TAG_SPECIAL, VAL_SPECIAL_EMPTY_BINARY)
  }


  #[inline]
  pub const fn nil() -> LTerm {
    LTerm::make_from_tag_and_value(TAG_SPECIAL, VAL_SPECIAL_EMPTY_LIST)
  }


  #[inline]
  pub const fn make_from_tag_and_value(t: Word, v: Word) -> LTerm {
    LTerm::from_raw(v << TERM_TAG_BITS | t)
  }


  // TODO: Some safety checks maybe? But oh well
  #[inline]
  pub const fn make_boxed<T>(p: *const T) -> LTerm {
    LTerm { value: p as Word }
  }


  /// Create a NON_VALUE.
  #[inline]
  pub fn non_value() -> LTerm {
    LTerm { value: 0 }
  }


  /// Check whether a value is a NON_VALUE.
  #[inline]
  pub fn is_non_value(self) -> bool {
    self.p.is_null()
  }


  /// Check whether a value is NOT a NON_VALUE.
  #[inline]
  pub fn is_value(self) -> bool {
    ! self.p.is_null()
  }


  /// Get tag bits from the p field as integer.
  #[inline]
  pub fn get_term_tag(self) -> TermTag {
    self.raw() & TERM_TAG_MASK
  }


  /// Check whether tag bits of a value equal to TAG_BOXED=0
  #[inline]
  pub fn is_boxed(self) -> bool {
    self.get_term_tag() == TAG_BOXED
  }


  #[inline]
  pub fn get_box_ptr(self) -> *const BoxHeader {
    self.p as *const BoxHeader
  }


  #[inline]
  pub fn get_box_ptr_mut(self) -> *mut BoxHeader {
    self.p
  }


  #[inline]
  pub fn is_binary(self) -> bool {
    self.is_boxed() && self.p.t == BoxTypeTag::Binary
  }


  #[inline]
  pub fn is_immediate(self) -> bool {
    self.get_term_tag() != TAG_BOXED
  }


  /// Check whether the value is tagged as atom
  #[inline]
  pub fn is_atom(self) -> bool {
    self.get_term_tag() == TAG_ATOM
  }


  /// Return true if a value's tag is regx, regy or regfp
  #[inline]
  pub fn is_internal_immediate(self) -> bool {
    self.get_term_tag() >= TAG_REGX
  }


  /// For non-pointer Term types get the encoded integer without tag bits
  pub fn get_p_val_without_tag(self) -> Word {
    debug_assert!(self.get_term_tag() != TAG_BOXED);
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
    LTerm::make_from_tag_and_value(TAG_LOCAL_PID, pindex)
  }


  pub fn make_remote_pid(hp: &mut IHeap,
                         node: LTerm,
                         pindex: Word) -> Hopefully<LTerm> {
    let rpid_ptr = boxed::RemotePid::create_into(hp, node, pindex)?;
    Ok(LTerm::make_boxed(rpid_ptr))
  }


  pub fn get_special_tag(self) -> SpecialTag {
    // cut away term tag bits and extract special tag
    ((self.value >> TERM_TAG_BITS) & TERM_SPECIAL_TAG_MASK) as SpecialTag
  }


  pub fn make_special(special_t: Word, val: Word) -> LTerm {
    let special_v = val << TAG_SPECIAL_BITS | VAL_SPECIAL_REGX;
    LTerm::make_from_tag_and_value(TAG_SPECIAL, v)
  }


  pub fn make_xreg(n: Word) -> LTerm {
    LTerm::make_special(VAL_SPECIAL_REGX, n)
  }


  pub fn make_yreg(n: Word) -> LTerm {
    LTerm::make_special(VAL_SPECIAL_REGY, n)
  }

  pub fn make_fpreg(n: Word) -> LTerm {
    LTerm::make_special(VAL_SPECIAL_REGFP, n)
  }


  //
  // Tuples
  //

//  pub fn header_get_arity(self) -> Word {
//    assert!(self.is_boxed());
//    primary::header::get_arity(self.value)
//  }


//  pub fn header_get_type(self) -> Word {
//    assert!(self.is_boxed());
//    primary::header::get_tag(self.value)
//  }


  //
  // Formatting helpers
  //

  fn format_immed(self, v: Word, f: &mut fmt::Formatter) -> fmt::Result {
    match immediate::get_imm1_tag(v) {
      immediate::TAG_IMM1_SMALL => write!(f, "{}", self.small_get_s()),

      immediate::TAG_IMM1_PID =>
        write!(f, "Pid({})", immediate::get_imm1_value(v)),

      immediate::TAG_IMM1_PORT =>
        write!(f, "Port({})", immediate::get_imm1_value(v)),

      immediate::TAG_IMM1_IMM2 =>

        match immediate::get_imm2_tag(v) {
          immediate::TAG_IMM2_CATCH =>
            write!(f, "Catch({})", immediate::get_imm2_value(v)),

          immediate::TAG_IMM2_SPECIAL => {
            if self.is_nil() {
              write!(f, "[]")
            } else if self.is_non_value() {
              write!(f, "NON_VALUE")
            } else if self.is_empty_binary() {
              write!(f, "<<>>")
            } else if self.is_empty_tuple() {
              write!(f, "{{}}")
            } else {
              write!(f, "Special(0x{:x})", immediate::get_imm2_value(v))
            }
          },

          immediate::TAG_IMM2_ATOM => {
            match atom::to_str(self) {
              Ok(s) => write!(f, "'{}'", s),
              Err(_e) => write!(f, "Atom?"),
            }
          },

          immediate::TAG_IMM2_IMM3 => {
            let v3 = immediate::get_imm3_value(v);

            match immediate::get_imm3_tag(v) {
              immediate::TAG_IMM3_XREG => write!(f, "X({})", v3),
              immediate::TAG_IMM3_YREG => write!(f, "Y({})", v3),
              immediate::TAG_IMM3_FPREG => write!(f, "FP({})", v3),
              immediate::TAG_IMM3_OPCODE => write!(f, "Opcode({})", v3),
              _ => panic!("Immediate3 tag must be in range 0..3")
            }
          }
          _ => panic!("Immediate2 tag must be in range 0..3")
        },
      _ => panic!("Immediate1 tag must be in range 0..3")
    }
  }


  /// Attempt to display contents of a tagged header word and the words which
  /// follow it. Arg `p` if not null is used to fetch the following memory words
  /// and display more detail.
  fn format_header(value_at_ptr: Word,
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

    let mut raw_cons = self.cons_get_ptr();
    loop {
      write!(f, "{}", raw_cons.hd())?;
      let tl = raw_cons.tl();
      if tl.is_nil() {
        // Proper list ends here, do not show the tail
        break;
      } else if tl.is_cons() {
        // List continues, print a comma and follow the tail
        write!(f, ", ")?;
        raw_cons = tl.cons_get_ptr();
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

    let mut raw_cons = self.cons_get_ptr();
    loop {
      write!(f, "{}", raw_cons.hd().small_get_u() as u8 as char)?;
      let tl = raw_cons.tl();
      if tl.is_nil() {
        // Proper list ends here, do not show the tail
        break;
      } else if tl.is_cons() {
        // List continues, follow the tail
        raw_cons = tl.cons_get_ptr();
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
    return self.get_p_val_without_tag();
  }
}


// Printing low_level Terms as "{}"
impl fmt::Display for LTerm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let v = self.value;

    match primary::get_tag(v) {
      primary::TAG_BOX => unsafe {
        let p = self.box_ptr();
        if self.is_cp() {
          write!(f, "CP({:p})", self.cp_get_ptr())
        } else {
          //write!(f, "Box(")?;
          LTerm::format_header(*p, p, f)
          //write!(f, ")")
        }
      },

      primary::TAG_CONS => unsafe {
        if self.cons_is_ascii_string() {
          self.format_cons_ascii(f)
        } else {
          self.format_cons(f)
        }
      },

      primary::TAG_IMMED => self.format_immed(v, f),

      primary::TAG_HEADER => {
        write!(f, "Header(")?;
        LTerm::format_header(v, ptr::null(), f)?;
        write!(f, ")")
      },

      _ => panic!("Primary tag must be in range 0..3")
    }
  }
} // trait Display


//
// Testing section
//

#[cfg(test)]
mod tests {
  use std::ptr;
  use std::mem;

  use rt_defs::{MIN_NEG_SMALL, MAX_POS_SMALL, MAX_UNSIGNED_SMALL, WORD_BYTES};
  use super::*;
  use term::lterm::aspect_smallint::*;

  #[test]
  fn test_nil_is_not_atom() {
    // Some obscure bit mishandling made nil be recognized as atom
    let n = LTerm::nil();
    assert!(!n.is_atom(), "must not be an atom {} 0x{:x} imm2_pfx 0x{:x}, imm2atompfx 0x{:x}",
            n, n.raw(), immediate::get_imm2_prefix(n.raw()),
            immediate::IMM2_ATOM_PREFIX);
  }

  #[test]
  fn test_term_size() {
    assert_eq!(mem::size_of::<LTerm>(), WORD_BYTES);
  }

  #[test]
  fn test_small_unsigned() {
    let s1 = make_small_u(1);
    assert_eq!(1, s1.small_get_u());

    let s2 = make_small_u(MAX_UNSIGNED_SMALL);
    assert_eq!(MAX_UNSIGNED_SMALL, s2.small_get_u());
  }


  #[test]
  fn test_small_signed_1() {
    let s2 = make_small_s(1);
    let s2_out = s2.small_get_s();
    assert_eq!(1, s2_out, "Expect 1, have 0x{:x}", s2_out);

    let s1 = make_small_s(-1);
    let s1_out = s1.small_get_s();
    assert_eq!(-1, s1_out, "Expect -1, have 0x{:x}", s1_out);
  }


  #[test]
  fn test_small_signed_limits() {
    let s2 = make_small_s(MAX_POS_SMALL);
    assert_eq!(MAX_POS_SMALL, s2.small_get_s());

    let s3 = make_small_s(MIN_NEG_SMALL);
    assert_eq!(MIN_NEG_SMALL, s3.small_get_s());
  }


  #[test]
  fn test_cp() {
    let s1 = make_cp(ptr::null());
    assert_eq!(s1.cp_get_ptr(), ptr::null());
  }
}
