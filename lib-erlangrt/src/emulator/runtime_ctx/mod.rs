//! Module defines Runtime Context which represents the low-level VM state of
//! a running process, such as registers, code pointer, etc.
use core::{fmt, ptr, slice};

use colored::Colorize;

use crate::{
  beam::gen_op,
  defs::{Reductions, Word, MAX_FPREGS, MAX_XREGS},
  emulator::{
    code::{opcode, CodePtr},
    code_srv::MFALookupResult,
    heap,
    process::Process,
    runtime_ctx::current_binary::CurrentBinaryState,
    vm::VM,
  },
  fail::RtResult,
  term::value::{self, PrimaryTag, Term},
};

pub mod call_closure;
pub mod call_export;
pub mod call_native_fun;
pub mod current_binary;

fn module() -> &'static str {
  "rt_ctx: "
}

/// Structure represents the runtime state of a VM process. It is "swapped in"
/// when the process is about to run, and "swapped out", when the process is
/// done running its time-slice.
pub struct Context {
  /// Current code location, const ptr (unsafe!).
  pub ip: CodePtr,
  /// For a fetched opcode this is op arguments location
  args_ptr: *const Word,

  /// Return location, for one return without using the stack.
  pub cp: CodePtr,

  /// A metric of CPU time spent on running the code, roughly equal to 1 function call
  pub reductions: isize,

  /// Current state of X registers.
  regs: [Term; MAX_XREGS],

  /// How many X registers are currently used.
  pub live: usize,

  /// Current state of Y registers.
  pub fpregs: [f64; MAX_FPREGS],

  /// Binary building shenanigans store state here
  pub current_bin: CurrentBinaryState,
}

/// Returned from return function, it can either be performed on an empty stack
/// and lead to the process' end of life, or can be a normal return (jump to CP).
pub enum ReturnResult {
  /// Return could not find any CP data on stack, because it was empty.
  /// This means the process has to end its life (no more code).
  EmptyStack,
  /// The return was done.
  Success,
}

impl Context {
  pub fn new(ip: CodePtr) -> Context {
    Context {
      cp: CodePtr::null(),
      args_ptr: ptr::null(),
      fpregs: [0.0; MAX_FPREGS],
      ip,
      regs: [Term::non_value(); MAX_XREGS],
      live: 0,
      reductions: 0,
      current_bin: CurrentBinaryState::new(),
    }
  }

  /// Perform a return just like BEAM `return` instruction does. The result must
  /// be in X0, but that is none of this function's business.
  #[inline]
  pub fn return_and_clear_cp(&mut self, proc: &Process) -> ReturnResult {
    if cfg!(feature = "trace_calls") {
      println!("{} x0={}", "Return:".yellow(), self.get_x(0));
    }
    if self.cp.is_null() {
      if proc.heap.stack_depth() == 0 {
        // Process end of life: return on empty stack
        println!(
          "Process {} end of life (return on empty stack) x0={}",
          proc.pid,
          self.get_x(0)
        );
        return ReturnResult::EmptyStack;
      } else {
        proc.heap.stack_dump();
        panic!(
          "{}Return instruction with null CP and nonempty stack. Possible error in CP value management",
          module()
        )
      }
    }
    self.jump_ptr(self.cp.get_pointer());
    self.clear_cp();
    ReturnResult::Success
  }

  /// Read contents of an X register.
  #[inline]
  pub fn get_x(&self, index: usize) -> Term {
    // debug_assert!(index < self.live);
    let result = self.regs[index];
    debug_assert!(result.is_value(), "Should never get a #Nonvalue<> from x[]");
    result
  }

  #[inline]
  pub fn set_x(&mut self, index: usize, val: Term) {
    if cfg!(feature = "trace_register_changes") {
      println!("{}{} = {}", "set x".blue(), index, val);
    }
    // debug_assert!(val.is_value(), "Should never set x[] to a #Nonvalue<>");
    self.regs[index] = val;
  }

  #[inline]
  pub fn swap_in(&mut self) {
    // This amount is RESET every time process is about to be scheduled in, i.e.
    // there can be no "debt" of reductions, but the idea is nice.
    self.reductions = Reductions::DEFAULT;
  }

