//! Module defines Runtime Context which represents the low-level VM state of
//! a running process, such as registers, code pointer, etc.
use defs::Word;
use defs;
use emulator::code::{CodePtr};
use emulator::heap;
use term::lterm::LTerm;
use term::immediate;

use std::fmt;


fn module() -> &'static str { "runtime_ctx: " }


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


//  /// Advance `self.ip` by `n` words.
//  pub fn ip_add(&mut self, n: isize) {
//    let CodePtr::Ptr(ip0) = self.ip;
//    self.ip = unsafe { CodePtr::Ptr(ip0.offset(n)) };
//  }


  /// Read a register otherwise term is returned unchanged.
  // TODO: Optimize - separate load constant from load register instruction
  pub fn load(&self, src: LTerm, hp: &heap::Heap) -> LTerm {
    if src.is_immediate3() {
      let v = src.value;
      match immediate::get_imm3_tag(v) {
        immediate::TAG_IMM3_XREG => return self.regs[immediate::get_imm3_value(v)],
        immediate::TAG_IMM3_YREG => {
          let y_index = immediate::get_imm3_value(v);
          let y_result = hp.stack_get_y(y_index);
          return y_result.unwrap()
        },
        _ => panic!("{}Don't know how to ctx.load from {}", module(), src)
      }
    }
    // otherwise return unchanged
    src
  }


  /// Copy a value from `src` (possibly a stack cell or a register) to `dst`.
  pub fn store(&mut self, src: LTerm, dst: LTerm, hp: &mut heap::Heap) {
    let src_val = self.load(src, hp);
    if dst.is_immediate3() {
      let v = dst.value;
      match immediate::get_imm3_tag(v) {
        immediate::TAG_IMM3_XREG => {
          self.regs[immediate::get_imm3_value(v)] = src_val;
          return
        },
        immediate::TAG_IMM3_YREG => {
          let y = immediate::get_imm3_value(v);
          let y_result = hp.stack_set_y(y, src_val);
          return y_result.unwrap()
        },
        _ => {}
      }
    }
    panic!("{}Don't know how to ctx.store {} to {}", module(), src, dst)
  }

}


impl fmt::Display for Context {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut str_regs = String::new();
    for v in self.regs[0..10].iter() {
      str_regs += &format!("{}; ", v)
    }

    writeln!(f, "ip: {:?}, cp: {:?}\nregs[..10]: {}", self.ip, self.cp, str_regs)
  }
}