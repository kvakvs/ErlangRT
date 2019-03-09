use super::load_time_structs::*;

/// Stage 1 raw structures, as loaded and decoded from the beam file but not
/// ready to be used in runtime
pub struct LoaderRaw {
  /// Raw atoms loaded from BEAM module as strings
  pub atoms: Vec<String>,
  pub imports: Vec<LImport>,
  pub exports: Vec<LExport>,
  pub locals: Vec<LExport>,
  pub lambdas: Vec<LFun>,
  /// Temporary storage for loaded code, will be parsed in stage 2
  pub code: Vec<u8>,
}

impl LoaderRaw {
  pub fn new() -> LoaderRaw {
    LoaderRaw {
      atoms: Vec::new(),
      imports: Vec::new(),
      exports: Vec::new(),
      locals: Vec::new(),
      lambdas: Vec::new(),
      code: Vec::new(),
    }
  }
}
