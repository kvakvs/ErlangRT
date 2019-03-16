//! Low level term library
//!
//! Low level term represents memory layout of Term bits to store the data
//! as compact as possible while maintaining an acceptable performance

pub mod cons;
mod format;
mod impl_binary;
mod impl_boxed;
mod impl_cons;
mod impl_cp;
mod impl_float;
mod impl_fun;
mod impl_map;
mod impl_nonvalue;
mod impl_number;
mod impl_special;
mod impl_tuple;
mod primary_tag;

pub use self::{
  impl_binary::*, impl_boxed::*, impl_cons::*, impl_cp::*, impl_float::*, impl_fun::*,
  impl_map::*, impl_nonvalue::*, impl_number::*, impl_special::*, impl_tuple::*,
  primary_tag::*,
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
pub const SMALLEST_SMALL: isize = isize::MIN >> PrimaryTag::TAG_BITS;
pub const LARGEST_SMALL: isize = isize::MAX >> PrimaryTag::TAG_BITS;
#[allow(dead_code)]
pub const SMALL_SIGNED_BITS: usize = defs::WORD_BITS - PrimaryTag::TAG_BITS - 1;

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
    Self::make_from_tag_and_value(PrimaryTag::ATOM, id)
  }

  #[inline]
  pub const fn empty_tuple() -> Self {
    Self::make_special(SpecialTag::CONST, SpecialConst::EMPTY_TUPLE.0)
  }

  #[inline]
  pub const fn nil() -> Self {
    Self::make_special(SpecialTag::CONST, SpecialConst::EMPTY_LIST.0)
  }

  pub const fn make_from_tag_and_value(t: PrimaryTag, v: usize) -> Self {
    Self::from_raw(v << PrimaryTag::TAG_BITS | t.0)
  }

  pub const fn make_from_tag_and_signed_value(t: PrimaryTag, v: isize) -> Self {
    Self::from_raw((v << PrimaryTag::TAG_BITS | (t.0 as isize)) as usize)
  }

  /// Get tag bits from the p field as integer.
  #[inline]
  pub const fn get_term_tag(self) -> PrimaryTag {
    PrimaryTag(self.value & PrimaryTag::TAG_MASK)
  }

  //

  #[inline]
  pub fn is_immediate(self) -> bool {
    self.get_term_tag() != PrimaryTag::BOX_PTR
  }

  /// Check whether the value is tagged as atom
  #[inline]
  pub fn is_atom(self) -> bool {
    self.get_term_tag() == PrimaryTag::ATOM
  }

  #[inline]
  pub fn is_pid(self) -> bool {
    self.is_local_pid() || self.is_external_pid()
  }

  #[inline]
  pub fn is_local_pid(self) -> bool {
    self.get_term_tag() == PrimaryTag::LOCAL_PID
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
    self.get_term_tag() == PrimaryTag::SPECIAL
  }

  /// For non-pointer Term types get the encoded integer without tag bits
  #[inline]
  pub fn get_term_val_without_tag(self) -> usize {
    debug_assert!(
      self.get_term_tag() != PrimaryTag::BOX_PTR
        && self.get_term_tag() != PrimaryTag::CONS_PTR
    );
    self.value >> PrimaryTag::TAG_BITS
  }

  // === === CONSTRUCTION === === ===
  //

  /// Any raw word becomes a term, possibly invalid
  pub const fn from_raw(w: usize) -> Self {
    Self { value: w }
  }

  pub fn make_local_pid(pindex: usize) -> Self {
    Self::make_from_tag_and_value(PrimaryTag::LOCAL_PID, pindex)
  }

  pub fn make_remote_pid(hp: &mut Heap, node: Self, pindex: usize) -> RtResult<Self> {
    let rpid_ptr = boxed::ExternalPid::create_into(hp, node, pindex)?;
    Ok(Self::make_boxed(rpid_ptr))
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
    assert!(Self::special_value_fits(catch_index));
    // TODO: Use some smart solution for handling code reloading
    Self::make_special(SpecialTag::CATCH, catch_index)
  }

  #[inline]
  pub fn is_catch(self) -> bool {
    self.is_special_of_type(SpecialTag::CATCH)
  }

  #[inline]
  pub fn get_catch_ptr(self) -> *const usize {
    assert!(self.is_catch(), "Attempt to get_catch_ptr on {}", self);
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
