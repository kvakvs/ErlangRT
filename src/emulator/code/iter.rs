//! Module defines `CodeIterator` which can step over the code
use term::primary;
use emulator::code::pointer::CodePtr;
use emulator::code::opcode;
use beam::gen_op;


// This is used by code walkers such as "disasm.rs" or code loader postprocess
#[allow(dead_code)]
pub struct CodeIterator {
  p: CodePtr,
  end: CodePtr,
}


impl CodeIterator {
  pub fn new(begin: CodePtr, end: CodePtr) -> CodeIterator {
    CodeIterator { p: begin, end }
  }


//  /// Read current value at the iterator location.
//  pub unsafe fn read_term(&self) -> LTerm {
//    let DataPtr::Ptr(p) = self.p;
//    LTerm::from_raw(*p)
//  }
}


impl Iterator for CodeIterator {
  type Item = CodePtr;


  /// Given an iterator (`self`) step forward over its args to find another.
  fn next(&mut self) -> Option<Self::Item> {
    let CodePtr::Ptr(p) = self.p;

    let current_op = unsafe { opcode::from_memory_word(*p) };
    let arity = gen_op::opcode_arity(current_op);

    // Step forward over opcode and `arity` words (args)
    let next_p = unsafe { DataPtr::Ptr(p.offset(arity + 1)) };

    if next_p >= self.end {
      return None
    }

    self.p = next_p;
    Some(self.p)
  }
}
