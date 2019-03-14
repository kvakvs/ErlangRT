use crate::{
  defs,
  term::value::{tag_term::TERM_TAG_BITS, Term, TERMTAG_SPECIAL},
};

// Structure of SPECIAL values,
// they are plethora of term types requiring fewer bits or useful in other ways
// [ special value ] [ VAL_SPECIAL_... 3 bits ] [ TAG_SPECIAL 3 bits ]
//
pub const TERM_SPECIAL_TAG_BITS: usize = 3;
pub const TERM_SPECIAL_TAG_MASK: usize = (1 << TERM_SPECIAL_TAG_BITS) - 1;

#[derive(Eq, PartialEq, Debug)]
pub struct SpecialTag(pub usize);

// special constants such as NIL, empty tuple, binary etc
pub const SPECIALTAG_CONST: SpecialTag = SpecialTag(0);
pub const SPECIALTAG_REG: SpecialTag = SpecialTag(1);
/// Catch tag contains index in the catch table of the current module
pub const SPECIALTAG_CATCH: SpecialTag = SpecialTag(2);
// decorates opcodes for easier code walking
pub const SPECIALTAG_OPCODE: SpecialTag = SpecialTag(3);
pub const SPECIALTAG_LOADTIME: SpecialTag = SpecialTag(4);
// unused 5
// unused 6
// unused 7
//-- End of 3-bit space for special tags

pub struct SpecialConst(pub usize);

pub const SPECIALCONST_EMPTYTUPLE: SpecialConst = SpecialConst(0);
pub const SPECIALCONST_EMPTYLIST: SpecialConst = SpecialConst(1);
pub const SPECIALCONST_EMPTYBINARY: SpecialConst = SpecialConst(2);

/// Used as prefix for special value in register index
#[derive(Eq, PartialEq, Debug)]
pub struct SpecialReg(pub usize);

/// How many bits are reserved in special value, if the special is a register index
const SPECIAL_REG_TAG_BITS: usize = 2;

/// How many bits are remaining in the machine word after taking away the prefix bits
#[allow(dead_code)]
pub const SPECIAL_REG_RESERVED_BITS: usize =
  SPECIAL_REG_TAG_BITS + TERM_TAG_BITS + TERM_SPECIAL_TAG_BITS;

pub const SPECIALREG_X: SpecialReg = SpecialReg(0); // register x
pub const SPECIALREG_Y: SpecialReg = SpecialReg(1); // register y
pub const SPECIALREG_FP: SpecialReg = SpecialReg(2); // float register

/// Used as prefix for special value in loadtime index
/// Loadtime value contains loadtime tag + loadtime value following after it
#[derive(Eq, PartialEq, Debug)]
pub struct SpecialLoadTime(pub usize);

pub const SPECIAL_LT_TAG_BITS: usize = 2;
/// How many bits are remaining in the machine word after taking away the prefix bits
pub const SPECIAL_LT_RESERVED_BITS: usize =
  SPECIAL_LT_TAG_BITS + TERM_TAG_BITS + TERM_SPECIAL_TAG_BITS;
pub const SPECIAL_LT_ATOM: SpecialLoadTime = SpecialLoadTime(0); // atom table index
pub const SPECIAL_LT_LABEL: SpecialLoadTime = SpecialLoadTime(1); // label table index
pub const SPECIAL_LT_LITERAL: SpecialLoadTime = SpecialLoadTime(2); // literal table index

impl Term {
  pub fn make_register_x(n: usize) -> Self {
    Self::make_special(SPECIALTAG_REG, n << SPECIAL_REG_TAG_BITS | SPECIALREG_X.0)
  }

  pub fn is_register_x(self) -> bool {
    self.is_special_of_type(SPECIALTAG_REG) && self.get_reg_tag() == SPECIALREG_X
  }

  pub fn make_register_y(n: usize) -> Self {
    Self::make_special(SPECIALTAG_REG, n << SPECIAL_REG_TAG_BITS | SPECIALREG_Y.0)
  }

  pub fn is_register_y(self) -> bool {
    self.is_special_of_type(SPECIALTAG_REG) && self.get_reg_tag() == SPECIALREG_Y
  }

