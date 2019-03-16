//! Primary tag module contains definitions for the first 3 bits of a term. This
//! is specific to 64 bit platform.
// Structure of term:
// [ Value or a pointer ] [ TAG_* value 3 bits ]
//
pub const TERM_TAG_BITS: usize = 3;
pub const TERM_TAG_MASK: usize = (1 << TERM_TAG_BITS) - 1;

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct PrimaryTag(pub usize);

/// This thing is valid for 64bit platform only, which allows us to use 3 bits
/// guaranteed to be zero for all aligned addresses.
#[cfg(target_pointer_width = "64")]
impl PrimaryTag {
  pub const SMALL_INT: Self = Self(0);
  pub const HEADER: Self = Self(1);
  pub const CONS_PTR: Self = Self(2);
  // From here and below, values are immediate (fit into a single word)
  pub const BOX_PTR: Self = Self(3);
  pub const ATOM: Self = Self(4);
  pub const LOCAL_PID: Self = Self(5);
  pub const LOCAL_PORT: Self = Self(6);
  pub const SPECIAL: Self = Self(7);

  #[inline]
  pub const fn get(self) -> usize {
    self.0
  }
}
