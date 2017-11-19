//!
//! Code server loads modules and stores them in memory, handles code lookups
//! as well as dynamic reloading and partial unloading.
//!
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, Arc};

use beam::loader;
use emulator::atom;
use emulator::code::CodePtr;
use emulator::mfa::MFArity;
use emulator::module;
use fail::{Hopefully, Error};
use term::lterm::*;


fn module() -> &'static str { "code_srv: " }


pub struct CodeServer {
  // Mapping {atom(): module()}
  mods: BTreeMap<LTerm, module::Ptr>,
  search_path: Vec<String>,
}


lazy_static! {
  static ref CODE_SRV: Mutex<CodeServer> = {
    Mutex::new(CodeServer::new())
  };
}


impl CodeServer {
  pub fn new() -> CodeServer {
    CodeServer {
      mods: BTreeMap::new(),
      search_path: vec![
        "priv/".to_string(),
        // "/home/kv/r20/lib/erts-9.1/ebin/".to_string(),
      ],
    }
  }


  /// Find module:function/arity
  pub fn lookup(&self, mfarity: &MFArity) -> Hopefully<CodePtr> {
    let m = mfarity.m;
    match self.mods.get(&m) {
      None => {
        let msg = format!("{}Module not found {}", module(), m);
        Err(Error::ModuleNotFound(msg))
      },
      Some(mptr) => mptr.lock().unwrap().lookup(mfarity)
    }
  }


  /// Find the module file from search path and return the path or error.
  pub fn find_module_file(&mut self, filename: &str) -> Hopefully<PathBuf> {
    match first_that_exists(&self.search_path, filename) {
      Some(found_first) => Ok(found_first),
      None => Err(Error::FileNotFound(filename.to_string()))
    }
  }


  /// Notify the code server about the fact that a new module is ready to be
  /// added to the codebase.
  pub fn module_loaded(&mut self, mod_ptr: module::Ptr) {
    let name = mod_ptr.lock().unwrap().name();
    self.mods.insert(name, mod_ptr);
    ()
  }


  /// Lookup, which will attempt to load a missing module if lookup fails
  /// on the first attempt.
  pub fn lookup_and_load(&mut self, mfarity: &MFArity) -> Hopefully<CodePtr> {
    // Try lookup once, then load if not found
    match self.lookup(mfarity) {
      Ok(ip) => return Ok(ip),
      Err(_e) => {
        let mod_name = atom::to_str(mfarity.m)?;
        let found_mod = self.find_module_file(&mod_name).unwrap();

        self.try_load_module(&found_mod)?;
      }
    };
    // Try lookup again
    match self.lookup(mfarity) {
      Ok(ip) => Ok(ip),
      Err(_e) => {
        let mod_str = atom::to_str(mfarity.m)?;
        let fun_str = atom::to_str(mfarity.f)?;
        let msg = format!("{}Func undef: {}:{}/{}",
                          module(), mod_str, fun_str, mfarity.arity);
        Err(Error::FunctionNotFound(msg))
      }
    }
  }


  /// Internal function: runs 3 stages of module loader and returns an atomic
  /// refc (Arc) module pointer or an error
  fn try_load_module(&mut self,
                     mod_file_path: &PathBuf) -> Hopefully<module::Ptr>
  {
    // Delegate the loading task to BEAM or another loader
    let mut loader = loader::Loader::new();

    // Phase 1: Preload data structures
    loader.load(mod_file_path)?;
    loader.load_stage2()?;

    let mod_ptr = loader.load_finalize()?;
    self.module_loaded(Arc::clone(&mod_ptr));
    Ok(mod_ptr)
  }


  /// Given a code address try find a module and function where this belongs.
  // TODO: Optimize search by giving a module name hint and using a range tree
  pub fn code_reverse_lookup(&self, ip: &CodePtr) -> Option<MFArity> {
    for (_key, val) in &self.mods {
      let modp = val.lock().unwrap();
      let lresult = modp.code_reverse_lookup(ip);
      if lresult.is_some() {
        return lresult
      }
      // nope, keep searching
    }
    return None
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

//
// External API guarded by mutex
//

#[allow(dead_code)]
#[inline]
pub fn lookup_no_load(mfarity: &MFArity) -> Hopefully<CodePtr> {
  let cs = CODE_SRV.lock().unwrap();
  cs.lookup(mfarity)
}


#[inline]
pub fn lookup_and_load(mfarity: &MFArity) -> Hopefully<CodePtr> {
  let mut cs = CODE_SRV.lock().unwrap();
  cs.lookup_and_load(mfarity)
}


#[inline]
pub fn code_reverse_lookup(ip: &CodePtr) -> Option<MFArity> {
  let cs = CODE_SRV.lock().unwrap();
  cs.code_reverse_lookup(ip)
}
