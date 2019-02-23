use crate::{
  emulator::funarity::FunArity,
  native_fun::{fn_entry::NativeFnEntry, NativeFn},
  term::lterm::LTerm,
};
use std::collections::hash_map::HashMap;

/// A loaded native module contains a dictionary of functions with arity
pub struct NativeModule {
  pub name: LTerm,
  pub functions: HashMap<FunArity, NativeFn>,
}

impl NativeModule {
  pub fn new(name: LTerm) -> Self {
    Self {
      name,
      functions: HashMap::new(),
    }
  }

  pub fn init_with<'x, T>(&'x mut self, iter: T)
  where
    T: Iterator<Item = &'x NativeFnEntry>,
  {
    for entry in iter {
      self.functions.insert(entry.fa.clone(), entry.func);
    }
  }
}
