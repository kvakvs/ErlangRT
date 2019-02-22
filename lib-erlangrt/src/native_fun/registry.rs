use crate::{defs::Arity, term::lterm::LTerm};
use std::collections::HashMap;

/// Registry stores a tree of loaded native modules.
/// Native modules may overlap with Erlang modules, while providing implementations
/// for selected functions.
pub struct NativeFunRegistry {
  modules: HashMap<LTerm, NativeModule>,
}

/// A loaded native module contains a dictionary of functions with arity
pub struct NativeModule {
  pub functions: HashMap<LTerm, NativeFun>,
}

pub struct NativeFun {
  pub name: LTerm,
  pub arity: Arity,
  pub func: super::BifFn,
}
