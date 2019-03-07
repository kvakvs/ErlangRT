//! Low level term library
//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance
//!
//! Do not import this file directly, use `use term::lterm::*;` instead.

use crate::{
  defs::*,
  emulator::{gen_atoms, heap::Heap},
  fail::{RtErr, RtResult},
  term::boxed::{self, BoxHeader, BoxType},
};
use core::{cmp::Ordering, isize};
use std::fmt;
use crate::term::boxed::box_header;

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
pub struct TermTag(usize);

impl TermTag {
  #[inline]
  pub const fn get(self) -> usize {
    self.0
  }
}

pub const TERMTAG_BOXED: TermTag = TermTag(0);
pub const TERMTAG_HEADER: TermTag = TermTag(1);
pub const TERMTAG_CONS: TermTag = TermTag(2);
// From here and below, values are immediate (fit into a single word)
pub const TERMTAG_SMALL: TermTag = TermTag(3);
pub const TERMTAG_ATOM: TermTag = TermTag(4);
pub const TERMTAG_LOCALPID: TermTag = TermTag(5);
pub const TERMTAG_LOCALPORT: TermTag = TermTag(6);
pub const TERMTAG_SPECIAL: TermTag = TermTag(7);

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
/// Contains index in the catch table of the current module
pub const SPECIALTAG_CATCH: SpecialTag = SpecialTag(4);
// decorates opcodes for easier code walking
pub const SPECIALTAG_OPCODE: SpecialTag = SpecialTag(5);

pub struct SpecialConst(pub Word);

pub const SPECIALCONST_EMPTYTUPLE: SpecialConst = SpecialConst(0);
pub const SPECIALCONST_EMPTYLIST: SpecialConst = SpecialConst(1);
pub const SPECIALCONST_EMPTYBINARY: SpecialConst = SpecialConst(2);

