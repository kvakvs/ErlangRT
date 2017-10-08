//! Module defines Runtime Context which represents the low-level VM state of
//! a running process, such as registers, code pointer, etc.
use term::lterm::LTerm;
use defs::Word;
use defs;
use emulator::code::{CodePtr};


/// Structure represents the runtime state of a VM process. It is "swapped in"
/// when the process is about to run, and "swapped out", when the process is
/// done running its time-slice.
pub struct Context {
  /// Current code location, const ptr (unsafe!)
  pub ip: CodePtr,
  /// Current state of X registers
  pub regs: [LTerm; defs::MAX_XREGS],
  /// Current state of Y registers
  pub fpregs: [defs::Float; defs::MAX_FPREGS],
  // TODO: Stack
}


impl Context {
  pub fn new(ip: CodePtr) -> Context {
    Context {
      ip,
      regs: [LTerm::non_value(); defs::MAX_XREGS],
      fpregs: [0.0; defs::MAX_FPREGS],
    }
  }

  pub fn fetch(&mut self) -> Word {
    let CodePtr::Ptr(ip0) = self.ip;
    unsafe {
      let w = *ip0;
      self.ip = CodePtr::Ptr(ip0.offset(1));
      w
    }
  }
}
