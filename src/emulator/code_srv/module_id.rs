//! Versioning for modules and unique id to find an older module even if it was
//! overridden by a newer version.

use term::lterm::LTerm;
use term::lterm::aspect_atom::{AtomAspect};


/// Defines a module version, unique integer which is incremented on each
/// module load or reload by the Code Server.
#[derive(Debug, Copy, Clone)]
pub struct ModuleVersion(usize);

impl ModuleVersion {
  pub fn new(ver: usize) -> ModuleVersion {
    ModuleVersion(ver)
  }


  pub fn value(&self) -> usize {
    let ModuleVersion(val) = *self;
    val
  }
}


/// An unique identifier of a module where multiple modules with the same
/// name may exist. Each new module is granted a new `version`.
/// Versions are managed by the Code Server.
#[derive(Debug, Copy, Clone)]
pub struct VersionedModuleId {
  module: LTerm,
  version: ModuleVersion,
}


impl VersionedModuleId {
  pub fn new(module: LTerm, version: ModuleVersion) -> VersionedModuleId {
    debug_assert!(module.is_atom());
    VersionedModuleId {
      module, version
    }
  }


  pub fn module(&self) -> LTerm { self.module }
  pub fn version(&self) -> ModuleVersion { self.version }
}
