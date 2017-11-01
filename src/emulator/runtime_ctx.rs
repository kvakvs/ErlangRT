//! Module defines Runtime Context which represents the low-level VM state of
//! a running process, such as registers, code pointer, etc.

use bif::BifFn;
use defs::{Word, Float, DispatchResult, MAX_XREGS, MAX_FPREGS};
use emulator::code::CodePtr;
use emulator::heap;
use emulator::process::Process;
use fail::Hopefully;
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

//
// Call Bif generic facilities
//

/// Generic bif0,1,2 application. Bif0 cannot have a fail label but bif1 and
/// bif2 can, so on exception a jump will be performed.
///
/// Args: `curr_p` - the process which is running;
/// `bif_fn` - the function to apply (the callable Rust fn), possibly an error;
/// `fail_label` - if not NIL, we suppress a possible exception and jump there;
/// `args` - the arguments; `dst` - register where the result will go;
/// `gc` if true, then gc is allowed and `ctx.live` will be used.
#[inline]
// Inline to allow const folding optimization
pub fn call_bif(ctx: &mut Context,
                curr_p: &mut Process,
                bif_fn: Hopefully<BifFn>,
                fail_label: LTerm,
                n_args: Word,
                dst: LTerm,
                gc: bool) -> DispatchResult
{
  let bif_callable = bif_fn.unwrap();
  let bif_result = {
    let args = &ctx.regs[0..n_args];
    (bif_callable)(curr_p, args)
  };
  match bif_result {
    // Check error
    t if t.is_non_value() => {
      // On error and if fail label is a CP, perform a go to
      if fail_label.is_cp() {
        ctx.ip = CodePtr::from_cp(fail_label)
      }
    },
    val => {
      // if dst is not NIL, store the result in it
      if !dst.is_nil() {
        ctx.store(val, dst, &mut curr_p.heap)
      }
    },
  }
  DispatchResult::Normal
}
