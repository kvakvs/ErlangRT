//! Module defines types to represent code structures.
pub mod iter;
pub mod opcode;
pub mod pointer;

use crate::defs::Word;
use std::collections::BTreeMap;

pub use crate::emulator::code::{opcode::*, pointer::*};


/// Code array stores opcodes/jump table offsets and args encoded as `LTerm`
// TODO: Make code vec of LTerm?
pub type Code = Vec<Word>;


/// A slice to a code array
pub type RefCode<'a> = &'a [Word];


/// Tagged word for label index
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
pub struct LabelId(pub Word);


/// Tagged word for offset in the code array
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
pub struct CodeOffset(pub Word);


/// Map of label id to offset. Maybe: Use binary search sorted array?
pub type Labels = BTreeMap<LabelId, CodeOffset>;
