//! Low level term library
//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance

pub mod cons;
mod format;
mod impl_binary;
mod impl_boxed;
mod impl_cons;
mod impl_fun;
mod impl_map;
mod impl_number;
mod impl_tuple;
mod special;
mod tag_term;
pub mod tuple;

pub use self::{
  impl_binary::*, impl_boxed::*, impl_cons::*, impl_fun::*, impl_map::*, impl_number::*,
  impl_tuple::*, special::*, tag_term::*,
};
use crate::{
  defs,
  emulator::{gen_atoms, heap::Heap},
  fail::{RtErr, RtResult},
  term::boxed::{self, box_header, BoxHeader, BoxType},
};
use core::{cmp::Ordering, isize};
use std::fmt;

/// Max value for a positive small integer packed into immediate2 low level
/// Term. Assume word size minus 4 bits for imm1 tag and 1 for sign
pub const SMALLEST_SMALL: isize = isize::MIN >> TERM_TAG_BITS;
pub const LARGEST_SMALL: isize = isize::MAX >> TERM_TAG_BITS;
#[allow(dead_code)]
pub const SMALL_SIGNED_BITS: usize = defs::WORD_BITS - TERM_TAG_BITS - 1;

/// A low-level term is either a pointer to memory term or an Immediate with
/// leading bits defining its type (see TAG_* consts below).
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Term {
  value: usize, // Contains a pointer or an integer
}

impl Ord for Term {
  fn cmp(&self, other: &Term) -> Ordering {
    self.value.cmp(&other.value)
  }
}

impl PartialOrd for Term {
  fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
    Some(self.value.cmp(&other.value))
  }
}

// TODO: Remove deadcode directive later and fix
#[allow(dead_code)]
impl Term {
  /// Retrieve the raw value of a `Term` as usize, including tag bits
  /// and everything.
  #[inline]
  pub const fn raw(self) -> usize {
    self.value
  }

  #[inline]
  pub const fn make_atom(id: usize) -> Self {
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

  pub const fn make_from_tag_and_value(t: TermTag, v: usize) -> Self {
    Self::from_raw(v << TERM_TAG_BITS | t.0)
  }

  pub const fn make_from_tag_and_signed_value(t: TermTag, v: isize) -> Self {
    Self::from_raw((v << TERM_TAG_BITS | (t.0 as isize)) as usize)
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

  /// Check whether a term is boxed and then whether it points to a word of
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
  pub fn get_term_val_without_tag(self) -> usize {
    debug_assert!(
      self.get_term_tag() != TERMTAG_BOXED && self.get_term_tag() != TERMTAG_CONS
    );
    self.value >> TERM_TAG_BITS
  }

  // === === CONSTRUCTION === === ===
  //

  /// Any raw word becomes a term, possibly invalid
  pub const fn from_raw(w: usize) -> Self {
    Self { value: w }
  }

  pub fn make_local_pid(pindex: usize) -> Self {
    Self::make_from_tag_and_value(TERMTAG_LOCALPID, pindex)
  }

  pub fn make_remote_pid(hp: &mut Heap, node: Self, pindex: usize) -> RtResult<Self> {
    let rpid_ptr = boxed::ExternalPid::create_into(hp, node, pindex)?;
    Ok(Self::make_boxed(rpid_ptr))
  }

  // === === Code Pointer (Continuation Pointer) === ===
  //

  // XXX: Can shift value right by 3 bits (WORD_ALIGN_SHIFT)
  #[inline]
  pub fn make_cp<T>(p: *const T) -> Self {
    assert_eq!(p as usize & TERM_TAG_MASK, 0); // must be aligned to 8
    let tagged_p = (p as usize) | defs::HIGHEST_BIT_CP;
    Self::from_raw(tagged_p)
  }

  #[inline]
  pub fn is_cp(self) -> bool {
    if !self.is_boxed() {
      return false;
    }
    self.value & defs::HIGHEST_BIT_CP == defs::HIGHEST_BIT_CP
  }

  pub fn get_cp_ptr<T>(self) -> *const T {
    debug_assert_eq!(self.value & defs::HIGHEST_BIT_CP, defs::HIGHEST_BIT_CP);
    (self.value & (defs::HIGHEST_BIT_CP - 1)) as *const T
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

  // === === ATOMS === ===
  //

  pub fn atom_index(self) -> usize {
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

  /// Constructor to create a float on heap. May fail if the heap is full or
  /// something else might happen.
  pub fn make_float(hp: &mut Heap, val: f64) -> RtResult<Self> {
    let pf = unsafe { boxed::Float::create_into(hp, val)? };
    Ok(Self::make_boxed(pf))
  }

  /// Check whether a term is boxed and then whether it points to a word of
  /// memory tagged as float
  pub fn is_float(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_FLOAT)
  }

  pub fn get_float(self) -> RtResult<f64> {
    if !self.is_boxed() {
      return Err(RtErr::TermIsNotABoxed);
    }
    let _p = self.get_box_ptr::<BoxHeader>();
    unimplemented!("float box")
  }

  /// Returns float value, performs no extra checks. The caller is responsible
  /// for the value being a boxed float.
  #[inline]
  pub unsafe fn get_float_unchecked(self) -> f64 {
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
  pub fn make_catch(p: *const usize) -> Self {
    let catch_index = (p as usize) >> defs::WORD_ALIGN_SHIFT;
    debug_assert!(Self::special_value_fits(catch_index));
    // TODO: Use some smart solution for handling code reloading
    Self::make_special(SPECIALTAG_CATCH, catch_index)
  }

  #[inline]
  pub fn is_catch(self) -> bool {
    self.is_special_of_type(SPECIALTAG_CATCH)
  }

  #[inline]
  pub fn get_catch_ptr(self) -> *const usize {
    debug_assert!(self.is_catch(), "Attempt to get_catch_ptr on {}", self);
    let val = self.get_special_value() << defs::WORD_ALIGN_SHIFT;
    val as *const usize
  }
}

pub unsafe fn helper_get_const_from_boxed_term<T>(
  t: Term,
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
  t: Term,
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

impl fmt::Debug for Term {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}
