//! `module` module handles Erlang modules as collections of functions,
//! literals and attributes.
use std::sync;
use std::collections::BTreeMap;

use function;
use mfa;
use term::Term;

pub type Ptr = sync::Arc<Box<Module>>;
pub type Weak = sync::Weak<Box<Module>>;

/// Represents a module with collection of functions. Modules are refcounted
/// and can be freed early if the situation allows.
pub struct Module {
  name: Term,
  /// Map to refcounted functions
  code: BTreeMap<mfa::FunArity, function::Ptr>,
  // TODO: attrs
  // TODO: lit table
}
