//! Module defines types to represent code structures.
pub mod opcode;
pub mod pointer;
pub mod iter;

use defs::Word;

use std::collections::BTreeMap;

pub use emulator::code::opcode::*;
pub use emulator::code::pointer::*;

/// Code array stores opcodes/jump table offsets and args encoded as `LTerm`
pub type Code = Vec<Word>;

/// A slice to a code array
pub type RefCode<'a> = &'a [Word];

/// Tagged word for label index
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub enum LabelId { Val(Word) }

/// Tagged word for offset in the code array
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub enum CodeOffset { Val(Word) }

/// Map of label id to offset. Maybe: Use binary search sorted array?
pub type Labels = BTreeMap<LabelId, CodeOffset>;
