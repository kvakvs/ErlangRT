use emulator::mfa;
use emulator::module;
use beam::instruction::Instr;

use std::sync;
use std::cell::RefCell;

pub type Ptr = sync::Arc<RefCell<Function>>;
pub type Weak = sync::Weak<RefCell<Function>>;

/// Represents a function and its bytecode. Is refcounted and can be freed
/// early and separately from the module if the situation allows.
pub struct Function {
  parent_mod: module::Weak,
  name: mfa::FunArity,
  // TODO: Use Word array and encode everything to Word/low_level::Term
  pub code: Vec<Instr>,
}

impl Function {
  pub fn new() -> Ptr {
    sync::Arc::new(RefCell::new(
      Function {
        parent_mod: sync::Weak::new(),
        name: mfa::FunArity::new(),
        code: Vec::new(),
      }
    ))
  }
}

pub fn make_weak(p: &Ptr) -> Weak {
  sync::Arc::downgrade(p)
}
