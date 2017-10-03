use emulator::mfa;
use emulator::module;
use beam::instruction::Instr;

use std::sync;

pub type Ptr = sync::Arc<Function>;
pub type Weak = sync::Weak<Function>;

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
    sync::Arc::new(Function {
      parent_mod: sync::Weak::new(),
      name: mfa::FunArity::new(),
      code: Vec::new(),
    })
  }
}

pub fn make_weak(p: &Ptr) -> Weak {
  sync::Arc::downgrade(p)
}
