// Code server loads modules and stores them in memory, handles code lookups
//
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use beam::loader;
use mfargs;
use rterror;
use term::Term;
use types::Word;

use std::sync::Arc;

type ModulePtr = Arc<Box<Module>>;
type InstrIndex = Word;


// Defines a code position in module
pub struct InstrPointer {
  mod_id: ModulePtr,
  instr_index: InstrIndex,
}

impl InstrPointer {
  pub fn new(mod_id: ModulePtr, instr_index: InstrIndex) -> InstrPointer {
    InstrPointer { mod_id, instr_index }
  }
}

pub struct Module {
  name: Term,
  code: Vec<Word>,
}

pub struct CodeServer {
  // Mapping {atom(): module()}
  mods: BTreeMap<Term, Module>,
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
  pub fn lookup(&self, mfa: &mfargs::IMFArity) -> Option<InstrPointer> {
    None
  }

  /// Loading the module. Pre-stage, finding the module file from search path
  pub fn load(&mut self, filename: &str) -> Result<(), rterror::Error> {
    match first_that_exists(&self.search_path, filename) {
      Some(first_filename) => {
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