use emulator::mfa;
use emulator::module;

use std::sync;

pub type Ptr = sync::Arc<Function>;
pub type Weak = sync::Weak<Function>;

/// Represents a function and its bytecode. Is refcounted and can be freed
/// early and separately from the module if the situation allows.
pub struct Function {
  parent_mod: module::Weak,
  name: mfa::FunArity,
}