  #[inline]
  pub fn fetch_opcode(&mut self) -> opcode::RawOpcode {
    self.reductions -= Reductions::FETCH_OPCODE_COST;
    let op = opcode::from_memory_word(self.ip_read());
    self.args_ptr = unsafe { self.ip.get_pointer().add(1) };
    self.ip_advance(1isize + gen_op::opcode_arity(op) as isize);
    op
  }

  /// Read a word from `self.ip` and advance `ip` by 1 word.
  /// NOTE: The compiler seems to be smart enough to optimize multiple fetches
  /// as multiple reads and a single increment.
  #[inline]
  pub fn ip_read(&mut self) -> Word {
    unsafe {
      let w = *(self.ip.get_pointer());
      w
    }
  }

  /// Read raw word from `ip[offs]`
  #[inline]
  pub fn op_arg_read_at(&mut self, offs: usize) -> Word {
    unsafe {
      let w = *(self.args_ptr.add(offs));
      w
    }
  }

  #[inline]
  pub fn ip_advance(&mut self, offs: isize) {
    self.ip.offset(offs);
  }

  /// Fetch a word from code, assume it is an `Term`. The code position is
  /// advanced by 1.
  #[inline]
  pub fn op_arg_read_term_at(&mut self, offs: usize) -> Term {
    Term::from_raw(self.op_arg_read_at(offs))
  }

