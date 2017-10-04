//! `module` module handles Erlang modules as collections of functions,
//! literals and attributes.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync;

use emulator::function;
use emulator::funarity::FunArity;
use term::lterm::LTerm;

pub type Ptr = sync::Arc<RefCell<Module>>;
pub type Weak = sync::Weak<RefCell<Module>>;

/// Represents a module with collection of functions. Modules are refcounted
/// and can be freed early if the situation allows.
pub struct Module {
  name: LTerm,
  /// Map to refcounted functions
  pub funs: BTreeMap<FunArity, function::Ptr>,
  // TODO: attrs
  // TODO: lit table
}

impl Module {
  /// Create an empty module wrapped in atomic refcounted refcell.
  pub fn new(name: LTerm) -> Ptr {
    sync::Arc::new(RefCell::new(
      Module{
        name,
        funs: BTreeMap::new(),
      }
    ))
  }

  pub fn name(&self) -> LTerm { self.name }
}
