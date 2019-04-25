use crate::{
  defs,
  term::value::{PrimaryTag, Term},
};

// Structure of SPECIAL values,
// they are plethora of term types requiring fewer bits or useful in other ways
// [ special value ] [ VAL_SPECIAL_... 3 bits ] [ TAG_SPECIAL 3 bits ]
//
pub const TERM_SPECIAL_TAG_BITS: usize = 3;
pub const TERM_SPECIAL_TAG_MASK: usize = (1 << TERM_SPECIAL_TAG_BITS) - 1;

#[derive(Eq, PartialEq, Debug)]
pub struct SpecialTag(pub usize);

impl SpecialTag {
  // special constants such as NIL, empty tuple, binary etc
  pub const CONST: Self = Self(0);
  pub const REG: Self = Self(1);
  /// Catch tag contains index in the catch table of the current module
  pub const CATCH: Self = Self(2);
  // decorates opcodes for easier code walking. Used only in debug.
  #[cfg(debug_assertions)]
  pub const OPCODE: Self = Self(3);
  pub const LOAD_TIME: Self = Self(4);
  // unused 5
  // unused 6
  // unused 7
  //-- End of 3-bit space for special tags
}

/// Marks special constant.
/// Structure: `[Value] [SpecialTag::* 3 bits] [PrimaryTag::SPECIAL 3 bits]`
pub struct SpecialConst(pub usize);

impl SpecialConst {
  pub const EMPTY_TUPLE: Self = Self(0);
  pub const EMPTY_LIST: Self = Self(1);
  pub const EMPTY_BINARY: Self = Self(2);
  // unused 3
}

/// Used as prefix for special value in register index
#[derive(Eq, PartialEq, Debug)]
pub struct SpecialReg(pub usize);

impl SpecialReg {
  /// How many bits are reserved in special value, if the special is a register index
  const TAG_BITS: usize = 2;

  /// How many bits are remaining in the machine word after taking away the prefix bits
  #[allow(dead_code)]
  pub const WORD_RESERVED_BITS: usize =
    SpecialReg::TAG_BITS + PrimaryTag::TAG_BITS + TERM_SPECIAL_TAG_BITS;

  // Begin `SpecialReg::TAG_BITS`-bit space
  pub const REG_X: Self = Self(0); // register x
  pub const REG_Y: Self = Self(1); // register y
  pub const REG_FLOAT: Self = Self(2); // float register

  // unused 3
}

/// Used as prefix for special value in loadtime index
/// Loadtime value contains loadtime tag + loadtime value following after it
#[derive(Eq, PartialEq, Debug)]
pub struct SpecialLoadtime(pub usize);

impl SpecialLoadtime {
  /// How many bits are reserved in special value to store loadtime tag
  pub const TAG_BITS: usize = 2;
  /// How many bits are remaining in the machine word after taking away the prefix bits
  pub const RESERVED_BITS: usize =
    Self::TAG_BITS + PrimaryTag::TAG_BITS + TERM_SPECIAL_TAG_BITS;

  pub const ATOM: Self = Self(0); // atom table index
  pub const LABEL: Self = Self(1); // label table index
  pub const LITERAL: Self = Self(2); // literal table index

  // unused 3
}

impl Term {
  // === === === Register index values === ===
  //

  #[inline]
  fn make_special_register(tag: SpecialReg, n: usize) -> Self {
    debug_assert!(n < (1usize << (defs::WORD_BITS - SpecialReg::WORD_RESERVED_BITS)));
    debug_assert!(tag.0 < (1usize << SpecialReg::TAG_BITS));
    let special_val = n << SpecialReg::TAG_BITS | tag.0;
    Self::make_special_safe(SpecialTag::REG, special_val)
  }

  #[inline]
  pub fn make_register_x(n: usize) -> Self {
    Self::make_special_register(SpecialReg::REG_X, n)
  }

  #[inline]
  pub fn is_register_x(self) -> bool {
    self.is_special_of_type(SpecialTag::REG) && self.get_reg_tag() == SpecialReg::REG_X
  }

  #[inline]
  pub fn make_register_y(n: usize) -> Self {
    Self::make_special_register(SpecialReg::REG_Y, n)
  }

  #[inline]
  pub fn is_register_y(self) -> bool {
    self.is_special_of_type(SpecialTag::REG) && self.get_reg_tag() == SpecialReg::REG_Y
  }

  #[inline]
  pub fn make_register_float(n: usize) -> Self {
    Self::make_special_register(SpecialReg::REG_FLOAT, n)
  }

  #[inline]
  pub fn is_register_float(self) -> bool {
    self.is_special_of_type(SpecialTag::REG)
      && self.get_reg_tag() == SpecialReg::REG_FLOAT
  }

