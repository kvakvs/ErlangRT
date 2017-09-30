//!
//! Code server loads modules and stores them in memory, handles code lookups
//! as well as dynamic reloading and partial unloading.
//!
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use beam::loader; // this is TODO: changeable BEAM loader
use function;
use mfa;
use module;
use rterror;
use term::Term;
use types::Word;
use vm::VM;

use std::sync::Arc;

type InstrIndex = Word;

/// Defines a code position in module by referring to a function and an offset.
/// Function pointer is refcounted, and function points to a module which is
/// also refcounted.
pub struct InstrPointer {
  fun: function::Ptr,
  instr_index: InstrIndex,
}

impl InstrPointer {
  pub fn new(fun: function::Ptr, instr_index: InstrIndex) -> InstrPointer {
    InstrPointer { fun, instr_index }
  }
}

pub struct CodeServer {
  // Mapping {atom(): module()}
  mods: BTreeMap<Term, module::Ptr>,
  search_path: Vec<String>,
}

impl CodeServer {
  pub fn new() -> CodeServer {
    CodeServer {
      mods: BTreeMap::new(),
      search_path: vec!["priv/".to_string()],
    }
  }

  /// Find module:function/arity
  pub fn lookup(&self, mfa: &mfa::IMFArity) -> Option<InstrPointer> {
    None
  }

  /// Loading the module. Pre-stage, finding the module file from search path
  pub fn load(&mut self, filename: &str)
    -> Result<(module::Ptr, String), rterror::Error>
  {
    match first_that_exists(&self.search_path, filename) {
      Some(first_filename) => {
        // Delegate the loading task to BEAM or another loader
        let mut loader = loader::Loader::new();
        loader.load(&first_filename)
      },
      None => Err(rterror::Error::FileNotFound(filename.to_string()))
    }
  }
}

/// Iterate through the search path list and try to find a file
fn first_that_exists(search_path: &Vec<String>,
                     filename: &str) -> Option<PathBuf> {
  for s in search_path {
    let full_path = format!("{}/{}.beam", s, filename).to_string();
    let p = Path::new(&full_path);
    if p.exists() {
      return Some(p.to_path_buf())
    }
  }
  None
}