  pub fn make_register_float(n: usize) -> Self {
    Self::make_special(SPECIALTAG_REG, n << SPECIAL_REG_TAG_BITS | SPECIALREG_FP.0)
  }

  pub fn is_register_float(self) -> bool {
    self.is_special_of_type(SPECIALTAG_REG) && self.get_reg_tag() == SPECIALREG_FP
  }

  /// For register special, retrieve tag bits which are stored in the special value
  pub fn get_reg_tag(self) -> SpecialReg {
    SpecialReg(self.get_special_value() & (1 << SPECIAL_REG_TAG_BITS - 1))
  }

  /// For register special, retrieve value bits which are stored in the special value
  pub fn get_reg_value(self) -> usize {
    self.get_special_value() >> SPECIAL_REG_TAG_BITS
  }

  #[inline]
  pub fn is_special(self) -> bool {
    self.get_term_tag() == TERMTAG_SPECIAL
  }

  #[inline]
  pub fn is_special_of_type(self, t: SpecialTag) -> bool {
    self.is_special() && self.get_special_tag() == t
  }

  /// For a special-tagged term extract its special tag
  pub fn get_special_tag(self) -> SpecialTag {
    debug_assert_eq!(self.get_term_tag(), TERMTAG_SPECIAL);
    // cut away term tag bits and extract special tag
    SpecialTag((self.value >> TERM_TAG_BITS) & TERM_SPECIAL_TAG_MASK)
  }

  /// From a special-tagged term extract its value
  pub fn get_special_value(self) -> usize {
    debug_assert_eq!(self.get_term_tag(), TERMTAG_SPECIAL);
    // cut away term tag bits and special tag, extract the remaining value bits
    self.value >> (TERM_TAG_BITS + TERM_SPECIAL_TAG_BITS)
  }

  #[inline]
  pub fn special_value_fits(n: usize) -> bool {
    let max = 1 << (defs::WORD_BITS - TERM_TAG_BITS - TERM_SPECIAL_TAG_BITS);
    max > n
  }

  pub const fn make_special(special_t: SpecialTag, val: usize) -> Self {
    Self::make_from_tag_and_value(
      TERMTAG_SPECIAL,
      val << TERM_SPECIAL_TAG_BITS | special_t.0,
    )
  }

  // === === SPECIAL - Load Time ATOM, LABEL, LITERAL indices === ===
  // These exist only during loading time and then must be converted to real
  // values using the lookup tables included in the BEAM file.
  //
  #[inline]
  fn make_loadtime(tag: SpecialLoadTime, n: usize) -> Self {
    debug_assert!(n < (1usize << (defs::WORD_BITS - SPECIAL_LT_RESERVED_BITS)));
    Self::make_special(SPECIALTAG_LOADTIME, n << SPECIAL_LT_TAG_BITS | tag.0)
  }

  #[inline]
  pub fn make_loadtime_atom(n: usize) -> Self {
    Self::make_loadtime(SPECIAL_LT_ATOM, n)
  }

  #[inline]
  pub fn make_loadtime_label(n: usize) -> Self {
    Self::make_loadtime(SPECIAL_LT_LABEL, n)
  }

  #[inline]
  pub fn make_loadtime_literal(n: usize) -> Self {
    Self::make_loadtime(SPECIAL_LT_LITERAL, n)
  }

  #[inline]
  pub fn is_loadtime(self) -> bool {
    self.is_special_of_type(SPECIALTAG_LOADTIME)
  }

  #[inline]
  pub fn get_loadtime_val(self) -> usize {
    debug_assert!(self.is_loadtime(), "Must be a loadtime value, got {}", self);
    self.get_special_value() >> SPECIAL_LT_TAG_BITS
  }

  #[inline]
  pub fn get_loadtime_tag(self) -> SpecialLoadTime {
    debug_assert!(self.is_loadtime(), "Must be a loadtime value, got {}", self);
    SpecialLoadTime(self.get_special_value() & (1 << SPECIAL_LT_TAG_BITS - 1))
  }
}