  /// For register special, retrieve tag bits which are stored in the special value
  #[inline]
  pub fn get_reg_tag(self) -> SpecialReg {
    let mask = (1usize << SpecialReg::TAG_BITS) - 1;
    SpecialReg(self.get_special_value() & mask)
  }

  /// For register special, retrieve value bits which are stored in the special value
  #[inline]
  pub fn get_reg_value(self) -> usize {
    debug_assert!(
      self.is_special_of_type(SpecialTag::REG),
      "A register value is expected, got {}",
      self
    );
    self.get_special_value() >> SpecialReg::TAG_BITS
  }

  #[inline]
  pub fn is_special(self) -> bool {
    self.get_term_tag() == PrimaryTag::SPECIAL
  }

  #[inline]
  pub fn is_special_of_type(self, t: SpecialTag) -> bool {
    self.is_special() && self.get_special_tag() == t
  }

  /// For a special-tagged term extract its special tag
  pub fn get_special_tag(self) -> SpecialTag {
    debug_assert_eq!(self.get_term_tag(), PrimaryTag::SPECIAL);
    // cut away term tag bits and extract special tag
    SpecialTag((self.value >> PrimaryTag::TAG_BITS) & TERM_SPECIAL_TAG_MASK)
  }

  /// From a special-tagged term extract its value
  /// NOTE: Ignores additional rules for storing registers and loadtime indices.
  pub fn get_special_value(self) -> usize {
    debug_assert_eq!(self.get_term_tag(), PrimaryTag::SPECIAL);
    // cut away term tag bits and special tag, extract the remaining value bits
    self.value >> (PrimaryTag::TAG_BITS + TERM_SPECIAL_TAG_BITS)
  }

  #[inline]
  pub const fn special_value_fits(n: usize) -> bool {
    let max = 1 << (defs::WORD_BITS - PrimaryTag::TAG_BITS - TERM_SPECIAL_TAG_BITS);
    max > n
  }

  #[inline]
  pub const fn make_special(special_t: SpecialTag, val: usize) -> Self {
    Self::make_from_tag_and_value(
      PrimaryTag::SPECIAL,
      val << TERM_SPECIAL_TAG_BITS | special_t.0,
    )
  }

  /// Calls make_special after a safety assertion
  #[inline]
  pub fn make_special_safe(special_t: SpecialTag, val: usize) -> Self {
    debug_assert!(
      Self::special_value_fits(val),
      "Value provided for crafting a special term is too large"
    );
    Self::make_special(special_t, val)
  }

  // === === OPCODE === ===
  // In debug only: Represents value in the opcode cell of an instruction
  //
  /// For opcode special, retrieve value bits which are stored in the special value
  #[cfg(debug_assertions)]
  #[inline]
  pub fn get_opcode_value(self) -> usize {
    debug_assert!(
      self.is_special_of_type(SpecialTag::OPCODE),
      "An opcode value is expected, got {}",
      self
    );
    self.get_special_value()
  }

  // === === SPECIAL - Load Time ATOM, LABEL, LITERAL indices === ===
  // These exist only during loading time and then must be converted to real
  // values using the lookup tables included in the BEAM file.
  //
  #[inline]
  fn make_special_loadtime(tag: SpecialLoadtime, n: usize) -> Self {
    debug_assert!(n < (1usize << (defs::WORD_BITS - SpecialLoadtime::RESERVED_BITS)));
    debug_assert!(tag.0 < (1usize << SpecialLoadtime::TAG_BITS));
    let special_val = n << SpecialLoadtime::TAG_BITS | tag.0;
    Self::make_special_safe(SpecialTag::LOAD_TIME, special_val)
  }

  #[inline]
  pub fn make_loadtime_atom(n: usize) -> Self {
    Self::make_special_loadtime(SpecialLoadtime::ATOM, n)
  }

  #[inline]
  pub fn make_loadtime_label(n: usize) -> Self {
    Self::make_special_loadtime(SpecialLoadtime::LABEL, n)
  }

  #[inline]
  pub fn make_loadtime_literal(n: usize) -> Self {
    Self::make_special_loadtime(SpecialLoadtime::LITERAL, n)
  }

  #[inline]
  pub fn is_loadtime(self) -> bool {
    self.is_special_of_type(SpecialTag::LOAD_TIME)
  }

  #[inline]
  pub fn get_loadtime_val(self) -> usize {
    debug_assert!(self.is_loadtime(), "Must be a loadtime value, got {}", self);
    let result = self.get_special_value() >> SpecialLoadtime::TAG_BITS;
    // println!("get_lt_val raw=0x{:x} result={}", self.raw(), result);
    result
  }

  #[inline]
  pub fn get_loadtime_tag(self) -> SpecialLoadtime {
    debug_assert!(self.is_loadtime(), "Must be a loadtime value, got {}", self);
    let mask = (1usize << SpecialLoadtime::TAG_BITS) - 1;
    SpecialLoadtime(self.get_special_value() & mask)
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
