//! Module defines Runtime Context which represents the low-level VM state of
//! a running process, such as registers, code pointer, etc.

pub mod call;

use emulator::code::CodePtr;
use emulator::heap;
use rt_defs::stack::IStack;
use rt_defs::{Word, Float, MAX_XREGS, MAX_FPREGS};
use term::immediate;
use term::lterm::{LTerm};

use std::fmt;
use std::slice;


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
  pub regs: [LTerm; MAX_XREGS],
  /// How many X registers are currently used.
  pub live: Word,

  /// Current state of Y registers.
  pub fpregs: [Float; MAX_FPREGS],
}


impl Context {
  /// For swapping in/out with a process, copy pointers and `live` amount of X
  /// registers.
  pub fn copy_from(&mut self, other: &Context) {
    self.ip = other.ip;
    self.cp = other.cp;

    let live = other.live;
    self.live = live;
    self.regs[0..live].clone_from_slice(&other.regs[0..live]);

    self.fpregs = other.fpregs;
  }


  pub fn new(ip: CodePtr) -> Context {
    Context {
      cp: CodePtr::null(),
      fpregs: [0.0; MAX_FPREGS],
      ip,
      regs: [LTerm::non_value(); MAX_XREGS],
      live: 0,
    }
  }


  /// Read a word from `self.ip` and advance `ip` by 1 word.
  /// NOTE: The compiler seems to be smart enough to optimize multiple fetches
  /// as multiple reads and a single increment.
  pub fn fetch(&mut self) -> Word {
    let CodePtr(ip0) = self.ip;
    unsafe {
      let w = *ip0;
      self.ip = CodePtr(ip0.offset(1));
      w
    }
  }


  /// Fetch a word from code, assume it is an `LTerm`. The code position is
  /// advanced by 1.
  #[inline]
  pub fn fetch_term(&mut self) -> LTerm { LTerm::from_raw(self.fetch()) }


  /// Using current position in code as the starting address, create a new
  /// `&[LTerm]` slice of given length and advance the read pointer. This is
  /// used for fetching arrays of args from code without moving them.
  pub fn fetch_slice(&mut self, sz: usize) -> &'static [LTerm] {
    let CodePtr(ip0) = self.ip;
    unsafe {
      self.ip = CodePtr(ip0.offset(sz as isize));
      slice::from_raw_parts(ip0 as *const LTerm, sz)
    }
  }


  pub fn registers_slice(&mut self, sz: usize) -> &'static [LTerm] {
    unsafe {
      slice::from_raw_parts(self.regs.as_ptr(), sz)
    }
  }


  /// Fetch a word from code, assume it is either an `LTerm` or a source X, Y or
  /// FP register, then perform a load operation.
  #[inline]
  pub fn fetch_and_load(&mut self, hp: &heap::Heap) -> LTerm {
    let src = LTerm::from_raw(self.fetch());
    self.load(src, hp)
  }


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
          return y_result.unwrap();
        }
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
          return;
        }
        immediate::TAG_IMM3_YREG => {
          let y = immediate::get_imm3_value(v);
          let y_result = hp.stack_set_y(y, src_val);
          return y_result.unwrap();
        }
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

    writeln!(f, concat!(
        "Emulator state:\n",
        "ip: {:?}, cp: {:?}\nregs[..10]: {}"
      ), self.ip, self.cp, str_regs)
  }
}
