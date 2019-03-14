use crate::term::value::tag_term::TERM_TAG_BITS;

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

pub const SPECIAL_REG_TAG_BITS: usize = 2;
/// How many bits are remaining in the machine word after taking away the prefix bits
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
