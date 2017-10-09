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


#[cfg(target_pointer_width = "32")]
const TAG_CP: Word = 1usize << 31;

#[cfg(target_pointer_width = "64")]
const TAG_CP: Word = 1usize << 63;


#[derive(Copy, Clone)]
pub enum CodePtr { Ptr(*const Word) }

impl CodePtr {
  pub fn null() -> CodePtr {
    CodePtr::Ptr(0 as *const Word)
  }


  /// Convert to tagged CP integer
  pub fn to_cp(&self) -> Word {
    let CodePtr::Ptr(p) = *self;
    let p1 = p as Word;
    p1 | TAG_CP
  }


  pub fn offset(&self, n: isize) -> CodePtr {
    let CodePtr::Ptr(p) = *self;
    let new_p = unsafe { p.offset(n) };
    CodePtr::Ptr(new_p)
  }

//  pub fn is_null(&self) -> bool {
//    let CodePtr::Ptr(p) = *self;
//    p == 0 as *const Word
//  }
}

//pub enum CodePtrMut { Ptr(*mut Word) }
