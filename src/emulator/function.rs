use emulator::funarity::FunArity;
use emulator::module;
use defs::Word;

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
}

pub fn make_weak(p: &Ptr) -> Weak {
  sync::Arc::downgrade(p)
}
