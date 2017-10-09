//! Module defines types to represent code structures.
use defs::Word;
//use term::lterm::LTerm;

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

impl CodePtr {
  pub fn null() -> CodePtr {
    CodePtr::Ptr(0 as *const Word)
  }

//  pub fn is_null(&self) -> bool {
//    let CodePtr::Ptr(p) = *self;
//    p == 0 as *const Word
//  }
}

//pub enum CodePtrMut { Ptr(*mut Word) }
