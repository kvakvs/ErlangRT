use crate::defs::Arity;

/// Imports table item (mfa's referred by this module).
/// Raw data structure as loaded from BEAM file
pub struct LtImport {
  pub mod_atom_i: usize,
  pub fun_atom_i: usize,
  pub arity: Arity,
}

/// Exports table item, as specified in `-export()` attribute.
/// Raw data structure as loaded from BEAM file.
pub struct LtExport {
  pub fun_atom_i: usize,
  pub arity: Arity,
  pub label: usize,
}

/// Function closures used in this file, with info on captured values.
/// Raw data structure as loaded from BEAM file
pub struct LtFun {
  pub arity: Arity,
  pub fun_atom_i: usize,
  pub code_pos: usize,
  pub index: usize,
  pub nfrozen: usize,
  pub ouniq: usize,
}
