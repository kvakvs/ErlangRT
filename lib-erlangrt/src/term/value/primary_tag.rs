//! Primary tag module contains definitions for the first 3 bits of a term. This
//! is specific to 64 bit platform.
// Structure of term:
// Boxed: [Pointer bits] [PrimaryTag::BOX_PTR|CONS_PTR 3 bits]
// Immediates: [Value] [PrimaryTag::ATOM|PID|PORT|SMALL 3 bits]
// Specials Reg/Loadtime: [Value] [SpecialReg|Loadtime Tag 2 bits]...
//          ... [SpecialTag::* 3 bits] [PrimaryTag::SPECIAL 3 bits]
// Special Consts etc: [Value] [SpecialTag::* 3 bits] [PrimaryTag::SPECIAL 3 bits]

/// This thing is valid for 64-bit platform only, which allows us to use 3 bits
/// guaranteed to be zero for all aligned addresses. As 32-bit platforms only
/// have 2 guaranteed bits, we will have to use compressed pointers or reorder
/// the tag bits to use only 2 bits.
#[cfg(target_pointer_width = "64")]
#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct PrimaryTag(pub usize);

#[cfg(target_pointer_width = "64")]
impl PrimaryTag {
  pub const TAG_BITS: usize = 3;
  pub const TAG_MASK: usize = (1 << Self::TAG_BITS) - 1;

  pub const SMALL_INT: Self = Self(0); // immediate, not a pointer
  pub const HEADER: Self = Self(1); // immediate, first word of a memory block
  pub const CONS_PTR: Self = Self(2); // a pointer
  pub const BOX_PTR: Self = Self(3); // a pointer
  pub const ATOM: Self = Self(4); // immediate, not a pointer
  pub const LOCAL_PID: Self = Self(5); // immediate, not a pointer
  pub const LOCAL_PORT: Self = Self(6); // immediate, not a pointer
  pub const SPECIAL: Self = Self(7); // immediate, not a pointer
}
