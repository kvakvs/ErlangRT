use crate::{
  emulator::funarity::FunArity,
  native_fun::{fn_entry::NativeFnEntry, NativeFn},
  term::Term,
};
use std::collections::hash_map::HashMap;

/// A loaded native module contains a dictionary of functions with arity
pub struct NativeModule {
  pub name: Term,
  pub functions: HashMap<FunArity, NativeFn>,
}

impl NativeModule {
  pub fn new(name: Term) -> Self {
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
