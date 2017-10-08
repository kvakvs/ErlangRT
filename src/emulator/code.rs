//! Module defines types to represent code structures.
use defs::Word;
use term::lterm::LTerm;

use std::collections::BTreeMap;


/// Code array stores opcodes/jump table offsets and args encoded as `LTerm`
pub type Code = Vec<Word>;

/// Tagged word for label index
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub enum LabelId { Val(Word) }

/// Tagged word for offset in the code array
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub enum CodeOffset { Val(Word) }

/// Map of label id to offset. Maybe: Use binary search sorted array?
pub type Labels = BTreeMap<LabelId, CodeOffset>;

pub enum CodePtr { Ptr(*const Word) }
pub enum CodePtrMut { Ptr(*mut Word) }

// / Universal pointer to module and offset in the module code.
// TODO: Make this *const Word wrapped in Enum
//pub struct InstrPointer {
//  mod_name: LTerm,
//  ip: CodeOffset,
//}

//impl InstrPointer {
//  pub fn new(mod_name: LTerm, ip: CodeOffset) -> InstrPointer {
//    InstrPointer { mod_name, ip }
//  }
//}
