use defs::Word;
use emulator::funarity::FunArity;
use emulator::gen_op;
use emulator::module;
use term::lterm::LTerm;

use std::sync;
use std::cell::RefCell;

pub type Ptr = sync::Arc<RefCell<Function>>;
pub type Weak = sync::Weak<RefCell<Function>>;

/// Represents a function and its bytecode. Is refcounted and can be freed
/// early and separately from the module if the situation allows.
pub struct Function {
  pub parent_mod: module::Weak,
  pub funarity: FunArity,
  pub code: Vec<Word>,
}

impl Function {
  pub fn new() -> Ptr {
    sync::Arc::new(RefCell::new(
      Function {
        parent_mod: sync::Weak::new(),
        funarity: FunArity::new(),
        code: Vec::new(),
      }
    ))
  }

  /// Print to screen disassembly of the current function
  #[cfg(feature="dev_build")]
  pub fn disasm(&self) {
    let mut i = 0;
    while i < self.code.len() {
      let op = self.code[i];
      assert!(op < 256);
      print!("{} ", gen_op::opcode_name(op as u8));
      i += 1;

      let arity = gen_op::opcode_arity(op as u8) as Word;
      for j in 0..arity {
        print!("{} ", LTerm::from_raw(self.code[i + j]))
      }

      i += arity;
      println!();
    }
  }
}

pub fn make_weak(p: &Ptr) -> Weak {
  sync::Arc::downgrade(p)
}
