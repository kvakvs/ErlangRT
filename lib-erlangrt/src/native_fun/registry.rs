use crate::{
  emulator::{atom, mfa::MFArity},
  native_fun::{self, module::NativeModule, NativeFn},
  term::lterm::LTerm,
};
use std::collections::HashMap;

/// Registry stores a tree of loaded native modules.
/// Native modules may overlap with Erlang modules, while providing implementations
/// for selected functions.
pub struct NativeFunRegistry {
  modules: HashMap<LTerm, NativeModule>,
}

impl NativeFunRegistry {
  pub fn new() -> Self {
    let mut new_self = Self {
      modules: HashMap::new(),
    };
    Self::register_preloaded_modules(&mut new_self);
    new_self
  }

  fn register_preloaded_modules(&mut self) {
    self
      .modules
      .insert(atom::from_str("erlang"), native_fun::erlang::new());
  }

  /// Check whether an MFA is loaded as a native function.
  pub fn mfa_exists(&self, mfa: &MFArity) -> bool {
    if let Some(module_def) = self.modules.get(&mfa.m) {
      if let Some(_fn_def) = module_def.functions.get(&mfa.get_funarity()) {
        return true;
      }
    }
    false
  }

  pub fn find_mfa(&self, mfa: &MFArity) -> Option<NativeFn> {
    if let Some(module_def) = self.modules.get(&mfa.m) {
      if let Some(fn_ptr) = module_def.functions.get(&mfa.get_funarity()) {
        return Some(*fn_ptr);
      }
    }
    None
  }
}
