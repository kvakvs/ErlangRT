//!
//! Code server loads modules and stores them in memory, handles code lookups
//! as well as dynamic reloading and partial unloading.
//!
pub mod module_id;

use std::{
  collections::BTreeMap,
  path::{Path, PathBuf},
};

use crate::{
  beam::loader,
  emulator::{
    atom,
    code::{pointer::FarCodePointer, CodePtr},
    mfa::MFArity,
    module,
  },
  fail::{Error, RtResult},
  term::lterm::*,
};

fn module() -> &'static str {
  "code_srv: "
}

// Contains 2 versions of module code: current and previous
#[allow(dead_code)]
struct ModuleGenerations {
  curr_modp: module::Ptr,
  curr_version: usize,
  // old module pointer can be None, then version makes no sense
  old_modp: Option<module::Ptr>,
  old_version: usize,
}

pub struct CodeServer {
  // Mapping {atom(): ModuleGenerations} where generations contains current
  // and previous mod versions
  mods: BTreeMap<LTerm, ModuleGenerations>,
  search_path: Vec<String>,
  mod_version: usize,
}

//lazy_static! {
//  static ref CODE_SRV: RwLock<CodeServer> = {
//    RwLock::new(CodeServer::new())
//  };
//
//  // TODO: Maybe use map<modulename, counter> here
//  static ref MOD_VERSION: AtomicUsize = {
//    AtomicUsize::new(0)
//  };
//}

impl CodeServer {
  pub fn new() -> CodeServer {
    CodeServer {
      mod_version: 1,
      mods: BTreeMap::new(),
      search_path: vec![
        "priv/".to_string(),
        // "/home/kv/r20/lib/erts-9.1/ebin/".to_string(),
      ],
    }
  }

  // Find a module and verify that the given version exists
  pub fn lookup_far_pointer(&self, farp: FarCodePointer) -> Option<CodePtr> {
    match self.mods.get(&farp.mod_id.module) {
      None => None,
      Some(_mptr) => panic!("Not impl"),
    }
  }

  /// Find module:function/arity
  pub fn lookup(&self, mfarity: &MFArity) -> RtResult<CodePtr> {
    let m = mfarity.m;
    match self.mods.get(&m) {
      None => {
        let msg = format!("{}Module not found {}", module(), m);
        Err(Error::ModuleNotFound(msg))
      }
      Some(mptr) => mptr.curr_modp.lookup(mfarity),
    }
  }

  /// Find the module file from search path and return the path or error.
  pub fn find_module_file(&mut self, filename: &str) -> RtResult<PathBuf> {
    match first_that_exists(&self.search_path, filename) {
      Some(found_first) => Ok(found_first),
      None => Err(Error::FileNotFound(filename.to_string())),
    }
  }

  /// Notify the code server about the fact that a new module is ready to be
  /// added to the codebase.
  pub fn module_loaded(&mut self, mod_ptr: module::Ptr) {
    let name = mod_ptr.mod_id.module;
    let v = mod_ptr.mod_id.version;
    let mg = ModuleGenerations {
      curr_modp: mod_ptr,
      curr_version: v,
      old_modp: None,
      old_version: 0,
    };
    self.mods.insert(name, mg);
  }

  /// Lookup, which will attempt to load a missing module if lookup fails
  /// on the first attempt.
  pub fn lookup_and_load(&mut self, mfarity: &MFArity) -> RtResult<CodePtr> {
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
        let msg = format!(
          "{}Func undef: {}:{}/{}",
          module(),
          mod_str,
          fun_str,
          mfarity.arity
        );
        Err(Error::FunctionNotFound(msg))
      }
    }
  }

  /// Internal function: runs 3 stages of module loader and returns an atomic
  /// refc (Arc) module pointer or an error
  fn try_load_module(&mut self, mod_file_path: &PathBuf) -> RtResult<()> {
    let mod_ptr = loader::load_module(self, mod_file_path)?;
    self.module_loaded(mod_ptr);
    Ok(())
  }

  /// Given a code address try find a module and function where this belongs.
  // TODO: Optimize search by giving a module name hint and using a range tree
  pub fn code_reverse_lookup(&self, ip: CodePtr) -> Option<MFArity> {
    for val in self.mods.values() {
      let modp = &val.curr_modp;
      let lresult = modp.code_reverse_lookup(ip);
      // TODO: Might be situation when ip points to old version of a module
      if lresult.is_some() {
        return lresult;
      }
      // nope, keep searching
    }
    None
  }

  pub fn next_module_version(&mut self, _m: LTerm) -> usize {
    let ver = self.mod_version;
    self.mod_version += 1;
    ver
  }
}

/// Iterate through the search path list and try to find a file
fn first_that_exists(search_path: &[String], filename: &str) -> Option<PathBuf> {
  for s in search_path {
    let full_path = format!("{}/{}.beam", s, filename).to_string();
    let p = Path::new(&full_path);
    if p.exists() {
      return Some(p.to_path_buf());
    }
  }
  None
}

//
// External API guarded by mutex
//

//#[allow(dead_code)]
//#[inline]
//pub fn lookup_no_load(mfarity: &MFArity) -> Hopefully<CodePtr> {
//  let cs = CODE_SRV.read().unwrap();
//  cs.lookup(mfarity)
//}

//#[inline]
//pub fn lookup_and_load(mfarity: &MFArity) -> Hopefully<CodePtr> {
//  // TODO: Optimize by write-locking the load part and read-locking the lookup part
//  let mut cs = CODE_SRV.write().unwrap();
//  cs.lookup_and_load(mfarity)
//}

//#[inline]
//pub fn code_reverse_lookup(ip: CodePtr) -> Option<MFArity> {
//  let cs = CODE_SRV.read().unwrap();
//  cs.code_reverse_lookup(ip)
//}

//#[inline]
//pub fn lookup_far_pointer(farp: FarCodePointer) -> Option<CodePtr> {
//  let cs = CODE_SRV.read().unwrap();
//  cs.lookup_far_pointer(farp)
//}
