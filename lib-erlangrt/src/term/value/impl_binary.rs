use crate::{
  defs::ByteSize,
  term::{
    boxed,
    value::{Term, SPECIALCONST_EMPTYBINARY, SPECIALTAG_CONST},
  },
};

impl Term {
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
}
