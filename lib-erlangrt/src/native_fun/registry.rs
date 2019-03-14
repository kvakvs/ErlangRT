use crate::{
  emulator::{atom, mfa::ModFunArity},
  native_fun::{erlang, erts_internal, lists, module::NativeModule, NativeFn},
  term::value::Term,
};
use std::collections::HashMap;

/// Registry stores a tree of loaded native modules.
/// Native modules may overlap with Erlang modules, while providing implementations
/// for selected functions.
pub struct NativeFunRegistry {
  modules: HashMap<Term, NativeModule>,
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
    let a_erlang = atom::from_str("erlang");
    self.modules.insert(a_erlang, erlang::new());

    let a_ertsi = atom::from_str("erts_internal");
    self.modules.insert(a_ertsi, erts_internal::new());

    let a_lists = atom::from_str("lists");
    self.modules.insert(a_lists, lists::new());
  }

  /// Check whether an MFA is loaded as a native function.
  pub fn mfa_exists(&self, mfa: &ModFunArity) -> bool {
    if let Some(module_def) = self.modules.get(&mfa.m) {
      if let Some(_fn_def) = module_def.functions.get(&mfa.get_funarity()) {
        return true;
      }
    }
    false
  }

  pub fn find_mfa(&self, mfa: &ModFunArity) -> Option<NativeFn> {
    if let Some(module_def) = self.modules.get(&mfa.m) {
      if let Some(fn_ptr) = module_def.functions.get(&mfa.get_funarity()) {
        return Some(*fn_ptr);
      }
    }
    None
  }
}
