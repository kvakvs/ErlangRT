use term::primary_tag;
use types::Word;

// .... .... ..22 11PP
// Here PP are the primary tag, always primary_tag::Tag::Immediate
// And "11" with size 2 bits, uses Immediate1 bits.
// To use Immediate2 bits set "11" to Immediate1::Immed2 and set 22 to the desired value
pub const IMM1_SIZE: Word = 2;
pub const IMM1_VALUE_SHIFT: Word = primary_tag::SIZE + IMM1_SIZE;
pub const IMM1_MASK: Word = (1 << IMM1_VALUE_SHIFT) - 1;

pub enum Immediate1 {
  Pid = 0,
  Port = 1,
  Small = 2,
  Immed2 = 3,
}

pub enum Immediate2 {
  Atom = 0,
  Catch = 1,
  // Special includes NIL, NONVALUE etc
  Special = 2,
  // not used
  Immed3 = 3,
}

pub enum SpecialImm2 {
  Nil = 1,
  NonValue = 2,
}

pub const IMM2_SIZE: Word = 2;
pub const IMM2_VALUE_SHIFT: Word = primary_tag::SIZE + IMM1_SIZE + IMM2_SIZE;
pub const IMM2_MASK: Word = (1 << IMM2_VALUE_SHIFT) - 1;

//
// Premade bit combinations for some constants
//

// Special Primary tag+Immed1 precomposed
const RAW_IMMED1: Word = primary_tag::Tag::Immediate as Word;
const RAW_IMMED2: Word = (primary_tag::Tag::Immediate as Word)
    | ((Immediate1::Immed2 as Word) << primary_tag::SIZE);

// Precomposed bits for pid imm1
pub const RAW_PID: Word = RAW_IMMED1
    | ((Immediate1::Pid as Word) << primary_tag::SIZE);

// Precomposed bits for atom imm2
pub const RAW_ATOM: Word = RAW_IMMED2
    | ((Immediate2::Atom as Word) << IMM1_VALUE_SHIFT);

// Special Primary tag+Immed1+Immed2 bits precomposed
const RAW_SPECIAL: Word = (primary_tag::Tag::Immediate as Word)
    | ((Immediate1::Immed2 as Word) << primary_tag::SIZE)
    | ((Immediate2::Special as Word) << IMM1_VALUE_SHIFT);

// Precomposed bits for NIL constant
pub const RAW_NIL: Word = RAW_SPECIAL
    | ((SpecialImm2::Nil as Word) << IMM2_VALUE_SHIFT);

// Precomposed bits for NON_VALUE constant
pub const RAW_NON_VALUE: Word = RAW_SPECIAL
    | ((SpecialImm2::NonValue as Word) << IMM2_VALUE_SHIFT);

// Given value (to be shifted) and RAW_* preset bits, compose them together for imm2
fn create_imm2(val: Word, raw_preset: Word) -> Word {
  (val << IMM2_VALUE_SHIFT) | raw_preset
}

// Given a value (to be shifted) and RAW_* preset bits, compose them together for imm1
fn create_imm1(val: Word, raw_preset: Word) -> Word {
  (val << IMM1_VALUE_SHIFT) | raw_preset
}

//
// Construction
//

// Create a raw value for a term from atom index
pub fn make_atom_raw(val: Word) -> Word {
  create_imm2(val, RAW_ATOM)
}

// Create a raw value for a pid term from process index
pub fn make_pid_raw(pindex: Word) -> Word {
  create_imm1(pindex, RAW_PID)
}
