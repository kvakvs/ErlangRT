//! Module defines Runtime Context which represents the low-level VM state of
//! a running process, such as registers, code pointer, etc.
use defs::Word;
use defs;
use emulator::code::{CodePtr};
use emulator::heap;
use term::lterm::LTerm;


/// Structure represents the runtime state of a VM process. It is "swapped in"
/// when the process is about to run, and "swapped out", when the process is
/// done running its time-slice.
pub struct Context {
  /// Current code location, const ptr (unsafe!).
  pub ip: CodePtr,

  /// Return location, for one return without using the stack.
  pub cp: CodePtr,

  /// Current state of X registers.
  pub regs: [LTerm; defs::MAX_XREGS],

  /// Current state of Y registers.
  pub fpregs: [defs::Float; defs::MAX_FPREGS],
}


impl Context {
  pub fn new(ip: CodePtr) -> Context {
    Context {
      cp: CodePtr::null(),
      fpregs: [0.0; defs::MAX_FPREGS],
      ip,
      regs: [LTerm::non_value(); defs::MAX_XREGS],
    }
  }


  /// Read a word from `self.ip` and advance `ip` by 1 word.
  /// NOTE: The compiler seems to be smart enough to optimize multiple fetches
  /// as multiple reads and a single increment.
  pub fn fetch(&mut self) -> Word {
    let CodePtr::Ptr(ip0) = self.ip;
    unsafe {
      let w = *ip0;
      self.ip = CodePtr::Ptr(ip0.offset(1));
      w
    }
  }


  #[inline]
  pub fn fetch_term(&mut self) -> LTerm { LTerm::from_raw(self.fetch()) }


  /// Advance `self.ip` by `n` words.
  pub fn ip_add(&mut self, n: isize) {
    let CodePtr::Ptr(ip0) = self.ip;
    self.ip = unsafe { CodePtr::Ptr(ip0.offset(n)) };
  }
}
