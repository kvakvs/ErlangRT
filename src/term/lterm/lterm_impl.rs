//!
//! Low level term library
//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance
//!
use term::immediate;
use term::primary;
use emulator::atom;
use rt_defs::{Word};
use super::super::lterm::*;
use term::raw::*;

use std::cmp::Ordering;
use std::fmt;
use std::ptr;


//fn module() -> &'static str { "lterm_impl: " }


/// A low-level term, packed conveniently in a Word, or containing a
/// pointer to heap.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct LTerm {
  pub value: Word
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
  /// Access the raw Word value of the low-level term.
  #[inline]
  pub fn raw(&self) -> Word { self.value }


  /// Create a NON_VALUE.
  #[inline]
  pub fn non_value() -> LTerm {
    LTerm { value: immediate::IMM2_SPECIAL_NONVALUE_RAW }
  }


  /// Check whether a value is a NON_VALUE.
  #[inline]
  pub fn is_non_value(&self) -> bool {
    self.value == immediate::IMM2_SPECIAL_NONVALUE_RAW
  }


  /// Check whether a value is NOT a NON_VALUE.
  #[inline]
  pub fn is_value(&self) -> bool {
    ! self.is_non_value()
  }


  /// Get primary tag bits from a raw term
  #[inline]
  pub fn primary_tag(&self) -> Word {
    primary::get_tag(self.value)
  }

  /// Check whether a value has immediate1 bits as prefix.
  #[inline]
  pub fn is_immediate1(&self) -> bool {
    immediate::is_immediate1(self.value)
  }

  /// Check whether a value has immediate2 bits as prefix.
  #[inline]
  pub fn is_immediate2(&self) -> bool {
    immediate::is_immediate2(self.value)
  }

  /// Check whether a value has immediate3 bits as prefix.
  #[inline]
  pub fn is_immediate3(&self) -> bool {
    immediate::is_immediate3(self.value)
  }

  /// Check whether primary tag of a value is `TAG_HEADER`.
  #[inline]
  pub fn is_header(&self) -> bool {
    self.primary_tag() == primary::TAG_HEADER
  }

  /// Retrieve the raw value of a `LTerm`.
  #[inline]
  pub fn get_raw(&self) -> Word { self.value }

  //
  // Construction
  //

  /// Any raw word becomes a term, possibly invalid
  #[inline]
  pub fn from_raw(w: Word) -> LTerm {
    LTerm { value: w }
  }


  /// From internal process index create a pid. To create a process use vm::create_process
  #[inline]
  pub fn make_pid(pindex: Word) -> LTerm {
    LTerm { value: immediate::make_pid_raw(pindex) }
  }

  #[inline]
  pub fn make_xreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_xreg_raw(n) }
  }

  #[inline]
  pub fn make_yreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_yreg_raw(n) }
  }

  #[inline]
  pub fn make_fpreg(n: Word) -> LTerm {
    LTerm { value: immediate::make_fpreg_raw(n) }
  }

//  #[inline]
//  pub fn make_label(n: Word) -> LTerm {
//    LTerm { value: immediate::make_label_raw(n) }
//  }


  //
  // Tuples
  //

  pub fn header_get_arity(&self) -> Word {
    assert!(self.is_header());
    primary::header::get_arity(self.value)
  }


  pub fn header_get_type(&self) -> Word {
    assert!(self.is_header());
    primary::header::get_tag(self.value)
  }


  //
  // Formatting helpers
  //

  fn format_immed(&self, v: Word, f: &mut fmt::Formatter) -> fmt::Result {
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
            match atom::to_str(*self) {
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
  fn format_header(v: Word, p: *const Word,
                   f: &mut fmt::Formatter) -> fmt::Result {
    let arity = primary::header::get_arity(v);
    let h_tag = primary::header::get_tag(v);

    match h_tag {
      primary::header::TAG_HEADER_TUPLE =>
        if p.is_null() {
          write!(f, "Tuple[{}]", arity)
        } else {
          unsafe { LTerm::format_tuple(p, f) }
        },
//      primary::header::TAG_HEADER_BIGNEG => write!(f, "BigNeg"),
//      primary::header::TAG_HEADER_BIGPOS => write!(f, "BigPos"),
      primary::header::TAG_HEADER_REF => write!(f, "Ref"),
      primary::header::TAG_HEADER_FUN => write!(f, "Fun"),
      primary::header::TAG_HEADER_FLOAT => write!(f, "Float"),
//      primary::header::TAG_HEADER_EXPORT => write!(f, "Export"),
//      primary::header::TAG_HEADER_REFCBIN => write!(f, "RefcBin"),
//      primary::header::TAG_HEADER_HEAPBIN => write!(f, "HeapBin"),
//      primary::header::TAG_HEADER_SUBBIN => write!(f, "SubBin"),
      primary::header::TAG_HEADER_HEAPOBJ => unsafe {
        let ho_ptr = p.offset(1);
        let hoclass = *(ho_ptr) as *const HeapObjClass;
        // Starting word of the heap object (the primary header word) becomes
        // `this` pointer for the heapobject, hence passing `p`.
        let s = ((*hoclass).fmt_str)(p) ;
        write!(f, "{}", s)
      },
      primary::header::TAG_HEADER_EXTPID => write!(f, "ExtPid"),
      primary::header::TAG_HEADER_EXTPORT => write!(f, "ExtPort"),
      primary::header::TAG_HEADER_EXTREF => write!(f, "ExtRef"),

      _ => panic!("Unexpected header tag {} value {}",
                  h_tag, primary::get_value(v))
    }
  }


  /// Given `p`, a pointer to tuple header word, format tuple contents.
  unsafe fn format_tuple(p: *const Word,
                  f: &mut fmt::Formatter) -> fmt::Result {
    let tptr = rtuple::Ptr::from_pointer(p);

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


  pub unsafe fn format_cons(&self, f: &mut fmt::Formatter) -> fmt::Result {
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


  pub unsafe fn format_cons_ascii(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
  #[inline]
  pub fn is_same(a: LTerm, b: LTerm) -> bool {
    a.raw() == b.raw()
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
  use term::lterm::aspect_atom::*;

  #[test]
  fn test_nil_is_not_atom() {
    // Some obscure bit mishandling made nil be recognized as atom
    let n = nil();
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
