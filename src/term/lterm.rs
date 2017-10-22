//!
//! Low level term library
//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance
//!
use term::immediate;
use term::immediate::{IMM2_SPECIAL_NIL_RAW, IMM2_SPECIAL_NONVALUE_RAW};
use term::primary;
use term::raw::{ConsPtr, ConsPtrMut, TuplePtr, TuplePtrMut};
use emulator::atom;

use defs;
use defs::{Word, SWord, MIN_NEG_SMALL, MAX_POS_SMALL, MAX_UNSIGNED_SMALL};
//type Word = defs::Word;

use std::cmp::Ordering;
use std::fmt;
use std::ptr;


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

  #[inline]
  pub fn is_list(&self) -> bool {
    self.is_cons() || self.is_nil()
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
  pub unsafe fn cons_get_ptr(&self) -> ConsPtr {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_CONS);
    let boxp = primary::pointer(v);
    ConsPtr::from_pointer(boxp)
  }


  /// Get a proxy object for looking and modifying cons contents.
  pub unsafe fn cons_get_ptr_mut(&self) -> ConsPtrMut {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_CONS);
    let boxp = primary::pointer_mut(v);
    ConsPtrMut::from_pointer(boxp)
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
    assert!(self.is_box());
    primary::pointer(self.value)
  }


  #[inline]
  pub fn box_ptr_mut(&self) -> *mut Word {
    assert!(self.is_box());
    primary::pointer_mut(self.value)
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
    immediate::get_imm1_value_s(self.value)
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
  pub unsafe fn raw_tuple(&self) -> TuplePtr {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_HEADER);
    assert_eq!(primary::header::get_tag(v),
               primary::header::TAG_HEADER_TUPLE);
    let boxp = primary::pointer(v);
    TuplePtr::from_pointer(boxp)
  }


  /// Get a proxy object for looking and modifying cons contents.
  pub unsafe fn raw_tuple_mut(&self) -> TuplePtrMut {
    let v = self.value;
    assert_eq!(primary::get_tag(v), primary::TAG_HEADER);
    assert_eq!(primary::header::get_tag(v),
               primary::header::TAG_HEADER_TUPLE);
    let boxp = primary::pointer_mut(v);
    TuplePtrMut::from_pointer(boxp)
  }


  //
  // Code Pointer manipulation.
  // CP is tagged as Boxed + Top bit set.
  //

  #[inline]
  pub fn make_cp(p: *const Word) -> LTerm {
    let tagged_p = (p as Word) | defs::TAG_CP;
    LTerm::make_box(tagged_p as *const Word)
  }


  #[inline]
  pub fn is_cp(&self) -> bool {
    self.is_box() && (self.value & defs::TAG_CP == defs::TAG_CP)
  }


  #[inline]
  pub fn cp_get_ptr(&self) -> *const Word {
    assert!(self.is_box(), "CP value must be boxed (have {})", self);
    assert_eq!(self.value & defs::TAG_CP, defs::TAG_CP,
            "CP value must have its top bit set (have 0x{:x})", self.value);
    let untagged_p = self.value & !(defs::TAG_CP | primary::PRIM_MASK);
    untagged_p as *const Word
  }

  //
  // Formatting helpers
  //
  fn format_immed(&self, v: Word, f: &mut fmt::Formatter) -> fmt::Result {
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
      primary::header::TAG_HEADER_BIGNEG => write!(f, "BigNeg"),
      primary::header::TAG_HEADER_BIGPOS => write!(f, "BigPos"),
      primary::header::TAG_HEADER_REF => write!(f, "Ref"),
      primary::header::TAG_HEADER_FUN => write!(f, "Fun"),
      primary::header::TAG_HEADER_FLOAT => write!(f, "Float"),
      primary::header::TAG_HEADER_EXPORT => write!(f, "Export"),
      primary::header::TAG_HEADER_REFCBIN => write!(f, "RefcBin"),
      primary::header::TAG_HEADER_HEAPBIN => write!(f, "HeapBin"),
      primary::header::TAG_HEADER_SUBBIN => write!(f, "SubBin"),
      primary::header::TAG_HEADER_HEAPOBJ => write!(f, "HeapObj"),
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
    let tptr = TuplePtr::from_pointer(p);

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
          write!(f, "Box(")?;
          LTerm::format_header(*p, p, f)?;
          write!(f, ")")
        }
      },

      primary::TAG_CONS => unsafe {
        let raw_cons = self.cons_get_ptr();
        write!(f, "[{} | {}]", raw_cons.hd(), raw_cons.tl())
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
  use super::*;
  use std::ptr;

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
    let s2_out = s2.small_get_s();
    assert_eq!(1, s2_out, "Expect 1, have 0x{:x}", s2_out);

    let s1 = LTerm::make_small_s(-1);
    let s1_out = s1.small_get_s();
    assert_eq!(-1, s1_out, "Expect -1, have 0x{:x}", s1_out);
  }


  #[test]
  fn test_small_signed_limits() {
    let s2 = LTerm::make_small_s(MAX_POS_SMALL);
    assert_eq!(MAX_POS_SMALL, s2.small_get_s());

    let s3 = LTerm::make_small_s(MIN_NEG_SMALL);
    assert_eq!(MIN_NEG_SMALL, s3.small_get_s());
  }


  #[test]
  fn test_cp() {
    let s1 = LTerm::make_cp(ptr::null());
    assert_eq!(s1.cp_get_ptr(), ptr::null());
  }
}
