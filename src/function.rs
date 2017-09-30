use mfa;
use module;

use std::sync;

pub type Ptr = sync::Arc<Box<Function>>;
pub type Weak = sync::Weak<Box<Function>>;

/// Represents a function and its bytecode. Is refcounted and can be freed
/// early and separately from the module if the situation allows.
pub struct Function {
  parent_mod: module::Weak,
  name: mfa::FunArity,
}
