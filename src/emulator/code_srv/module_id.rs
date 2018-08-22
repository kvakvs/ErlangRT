//! Versioning for modules and unique id to find an older module even if it was
//! overridden by a newer version.

use term::lterm::*;


/// An unique identifier of a module where multiple modules with the same
/// name may exist. Each new module is granted a new `version`.
/// Versions are managed by the Code Server.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct VersionedModuleId {
  pub module: LTerm,
  pub version: usize,
}


impl VersionedModuleId {
  pub fn new(module: LTerm, version: usize) -> VersionedModuleId {
    debug_assert!(module.is_atom());
    VersionedModuleId {
      module,
      version
    }
  }


  pub fn module(&self) -> LTerm { self.module }
}
