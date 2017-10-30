//! Module defines Runtime Context which represents the low-level VM state of
//! a running process, such as registers, code pointer, etc.

use bif::BifFn;
use defs::{Word, Float, DispatchResult, MAX_XREGS, MAX_FPREGS};
use emulator::code::{CodePtr};
use emulator::heap;
use emulator::process::Process;
use term::immediate;
use term::lterm::LTerm;

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
  pub regs: [LTerm; MAX_XREGS],

  /// Current state of Y registers.
  pub fpregs: [Float; MAX_FPREGS],
}


impl Context {

//  /// For swapping out of a process, copy pointers and `live` amount of X
//  /// registers.
//  pub fn clone_ctx(&self, live: Word) -> Context {
//    let mut c = Context {
//      ip: self.ip,
//      cp: self.cp,
//      regs: [LTerm::nil(); MAX_XREGS],
//      fpregs: self.fpregs,
//    };
//    c.regs[0..live].clone_from_slice(&self.regs[0..live]);
//    c
//  }


  pub fn new(ip: CodePtr) -> Context {
    Context {
      cp: CodePtr::null(),
      fpregs: [0.0; MAX_FPREGS],
      ip,
      regs: [LTerm::non_value(); MAX_XREGS],
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


  /// Fetch a word from code, assume it is an LTerm.
  #[inline]
  pub fn fetch_term(&mut self) -> LTerm { LTerm::from_raw(self.fetch()) }


  /// Fetch a word from code, assume it is either an LTerm or a source X, Y or
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


  /// Generic bif0,1,2 application. Bif0 cannot have a fail label but bif1 and
  /// bif2 can, so on exception a jump will be performed.
  pub fn call_bif<T>(&self, curr_p: &mut Process, bif_fn: BifFn,
                     fail_label: LTerm, args: *const Word,
                     dst: LTerm) -> DispatchResult
  {
    (bif_fn)(curr_p, args);
    DispatchResult::Normal
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