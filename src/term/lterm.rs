//!
//! Low level term library
//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance
//!
use term::immediate;
use term::immediate::{IMM2_SPECIAL_NIL_RAW, IMM2_SPECIAL_NONVALUE_RAW};
use term::primary;
use term::raw::{RawCons, RawConsMut, RawTuple, RawTupleMut};
use emulator::atom;

use defs;
use defs::{Word, SWord, MIN_NEG_SMALL, MAX_POS_SMALL, MAX_UNSIGNED_SMALL};
//type Word = defs::Word;

use std::cmp::Ordering;
use std::fmt;


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

  /// Create a NIL value.
  #[inline]
  pub fn nil() -> LTerm { LTerm { value: IMM2_SPECIAL_NIL_RAW } }

  /// Check whether a value is a NIL \[ \]
  #[inline]
  pub fn is_nil(&self) -> bool {
    self.value == IMM2_SPECIAL_NIL_RAW
  }

  /// Create a NON_VALUE.
  #[inline]
  pub fn non_value() -> LTerm {
    LTerm { value: IMM2_SPECIAL_NONVALUE_RAW }
  }

  /// Check whether a value is a NON_VALUE.
  #[inline]
  pub fn is_non_value(&self) -> bool {
    self.value == IMM2_SPECIAL_NONVALUE_RAW
  }

  /// Check whether a value is NOT a NON_VALUE.
  #[inline]
  pub fn is_value(&self) -> bool {
    ! self.is_non_value()
  }

  /// Check whether a value is a local pid.
  #[inline]
  pub fn is_local_pid(&self) -> bool {
    immediate::is_pid_raw(self.value)
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

  /// Check whether primary tag of a value is `TAG_BOX`.
  #[inline]
  pub fn is_box(&self) -> bool {
    self.primary_tag() == primary::TAG_BOX
  }

  /// Check whether primary tag of a value is `TAG_CONS`.
  #[inline]
  pub fn is_cons(&self) -> bool {
    self.primary_tag() == primary::TAG_CONS
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


  #[inline]
  pub fn make_cp(p: *const Word) -> LTerm {
    LTerm { value: (p as Word) | defs::TAG_CP }
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

  /// From a pointer to heap create a generic box
  #[inline]
  pub fn make_box(ptr: *const Word) -> LTerm {
    LTerm { value: primary::make_box_raw(ptr) }
  }

  //
  // Cons, lists, list cells, heads, tails
  //

  /// From a pointer to heap create a cons box
  #[inline]
  pub fn make_cons(ptr: *const Word) -> LTerm {
    LTerm { value: primary::make_cons_raw(ptr) }
  }


  /// Get a proxy object for read-only accesing the cons contents.
  pub unsafe fn raw_cons(&self) -> RawCons {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_CONS);
    let boxp = primary::pointer(v);
    RawCons::from_pointer(boxp)
  }


  /// Get a proxy object for looking and modifying cons contents.
  pub unsafe fn raw_cons_mut(&self) -> RawConsMut {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_CONS);
    let boxp = primary::pointer_mut(v);
    RawConsMut::from_pointer(boxp)
  }

  //
  // Atom services - creation, checking
  //

  /// From atom index create an atom. To create from string use vm::new_atom
  #[inline]
  pub fn make_atom(index: Word) -> LTerm {
    LTerm { value: immediate::make_atom_raw(index) }
  }


  /// Check whether a value is a runtime atom.
  #[inline]
  pub fn is_atom(&self) -> bool {
    immediate::is_atom_raw(self.value)
  }


  /// For an atom value, get index.
  pub fn atom_index(&self) -> Word {
    assert!(self.is_atom());
    immediate::get_imm2_value(self.value)
  }

  //
  // Box services - boxing, unboxing, checking
  //

  #[inline]
  pub fn box_ptr(&self) -> *const Word {
    primary::pointer(self.value)
  }

  //
  // Small integer handling
  //

  /// Check whether a value is a small integer.
  #[inline]
  pub fn is_small(&self) -> bool {
    immediate::is_small_raw(self.value)
  }


  #[inline]
  pub fn make_small_u(n: Word) -> LTerm {
    assert!(n <= MAX_UNSIGNED_SMALL,
            "make_small_u n=0x{:x} <= limit=0x{:x}", n, MAX_UNSIGNED_SMALL);
    LTerm { value: immediate::make_small_raw(n as SWord) }
  }


  #[inline]
  pub fn make_small_s(n: SWord) -> LTerm {
    // TODO: Do the proper min neg small
    assert!(n >= MIN_NEG_SMALL,
            "make_small_s: n=0x{:x} must be >= MIN_NEG_SMALL 0x{:x}",
            n, MIN_NEG_SMALL);
    assert!(n <= MAX_POS_SMALL,
            "make_small_s: n=0x{:x} must be <= MAX_POS_SMALL 0x{:x}",
            n, MAX_POS_SMALL);
    //let un = defs::unsafe_sword_to_word(n);
    LTerm { value: immediate::make_small_raw(n) }
  }


  #[inline]
  pub fn small_get_s(&self) -> SWord {
    let n = immediate::get_imm1_value(self.value);
    //return defs::unsafe_word_to_sword(n);
    n as SWord
  }


  #[inline]
  pub fn small_get_u(&self) -> Word {
    immediate::get_imm1_value(self.value)
  }

  //
  // Headers, tuples etc, boxed stuff on heap and special stuff in code
  //

  #[inline]
  pub fn make_tuple_header(arity: Word) -> LTerm {
    LTerm { value: primary::header::make_tuple_header_raw(arity) }
  }


  pub fn header_arity(&self) -> Word {
    assert!(self.is_header());
    primary::get_value(self.value)
  }


  /// Get a proxy object for read-only accesing the cons contents.
  pub unsafe fn raw_tuple(&self) -> RawTuple {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_HEADER);
    assert_eq!(primary::header::get_tag(v),
               primary::header::TAG_HEADER_TUPLE);
    let boxp = primary::pointer(v);
    RawTuple::from_pointer(boxp)
  }


  /// Get a proxy object for looking and modifying cons contents.
  pub unsafe fn raw_tuple_mut(&self) -> RawTupleMut {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_HEADER);
    assert_eq!(primary::header::get_tag(v),
               primary::header::TAG_HEADER_TUPLE);
    let boxp = primary::pointer_mut(v);
    RawTupleMut::from_pointer(boxp)
  }

}


