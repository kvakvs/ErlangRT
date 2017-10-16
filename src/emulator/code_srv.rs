//!
//! Code server loads modules and stores them in memory, handles code lookups
//! as well as dynamic reloading and partial unloading.
//!
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use emulator::code::CodePtr;
use emulator::mfa;
use emulator::module;
use fail::{Hopefully, Error};
use term::lterm::LTerm;

fn module() -> &'static str { "code_srv: " }

pub struct CodeServer {
  // Mapping {atom(): module()}
  mods: BTreeMap<LTerm, module::Ptr>,
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
  pub fn lookup(&self, mfa: &mfa::MFArity) -> Hopefully<CodePtr> {
    let m = mfa.m;
    match self.mods.get(&m) {
      None => {
        let msg = format!("{}Module not found {}", module(), m);
        Err(Error::ModuleNotFound(msg))
      },
      Some(mptr) => mptr.borrow().lookup(mfa)
    }
  }

  /// Find the module file from search path and return the path or error.
  pub fn find_module_file(&mut self, filename: &str) -> Hopefully<PathBuf>
  {
    match first_that_exists(&self.search_path, filename) {
      Some(found_first) => Ok(found_first),
      None => Err(Error::FileNotFound(filename.to_string()))
    }
  }

  /// Notify the code server about the fact that a new module is ready to be
  /// added to the codebase.
  pub fn module_loaded(&mut self, mod_ptr: module::Ptr) {
    let name = mod_ptr.borrow().name();
    self.mods.insert(name, mod_ptr);
    ()
  }
}

/// Iterate through the search path list and try to find a file
fn first_that_exists(search_path: &[String],
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
