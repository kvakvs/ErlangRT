use term::primary_tag;
use types::Word;

// .... .... ..22 11PP
// Here PP are the primary tag, always primary_tag::Tag::Immediate
// And "11" with size 2 bits, uses Immediate1 bits.
// To use Immediate2 bits set "11" to Immediate1::Immed2 and set 22 to the desired value
pub const IMM1_SIZE: Word = 2;

pub enum Immediate1 {
  Pid = 0,
  Port = 1,
  Small = 2,
  Immed2 = 3,
}

pub enum Immediate2 {
  Atom = 0,
  Catch = 1,
  Nil = 2,
  // not used
  Immed3 = 3,
}

pub const IMM2_SIZE: Word = 2;
pub const IMM2_VAL_SHIFT: Word = primary_tag::SIZE + IMM1_SIZE + IMM2_SIZE;

pub fn atom(val: Word) -> Word {
  let val1 = val << IMM2_VAL_SHIFT;
  let val2= primary_tag::set_primary_immediate(val1);
  set_immediate2_tag(val2, Immediate2::Atom)
}

// Overlay immediate1 tag bits without modifying the rest of the word
pub fn set_immediate1_tag(val: Word, imm1: Immediate1) -> Word {
  val | ((imm1 as Word) << primary_tag::SIZE)
}

// Overlay immediate2 tag bits without modifying the rest of the word
pub fn set_immediate2_tag(val: Word, imm2: Immediate2) -> Word {
  let val2 = set_immediate1_tag(val, Immediate1::Immed2);
  val2 | ((imm2 as Word) << (IMM1_SIZE + primary_tag::SIZE))
}
