use std::cell::RefCell;
use std::sync;

use beam::gen_op;
use defs::Word;
use emulator::funarity::FunArity;
use emulator::module;
use term::lterm::LTerm;

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
  /// Create an empty function wrapped in atomic refcounted refcell.
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
      assert!(op < gen_op::OPCODE_MAX);
      print!("{:04x} {} ", i, gen_op::opcode_name(op as u8));
      i += 1;

      let arity = gen_op::opcode_arity(op as u8) as Word;
      for j in 0..arity {
        let arg_raw = self.code[i + j];
        let arg = LTerm::from_raw(arg_raw);

        // Header value in code marks an embedded block of terms
        // Header{Arity=3} Term1 Term2 Term3
        if arg.is_header() {
          print!("Table{{");
          for _h in 0..arg.header_arity() {
            print!("{} ", LTerm::from_raw(self.code[i + j]));
            i += 1;
          }
          print!("}} ");
        } else { // Otherwise it is printable like this, and occupies 1w
          print!("{} ", arg)
        }
      }

      i += arity;
      println!();
    }
  }
}

pub fn make_weak(p: &Ptr) -> Weak {
  sync::Arc::downgrade(p)
}
