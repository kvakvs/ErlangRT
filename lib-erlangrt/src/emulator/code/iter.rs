//! Module defines `CodeIterator` which can step over the code
use crate::{
  beam::gen_op,
  defs::Word,
  emulator::code::{
    opcode,
    pointer::{CodePtr, CodePtrMut},
  },
};

// This is used by read-only code walkers such as "disasm.rs".
#[allow(dead_code)]
pub struct CodeIterator {
  p: CodePtr,
  end: CodePtr,
}

// This is used by code walkers which modify the code values, such as code
// loader postprocess algorithms.
#[allow(dead_code)]
pub struct CodeIteratorMut {
  p: CodePtrMut,
  end: CodePtrMut,
}

impl CodeIteratorMut {
  pub fn new(begin: CodePtrMut, end: CodePtrMut) -> CodeIteratorMut {
    CodeIteratorMut { p: begin, end }
  }
}

impl CodeIterator {
  //  pub fn new(begin: CodePtr, end: CodePtr) -> CodeIterator {
  //    CodeIterator { p: begin, end }
  //  }

  //  /// Read current value at the iterator location.
  //  pub unsafe fn read_term(&self) -> Term {
  //    let DataPtr::Ptr(p) = self.p;
  //    Term::from_raw(*p)
  //  }
}

impl Iterator for CodeIterator {
  type Item = CodePtr;

  /// Given an iterator (`self`) step forward over its args to find another.
  fn next(&mut self) -> Option<Self::Item> {
    let p = self.p.get_pointer();

    let current_op = unsafe { opcode::from_memory_word(*p) };
    let arity = gen_op::opcode_arity(current_op);

    // Step forward over opcode and `arity` words (args)
    let next_p = unsafe { CodePtr::from_ptr(p.add(arity as usize + 1)) };

    if next_p >= self.end {
      return None;
    }

    self.p = next_p;
    Some(self.p)
  }
}

///// Create an iterator for readonly walking the code.
// pub unsafe fn create(code: &[Word]) -> CodeIterator {
//  let begin = &code[0] as *const Word;
//  let last = begin.offset(code.len() as isize);
//  CodeIterator::new(CodePtr::Ptr(begin),
//                    CodePtr::Ptr(last))
//}

impl Iterator for CodeIteratorMut {
  type Item = CodePtrMut;

  /// Given an iterator (`self`) step forward over its args to find another.
  fn next(&mut self) -> Option<Self::Item> {
    let CodePtrMut(p) = self.p;

    let current_op = opcode::from_memory_ptr(p);
    let arity = gen_op::opcode_arity(current_op);

    // Step forward over opcode and `arity` words (args)
    let next_p = unsafe {
      let p_plus_arity = p.offset(arity as isize + 1);
      CodePtrMut(p_plus_arity)
    };

    if next_p >= self.end {
      return None;
    }

    self.p = next_p;
    Some(self.p)
  }
}

/// Create am iterator for walking and modifying the code.
pub unsafe fn create_mut(code: &mut Vec<Word>) -> CodeIteratorMut {
  let begin = &mut code[0] as *mut Word;
  let end = begin.add(code.len());
  CodeIteratorMut::new(CodePtrMut(begin), CodePtrMut(end))
}