// Printing low_level Terms as "{}"
impl fmt::Display for LTerm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let v = self.value;

    match primary::get_tag(v) {
      primary::TAG_BOX => write!(f, "Box({:?})", self.box_ptr()),

      primary::TAG_CONS => unsafe {
        let raw_cons = self.raw_cons();
        write!(f, "[{} | {}]", raw_cons.hd(), raw_cons.tl())
      },

      primary::TAG_IMMED =>
        match immediate::get_imm1_tag(v) {
          immediate::TAG_IMM1_SMALL =>
            write!(f, "{}", self.small_get_s()),

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
                  } else {
                    write!(f, "Special(0x{:x})", immediate::get_imm2_value(v))
                  }
                },

              immediate::TAG_IMM2_ATOM =>
                write!(f, "'{}'", atom::to_str(*self)),

              immediate::TAG_IMM2_IMM3 =>

                match immediate::get_imm3_tag(v) {
                  immediate::TAG_IMM3_XREG =>
                    write!(f, "X({})", immediate::get_imm3_value(v)),

                  immediate::TAG_IMM3_YREG =>
                    write!(f, "Y({})", immediate::get_imm3_value(v)),

                  immediate::TAG_IMM3_FPREG =>
                    write!(f, "FP({})", immediate::get_imm3_value(v)),

//                  immediate::Immediate3::Label =>
//                    write!(f, "Label(0x{:04x})", immediate::imm3_value(v)),

                  _ => panic!("Immediate3 tag must be in range 0..3")
                }
              _ => panic!("Immediate2 tag must be in range 0..3")
            },
          _ => panic!("Immediate1 tag must be in range 0..3")
        },
      primary::TAG_HEADER => {
//        let hptr = primary::pointer(v);
//        let hval = unsafe { *hptr };

        let arity = primary::header::get_arity(v);

        match primary::header::get_tag(v) {
          primary::header::TAG_HEADER_TUPLE => {
            write!(f, "Header: Tuple[{}]", arity)
//            let raw_tuple = RawTuple::from_pointer(hptr);
//            write!(f, "{{").unwrap();
//            let arity = unsafe { raw_tuple.arity() };
//            for i in 0..arity {
//              let item = unsafe { raw_tuple.get_element(i) };
//              write!(f, "{}", item).unwrap();
//              if i + 1 < arity {
//                write!(f, ", ").unwrap();
//              }
//            }
//            write!(f, "}}")
          },
          primary::header::TAG_HEADER_BIGNEG => write!(f, "Header: BigNeg"),
          primary::header::TAG_HEADER_BIGPOS => write!(f, "Header: BigPos"),
          primary::header::TAG_HEADER_REF => write!(f, "Header: Ref"),
          primary::header::TAG_HEADER_FUN => write!(f, "Header: Fun"),
          primary::header::TAG_HEADER_FLOAT => write!(f, "Header: Float"),
          primary::header::TAG_HEADER_EXPORT => write!(f, "Header: Export"),
          primary::header::TAG_HEADER_REFCBIN => write!(f, "Header: RefcBin"),
          primary::header::TAG_HEADER_HEAPBIN => write!(f, "Header: HeapBin"),
          primary::header::TAG_HEADER_SUBBIN => write!(f, "Header: SubBin"),
          primary::header::TAG_HEADER_EXTPID => write!(f, "Header: ExtPid"),
          primary::header::TAG_HEADER_EXTPORT => write!(f, "Header: ExtPort"),
          primary::header::TAG_HEADER_EXTREF => write!(f, "Header: ExtRef"),

          _ => panic!("Unexpected header tag value {}",
                      primary::get_value(v))
        }
      },
      _ => panic!("Primary tag must be in range 0..3")
    }
  }
}


//
// Testing section
//

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_small_unsigned() {
    let s1 = LTerm::make_small_u(1);
    assert_eq!(1, s1.small_get_u());

    let s2 = LTerm::make_small_u(MAX_UNSIGNED_SMALL);
    assert_eq!(MAX_UNSIGNED_SMALL, s2.small_get_u());
  }

  #[test]
  fn test_small_signed_1() {
    let s2 = LTerm::make_small_s(1);
    assert_eq!(1, s2.small_get_s());

    let s1 = LTerm::make_small_s(-1);
    assert_eq!(-1, s1.small_get_s());
  }

  #[test]
  fn test_small_signed_limits() {
    let s2 = LTerm::make_small_s(MAX_POS_SMALL);
    assert_eq!(MAX_POS_SMALL, s2.small_get_s());

    let s3 = LTerm::make_small_s(MIN_NEG_SMALL);
    assert_eq!(MIN_NEG_SMALL, s3.small_get_s());
  }
}