  /// Using current position in code as the starting address, create a new
  /// `&[Term]` slice of given length and advance the read pointer. This is
  /// used for fetching arrays of args from code without moving them.
  pub fn op_arg_term_slice_at(&mut self, offset: usize, sz: usize) -> &'static [Term] {
    let arg_p = self.args_ptr as *const Term;
    unsafe { slice::from_raw_parts(arg_p.add(offset), sz) }
  }

  /// Returns slice of registers `offset` to `sz`, bypassing the borrow checker
  /// It is the caller responsibility to forget the registers slice ASAP.
  pub fn registers_slice(&mut self, offset: usize, sz: usize) -> &'static [Term] {
    unsafe { slice::from_raw_parts(self.regs.as_ptr().add(offset), sz) }
  }

  /// Returns mutable slice of registers `offset` to `sz`, bypassing the borrow checker
  /// It is the caller responsibility to forget the registers slice ASAP.
  pub fn registers_slice_mut(&mut self, offset: usize, sz: usize) -> &'static mut [Term] {
    unsafe { slice::from_raw_parts_mut(self.regs.as_mut_ptr().add(offset), sz) }
  }

  /// Fetch a word from code, assume it is either an `Term` or a source X, Y or
  /// FP register, then perform a load operation.
  #[inline]
  pub fn op_arg_load_term_at(&mut self, offs: usize, hp: &heap::Heap) -> Term {
    let src = self.op_arg_read_term_at(offs);
    self.load(src, hp)
  }

  //  /// Advance `self.ip` by `n` words.
  //  pub fn ip_add(&mut self, n: isize) {
  //    let CodePtr::Ptr(ip0) = self.ip;
  //    self.ip = unsafe { CodePtr::Ptr(ip0.offset(n)) };
  //  }

  /// Read a register otherwise term is returned unchanged.
  // TODO: Optimize - separate load constant from load register instruction
  pub fn load(&self, src: Term, hp: &heap::Heap) -> Term {
    if src.is_special() {
      if src.get_special_tag() == value::SpecialTag::REG {
        let r_tag = src.get_reg_tag();
        if r_tag == value::SpecialReg::REG_X {
          return self.get_x(src.get_reg_value());
        } else if r_tag == value::SpecialReg::REG_Y {
          let y_index = src.get_reg_value();
          let y_result = hp.get_y(y_index);
          return y_result.unwrap();
        } else if r_tag == value::SpecialReg::REG_FLOAT {
          panic!("todo fpreg load")
        } else {
          panic!("special tag not supported")
        }
      }
    }
    // Otherwise return unchanged
    src
  }

  /// Copy a value from `src` (possibly a stack cell or a register) to `dst`.
  /// Returns void `()` or an error.
  #[allow(dead_code)]
  #[inline]
  pub fn load_then_store(
    &mut self,
    src: Term,
    dst: Term,
    hp: &mut heap::Heap,
  ) -> RtResult<()> {
    let src_val = self.load(src, hp);
    self.store_value(src_val, dst, hp)
  }

  /// Copy a value `val` to `dst`. No attempt is done to load val from a
  /// stack value or a register, val is assumed to be a ready value, not a source.
  /// Returns void `()` or an error.
  pub fn store_value(
    &mut self,
    val: Term,
    dst: Term,
    hp: &mut heap::Heap,
  ) -> RtResult<()> {
    debug_assert!(
      !val.is_register_x(),
      "ctx.store value must not be a X reg, have {}",
      val
    );
    debug_assert!(
      !val.is_register_y(),
      "ctx.store value must not be a Y reg, have {}",
      val
    );
    debug_assert!(
      !val.is_register_float(),
      "ctx.store value must not be a FP reg, have {}",
      val
    );
    debug_assert!(
      dst.is_register_x() || dst.is_register_y() || dst.is_register_float(),
      "ctx.store destination must be a X, Y or FP register"
    );
    if dst.get_term_tag() == PrimaryTag::SPECIAL {
      if dst.get_special_tag() == value::SpecialTag::REG {
        let r_tag = dst.get_reg_tag();
        if r_tag == value::SpecialReg::REG_X {
          self.set_x(dst.get_reg_value(), val);
          return Ok(());
        } else if r_tag == value::SpecialReg::REG_Y {
          let y = dst.get_reg_value();
          return hp.set_y(y, val);
        } else if r_tag == value::SpecialReg::REG_FLOAT {
          panic!("todo fpreg store");
        } else {
          panic!("store: specialtag {:?} not supported", r_tag);
        }
      }
    }
    panic!(
      "{}Don't know how to ctx.store {} into {}",
      module(),
      val,
      dst
    )
  }

  #[inline]
  pub fn set_cp(&mut self, cp: Term) {
    debug_assert!(cp.is_cp());
    self.cp = CodePtr::from_cp(cp);
  }

  #[inline]
  pub fn clear_cp(&mut self) {
    self.cp = CodePtr::null();
  }

  #[inline]
  pub fn jump(&mut self, cp: Term) {
    debug_assert!(cp.is_cp());
    println!("{} {:p}", "jump to".purple(), cp.get_cp_ptr::<Word>());
    self.ip = CodePtr::from_cp(cp);
  }

  #[inline]
  pub fn jump_ptr(&mut self, code_ptr: *const Word) {
    debug_assert!(!code_ptr.is_null(), "Jumping to NULL is a bad idea");
    if cfg!(feature = "trace_opcode_execution") {
      println!("{} {:p}", "jump to".purple(), code_ptr);
    }
    self.ip = CodePtr::from_ptr(code_ptr);
  }

  /// Perform a call to lookup result (done by CodeServer).
  /// Optional `save_cp` defines whether CP will be saved
  pub fn call_mfa(
    &mut self,
    vm: &mut VM,
    curr_p: &mut Process,
    lr: &MFALookupResult,
    args: &[Term],
    save_cp: bool,
  ) -> RtResult<()> {
    match lr {
      MFALookupResult::FoundBeamCode(code_p) => {
        if save_cp {
          self.cp = self.ip;
        }
        self.ip = code_p.clone();
      }
      MFALookupResult::FoundBif(bif_fn) => {
        let x0 = call_native_fun::call_native_fun_fn(vm, self, curr_p, *bif_fn, args)?;
        self.set_x(0, x0);
      }
    }
    Ok(())
  }

  #[allow(dead_code)]
  pub fn dump_registers(&self, arity: usize) {
    if arity == 0 {
      println!("registers: empty");
      return;
    }

    for i in 0..arity {
      println!("reg X[{}] = {}", i, self.get_x(i));
    }
  }

  #[inline]
  pub fn debug_trace_call(
    &self,
    description: &str,
    _dst: Term,
    offset: usize,
    arity: usize,
  ) {
    if cfg!(feature = "trace_calls") {
      print!("{} <{}>, x[..{}] <", "Call".yellow(), description, arity);
      for i in offset..(offset + arity) {
        if i > 0 {
          print!("{}", "; ".red());
        }
        print!("{}", self.get_x(i));
      }
      println!(">");
    }
  }
}

// === === === ===
//

impl fmt::Display for Context {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut str_regs = String::new();
    for n in 0..self.live {
      let xn = format!("x{}", n).black().on_white();
      str_regs += &format!("{}={}; ", xn, self.get_x(n))
    }

    writeln!(
      f,
      "{}\nip: {:?}, cp: {:?}\nregs[..10]: {}",
      "Emulator state:".white().on_red(),
      self.ip,
      self.cp,
      str_regs
    )
  }
}