/// A low-level term is either a pointer to memory term or an Immediate with
/// leading bits defining its type (see TAG_* consts below).
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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
  pub const fn make_atom(id: Word) -> Self {
    Self::make_from_tag_and_value(TERMTAG_ATOM, id)
  }

  #[inline]
  pub const fn empty_tuple() -> Self {
    Self::make_special(SPECIALTAG_CONST, SPECIALCONST_EMPTYTUPLE.0)
  }

  #[inline]
  pub const fn nil() -> Self {
    Self::make_special(SPECIALTAG_CONST, SPECIALCONST_EMPTYLIST.0)
  }

  pub const fn make_from_tag_and_value(t: TermTag, v: Word) -> Self {
    Self::from_raw(v << TERM_TAG_BITS | t.0)
  }

  pub const fn make_from_tag_and_signed_value(t: TermTag, v: SWord) -> Self {
    Self::from_raw((v << TERM_TAG_BITS | (t.0 as SWord)) as Word)
  }

  /// Create a NON_VALUE.
  pub const fn non_value() -> Self {
    Self { value: 0 }
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

  // === === BOXED === === ===
  //

  // TODO: Some safety checks maybe? But oh well
  #[inline]
  pub fn make_boxed<T>(p: *const T) -> Self {
    Self { value: p as Word }
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
      return Err(RtErr::TermIsNotABoxed);
    }
    Ok(self.value as *const T)
  }

  pub fn get_box_ptr_safe_mut<T>(self) -> RtResult<*mut T> {
    if !self.is_boxed() {
      return Err(RtErr::TermIsNotABoxed);
    }
    Ok(self.value as *mut T)
  }

  /// Checks boxed tag to be equal to t, also returns false if not a boxed.
  #[inline]
  pub fn is_boxed_of_type(self, t: BoxType) -> bool {
    self.is_boxed_of_(|boxtag| boxtag == t)
  }

  /// Extracts boxed tag and runs an inline predicate on its boxed tag, allows
  /// for checking multiple boxed tag values. Returns false if not a boxed.
  #[inline]
  fn is_boxed_of_<F>(self, pred: F) -> bool
  where
    F: Fn(BoxType) -> bool,
  {
    if !self.is_boxed() {
      return false;
    }
    let box_ptr = self.get_box_ptr::<BoxHeader>();
    let trait_ptr = unsafe { (*box_ptr).get_trait_ptr() };
    let tag = unsafe { (*trait_ptr).get_type() };
    pred(tag)
  }

  // === === Binary === ===
  //

  #[inline]
  pub const fn empty_binary() -> Self {
    Self::make_special(SPECIALTAG_CONST, SPECIALCONST_EMPTYBINARY.0)
  }

  #[inline]
  pub fn is_binary(self) -> bool {
    self == Self::empty_binary() || self.is_boxed_of_type(boxed::BOXTYPETAG_BINARY)
  }

  pub unsafe fn binary_byte_size(self) -> ByteSize {
    let binp = boxed::Binary::get_trait_from_term(self);
    (*binp).get_byte_size()
  }

  //

  #[inline]
  pub fn is_immediate(self) -> bool {
    self.get_term_tag() != TERMTAG_BOXED
  }

  /// Check whether the value is tagged as atom
  #[inline]
  pub fn is_atom(self) -> bool {
    self.get_term_tag() == TERMTAG_ATOM
  }

  #[inline]
  pub fn is_pid(self) -> bool {
    self.is_local_pid() || self.is_external_pid()
  }

  #[inline]
  pub fn is_local_pid(self) -> bool {
    self.get_term_tag() == TERMTAG_LOCALPID
  }

  /// Check whether a lterm is boxed and then whether it points to a word of
  /// memory tagged as external pid
  #[inline]
  pub fn is_external_pid(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_EXTERNALPID)
  }

  /// Return true if a value's tag will fit into a single word
  #[inline]
  pub fn is_internal_immediate(self) -> bool {
    self.get_term_tag() == TERMTAG_SPECIAL
  }

  /// For non-pointer Term types get the encoded integer without tag bits
  #[inline]
  pub fn get_term_val_without_tag(self) -> Word {
    debug_assert!(
      self.get_term_tag() != TERMTAG_BOXED && self.get_term_tag() != TERMTAG_CONS
    );
    self.value >> TERM_TAG_BITS
  }

  // === === CONSTRUCTION === === ===
  //

  /// Any raw word becomes a term, possibly invalid
  pub const fn from_raw(w: Word) -> Self {
    Self { value: w }
  }

  pub fn make_local_pid(pindex: Word) -> Self {
    Self::make_from_tag_and_value(TERMTAG_LOCALPID, pindex)
  }

  pub fn make_remote_pid(hp: &mut Heap, node: Self, pindex: Word) -> RtResult<Self> {
    let rpid_ptr = boxed::ExternalPid::create_into(hp, node, pindex)?;
    Ok(Self::make_boxed(rpid_ptr))
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

  #[inline]
  pub fn special_value_fits(n: Word) -> bool {
    let max = 1 << (WORD_BITS - TERM_TAG_BITS - TERM_SPECIAL_TAG_BITS);
    max > n
  }

  pub const fn make_special(special_t: SpecialTag, val: Word) -> Self {
    Self::make_from_tag_and_value(
      TERMTAG_SPECIAL,
      val << TERM_SPECIAL_TAG_BITS | special_t.0,
    )
  }

  #[inline]
  pub fn is_special(self) -> bool {
    self.get_term_tag() == TERMTAG_SPECIAL
  }

  #[inline]
  pub fn is_special_of_type(self, t: SpecialTag) -> bool {
    self.is_special() && self.get_special_tag() == t
  }

  pub fn make_regx(n: Word) -> Self {
    Self::make_special(SPECIALTAG_REGX, n)
  }

  pub fn is_regx(self) -> bool {
    self.is_special_of_type(SPECIALTAG_REGX)
  }

  pub fn make_regy(n: Word) -> Self {
    Self::make_special(SPECIALTAG_REGY, n)
  }

  pub fn is_regy(self) -> bool {
    self.is_special_of_type(SPECIALTAG_REGY)
  }

  pub fn make_regfp(n: Word) -> Self {
    Self::make_special(SPECIALTAG_REGFP, n)
  }

  pub fn is_regfp(self) -> bool {
    self.is_special_of_type(SPECIALTAG_REGFP)
  }

  // === === Code Pointer (Continuation Pointer) === ===
  //

  // XXX: Can shift value right by 3 bits (WORD_ALIGN_SHIFT)
  #[inline]
  pub fn make_cp<T>(p: *const T) -> Self {
    assert_eq!(p as Word & TERM_TAG_MASK, 0); // must be aligned to 8
    let tagged_p = (p as Word) | HIGHEST_BIT_CP;
    Self::from_raw(tagged_p)
  }

  #[inline]
  pub fn is_cp(self) -> bool {
    if !self.is_boxed() {
      return false;
    }
    self.value & HIGHEST_BIT_CP == HIGHEST_BIT_CP
  }

  pub fn get_cp_ptr<T>(self) -> *const T {
    debug_assert_eq!(self.value & HIGHEST_BIT_CP, HIGHEST_BIT_CP);
    (self.value & (HIGHEST_BIT_CP - 1)) as *const T
  }

  // === === TUPLES === === ===
  //

  pub fn is_tuple(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_TUPLE)
  }

  // This function only has debug check, in release it will not do any checking
  #[inline]
  pub fn get_tuple_ptr(self) -> *const boxed::Tuple {
    debug_assert!(self.is_tuple(), "Value is not a tuple: {}", self);
    (self.value & (!TERM_TAG_MASK)) as *const boxed::Tuple
  }

  // This function only has debug check, in release it will not do any checking
  #[inline]
  pub fn get_tuple_ptr_mut(self) -> *mut boxed::Tuple {
    debug_assert!(self.is_tuple(), "Value is not a tuple: {}", self);
    (self.value & (!TERM_TAG_MASK)) as *mut boxed::Tuple
  }

  // === === LISTS/CONS CELLS === === ===
  //

  #[inline]
  pub fn is_list(self) -> bool {
    self == Self::nil() || self.is_cons()
  }

  /// Check whether the value is a CONS pointer
  #[inline]
  pub fn is_cons(self) -> bool {
    self.get_term_tag() == TERMTAG_CONS
  }

  #[inline]
  pub fn get_cons_ptr(self) -> *const boxed::Cons {
    self.assert_is_not_boxheader_guard_value();
    debug_assert!(self.is_cons(), "Value is not a cons: {}", self);
    (self.value & (!TERM_TAG_MASK)) as *const boxed::Cons
  }

  #[inline]
  pub fn get_cons_ptr_mut(self) -> *mut boxed::Cons {
    self.assert_is_not_boxheader_guard_value();
    debug_assert!(self.is_cons(), "Value is not a cons: {}", self);
    (self.value & (!TERM_TAG_MASK)) as *mut boxed::Cons
  }

  /// When a box header guard value is interpreted as a term, it will be caught here
  #[cfg(debug_assertions)]
  #[inline]
  fn assert_is_not_boxheader_guard_value(&self) {
    debug_assert_ne!(self.value, box_header::GUARD_WORD_VALUE,
      "Box header guard value cannot be interpreted as a term. Your pointer to data is corrupt.");
  }

  #[cfg(not(debug_assertions))]
  #[inline]
  const fn assert_is_not_boxheader_guard_value(&self) {}

  /// Create a LTerm from pointer to Cons cell. Pass a pointer to `LTerm` or
  /// a pointer to `boxed::Cons`. Attempting to create cons cell to Null pointer
  /// will create NIL (`[]`)
  #[inline]
  pub fn make_cons<T>(p: *const T) -> Self {
    Self {
      value: if p.is_null() {
        return Self::nil();
      } else {
        (p as Word) | TERMTAG_CONS.0
      },
    }
  }

  pub unsafe fn cons_is_ascii_string(self) -> bool {
    debug_assert!(self.is_cons());

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
        return tl == Self::nil();
      }
      cons_p = tl.get_cons_ptr();
    }
  }

  // === === SMALL INTEGERS === === ===
  //

  #[inline]
  pub fn is_integer(self) -> bool {
    self.is_small() || self.is_big_int()
  }

  /// Check whether the value is a small integer
  #[inline]
  pub fn is_small(self) -> bool {
    self.get_term_tag() == TERMTAG_SMALL
  }

  #[inline]
  pub const fn make_char(c: char) -> Self {
    Self::make_small_unsigned(c as usize)
  }

  #[inline]
  pub const fn make_small_unsigned(val: Word) -> Self {
    Self::make_from_tag_and_value(TERMTAG_SMALL, val)
  }

  pub const fn small_0() -> Self {
    Self::make_from_tag_and_value(TERMTAG_SMALL, 0)
  }

  pub const fn small_1() -> Self {
    Self::make_from_tag_and_value(TERMTAG_SMALL, 1)
  }

  pub const fn make_small_signed(val: SWord) -> Self {
    Self::make_from_tag_and_signed_value(TERMTAG_SMALL, val)
  }

  /// Check whether a signed isize fits into small integer range
  #[inline]
  pub fn small_fits(val: isize) -> bool {
    val >= SMALLEST_SMALL && val <= LARGEST_SMALL
  }

  #[inline]
  pub fn get_small_signed(self) -> SWord {
    debug_assert!(self.is_small(), "Small is expected, got raw=0x{:x}", self.value);
    (self.value as SWord) >> TERM_TAG_BITS
  }

  #[inline]
  pub fn get_small_unsigned(self) -> Word {
    debug_assert!(self.is_small());
    self.get_term_val_without_tag()
  }

  // === === BIG INTEGERS === ===
  //

  /// Check whether a lterm is boxed and then whether it points to a word of
  /// memory tagged as float
  pub fn is_big_int(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_BIGINTEGER)
  }

  // === === Atoms === ===
  //

  pub fn atom_index(self) -> Word {
    debug_assert!(self.is_atom());
    self.get_term_val_without_tag()
  }

  // === === FLOAT === ===
  //

  /// Check whether a value is a small integer, a big integer or a float.
  pub fn is_number(self) -> bool {
    self.is_small()
      || self.is_boxed_of_(|t| {
        t == boxed::BOXTYPETAG_BIGINTEGER || t == boxed::BOXTYPETAG_FLOAT
      })
  }

  /// Check whether a lterm is boxed and then whether it points to a word of
  /// memory tagged as float
  pub fn is_float(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_FLOAT)
  }

  pub fn get_f64(self) -> RtResult<f64> {
    if !self.is_boxed() {
      return Err(RtErr::TermIsNotABoxed);
    }
    let _p = self.get_box_ptr::<BoxHeader>();
    unimplemented!("float box")
  }

  /// Returns float value, performs no extra checks. The caller is responsible
  /// for the value being a boxed float.
  #[inline]
  pub unsafe fn get_f64_unsafe(self) -> f64 {
    let p = self.get_box_ptr::<boxed::Float>();
    (*p).value
  }

  /// Raw compare two term values.
  pub fn is_same(a: Self, b: Self) -> bool {
    a.raw() == b.raw()
  }

  // === === PORT === === ===
  //

  /// Check whether a value is any kind of port.
  pub fn is_port(self) -> bool {
    self.is_local_port() || self.is_external_port()
  }

  pub fn is_local_port(self) -> bool {
    false
  }

  pub fn is_external_port(self) -> bool {
    false
  }

  // === === MAP === ===
  //

  /// Check whether a value is a map.
  pub fn is_map(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_MAP)
  }

  /// Check whether a value is a small map < 32 elements (Flat). Does NOT check
  /// that the value is a map (assert!) assuming that the caller has checked
  /// it by now.
  pub fn is_flat_map(self) -> bool {
    false
  }

  /// Check whether a value is a hash map >= 32 elements (HAMT). Does NOT check
  /// that the value is a map (assert!) assuming that the caller has checked
  /// it by now.
  pub fn is_hash_map(self) -> bool {
    false
  }

  pub fn map_size(self) -> usize {
    0
  }

  // === === EXPORT === ===
  //

  /// Check whether a value is a boxed export (M:F/Arity triple).
  pub fn is_export(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_EXPORT)
  }

  // === === FUN / CLOSURE === ===
  //

  /// Check whether a value is a boxed fun (a closure or export).
  pub fn is_fun(self) -> bool {
    self.is_boxed_of_(|t| t == boxed::BOXTYPETAG_CLOSURE || t == boxed::BOXTYPETAG_EXPORT)
  }

  /// Check whether a value is a boxed fun (a closure or export).
  pub fn is_fun_of_arity(self, a: usize) -> bool {
    if !self.is_boxed() {
      return false;
    }

    let box_ptr = self.get_box_ptr::<BoxHeader>();
    let trait_ptr = unsafe { (*box_ptr).get_trait_ptr() };
    let box_type = unsafe { (*trait_ptr).get_type() };

    match box_type {
      boxed::BOXTYPETAG_CLOSURE => {
        let closure_p = box_ptr as *const boxed::Closure;
        unsafe { (*closure_p).mfa.arity - (*closure_p).nfrozen == a }
      }
      boxed::BOXTYPETAG_EXPORT => {
        let expt_p = box_ptr as *const boxed::Export;
        unsafe { (*expt_p).exp.mfa.arity == a }
      }
      _ => false,
    }
  }

  // === === REFERENCES === ===
  //

  /// Check whether a value is any kind of reference.
  pub fn is_ref(self) -> bool {
    self.is_local_ref() || self.is_external_ref()
  }

  pub fn is_local_ref(self) -> bool {
    false
  }

  pub fn is_external_ref(self) -> bool {
    false
  }

  // === ===  BOOLEAN === ===
  //

  #[inline]
  pub fn is_bool(self) -> bool {
    self.is_true() || self.is_false()
  }

  #[inline]
  pub fn make_bool(v: bool) -> Self {
    if v {
      return gen_atoms::TRUE;
    }
    gen_atoms::FALSE
  }

  #[inline]
  pub const fn is_true(self) -> bool {
    self.value == gen_atoms::TRUE.raw()
  }

  #[inline]
  pub const fn is_false(self) -> bool {
    self.value == gen_atoms::FALSE.raw()
  }

  //
  //=== === CATCH VALUES === ===
  //

  /// Create a catch marker on stack
  #[inline]
  pub fn make_catch(p: *const Word) -> Self {
    let catch_index = (p as Word) >> WORD_ALIGN_SHIFT;
    assert!(Self::special_value_fits(catch_index));
    // TODO: Use some smart solution for handling code reloading
    Self::make_special(SPECIALTAG_CATCH, catch_index)
  }

  #[inline]
  pub fn is_catch(self) -> bool {
    self.is_special_of_type(SPECIALTAG_CATCH)
  }

  #[inline]
  pub fn get_catch_ptr(self) -> *const Word {
    assert!(self.is_catch(), "Attempt to get_catch_ptr on {}", self);
    let val = self.get_special_value() << WORD_ALIGN_SHIFT;
    val as *const Word
  }
}

// Testing section
//

//#[cfg(test)]
// mod tests {
//  use core::ptr;
//  use std::mem;
//
//  use defs::*;
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
  box_type: BoxType,
  err: RtErr,
) -> RtResult<*const T> {
  if !t.is_boxed() {
    return Err(RtErr::TermIsNotABoxed);
  }

  let cptr = t.get_box_ptr::<T>();
  let box_ptr = cptr as *const BoxHeader;
  let trait_ptr = (*box_ptr).get_trait_ptr();

  if (*trait_ptr).get_type() != box_type {
    return Err(err);
  }
  Ok(cptr)
}

pub unsafe fn helper_get_mut_from_boxed_term<T>(
  t: LTerm,
  box_type: BoxType,
  _err: RtErr,
) -> RtResult<*mut T> {
  debug_assert!(t.is_boxed());

  let cptr = t.get_box_ptr_mut::<T>();
  let box_ptr = cptr as *const BoxHeader;
  let trait_ptr = (*box_ptr).get_trait_ptr();

  if (*trait_ptr).get_type() == box_type {
    return Ok(cptr);
  }
  Err(RtErr::BoxedTagCheckFailed)
}

impl fmt::Debug for LTerm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}
