use crate::{
  defs::Arity,
  emulator::{atom, funarity::FunArity},
  native_fun::NativeFn,
};

/// A nativefn entry for lookup tables and preloaded module tables.
pub struct NativeFnEntry {
  pub fa: FunArity,
  pub func: super::NativeFn,
}

impl NativeFnEntry {
  pub fn with_str(fun: &str, arity: Arity, func: NativeFn) -> Self {
    let fa = FunArity::new(atom::from_str(fun), arity);
    Self { fa, func }
  }
}
