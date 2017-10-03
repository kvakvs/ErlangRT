//!
//! Code server loads modules and stores them in memory, handles code lookups
//! as well as dynamic reloading and partial unloading.
//!
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use beam::loader; // this is TODO: changeable BEAM loader
use emulator::function;
use emulator::mfa;
use emulator::module;
use emulator::vm::VM;
use rterror;
use term::low_level::Term;
use defs::Word;

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

  /// Find the module file from search path and return the path or error.
  pub fn find_module_file(&mut self, filename: &str)
    -> Result<PathBuf, rterror::Error>
  {
    match first_that_exists(&self.search_path, filename) {
      Some(found_first) => Ok(found_first),
      None => Err(rterror::Error::FileNotFound(filename.to_string()))
    }
  }

  /// Notify the code server about the fact that a new module is ready to be
  /// added to the codebase.
  pub fn module_loaded(&mut self, mod_ptr: module::Ptr) {
    self.mods.insert(mod_ptr.name(), mod_ptr);
    ()
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