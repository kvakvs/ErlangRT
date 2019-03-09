//! Code server loads modules and stores them in memory, handles code lookups
//! as well as dynamic reloading and partial unloading.

use crate::{
  beam::loader,
  command_line_args::ErlStartArgs,
  emulator::{
    atom,
    code::{pointer::VersionedCodePtr, CodePtr},
    mfa::ModFunArity,
    module::{Module, VersionedModuleName},
  },
  fail::{RtErr, RtResult},
  native_fun::{registry::NativeFunRegistry, NativeFn},
  term::lterm::*,
};
use std::{
  collections::BTreeMap,
  path::{Path, PathBuf},
};

fn module() -> &'static str {
  "code_srv: "
}

// Contains 2 versions of module code: current and previous
#[allow(dead_code)]
struct ModuleGenerations {
  curr_modp: Box<Module>,
  curr_version: usize,
  // old module pointer can be None, then version makes no sense
  old_modp: Option<Box<Module>>,
  old_version: usize,
}

pub enum MFALookupResult {
  FoundBeamCode(CodePtr),
  FoundBif(NativeFn),
  /* TODO: also NIF?
   * FoundNif(?), */
}

pub struct CodeServer {
  // Mapping {atom(): ModuleGenerations} where generations contains current
  // and previous mod versions
  mods: BTreeMap<Term, ModuleGenerations>,
  search_path: Vec<String>,
  mod_version: usize,

  pub native_functions: NativeFunRegistry,
}

impl CodeServer {
  pub fn new(args: &mut ErlStartArgs) -> CodeServer {
    CodeServer {
      mod_version: 1,
      mods: BTreeMap::new(),
      search_path: args.search_path.clone(),
      native_functions: NativeFunRegistry::new(),
    }
  }

  /// Performs classification whether an MFA is a native_fun, a code location or
  /// something else.
  /// Arg: `allow_load` allows loading another BEAM file as needed
  pub fn lookup_mfa(
    &mut self,
    mfa: &ModFunArity,
    allow_load: bool,
  ) -> RtResult<MFALookupResult> {
    // It could be a BIF
    if let Some(bif_fn) = self.native_functions.find_mfa(mfa) {
      return Ok(MFALookupResult::FoundBif(bif_fn));
    }
    // Try look for a BEAM export somewhere
    if let Ok(code_p) = if allow_load {
      self.lookup_beam_code_and_load(mfa)
    } else {
      self.lookup_beam_code(mfa)
    } {
      return Ok(MFALookupResult::FoundBeamCode(code_p));
    }
    // TODO: Look up a NIF extension function when those are supported
    Err(RtErr::NotFound)
  }

  /// Find module:function/arity in BEAM code (i.e. exported by some module)
  /// Returns: Versioned pointer to code, suitable for storing
  pub fn lookup_beam_code_versioned(
    &self,
    mfarity: &ModFunArity,
  ) -> RtResult<VersionedCodePtr> {
    let m = mfarity.m;
    match self.mods.get(&m) {
      None => {
        let msg = format!("{}Module not found {}", module(), m);
        Err(RtErr::ModuleNotFound(msg))
      }
      Some(mptr) => {
        let v = VersionedModuleName::new(m, mptr.curr_version);
        let code_p = mptr.curr_modp.lookup(mfarity)?;
        Ok(VersionedCodePtr::new(v, code_p))
      }
    }
  }

  /// Find module:function/arity in BEAM code (i.e. exported by some module)
  /// Returns: Memory pointer to code, not versioned (do not store)
  pub fn lookup_beam_code(&self, mfarity: &ModFunArity) -> RtResult<CodePtr> {
    let m = mfarity.m;
    match self.mods.get(&m) {
      None => {
        let msg = format!("{}Module not found {}", module(), m);
        Err(RtErr::ModuleNotFound(msg))
      }
      Some(mptr) => mptr.curr_modp.lookup(mfarity),
    }
  }

  /// Find the module file from search path and return the path or error.
  pub fn find_module_file(&mut self, filename: &str) -> RtResult<PathBuf> {
    match first_that_exists(&self.search_path, filename) {
      Some(found_first) => Ok(found_first),
      None => Err(RtErr::FileNotFound(filename.to_string())),
    }
  }

  /// Notify the code server about the fact that a new module is ready to be
  /// added to the codebase.
  pub fn module_loaded(&mut self, mod_ptr: Box<Module>) {
    let name = mod_ptr.versioned_name.module;
    let v = mod_ptr.versioned_name.version;
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
  pub fn lookup_beam_code_and_load(&mut self, mfarity: &ModFunArity) -> RtResult<CodePtr> {
    // Try lookup once, then load if not found
    match self.lookup_beam_code(mfarity) {
      Ok(ip) => return Ok(ip),
      Err(_e) => {
        let mod_name = atom::to_str(mfarity.m)?;
        let found_mod = self.find_module_file(&mod_name).unwrap();

        self.try_load_module(&found_mod)?;
      }
    };
    // Try lookup again
    match self.lookup_beam_code(mfarity) {
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
        Err(RtErr::FunctionNotFound(msg))
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
  pub fn code_reverse_lookup(&self, ip: CodePtr) -> Option<ModFunArity> {
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

  pub fn next_module_version(&mut self, _m: Term) -> usize {
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

// External API guarded by mutex
//

//#[allow(dead_code)]
//#[inline]
// pub fn lookup_no_load(mfarity: &MFArity) -> Hopefully<CodePtr> {
//  let cs = CODE_SRV.read().unwrap();
//  cs.lookup(mfarity)
//}

//#[inline]
// pub fn lookup_and_load(mfarity: &MFArity) -> Hopefully<CodePtr> {
//  // TODO: Optimize by write-locking the load part and read-locking the lookup part
//  let mut cs = CODE_SRV.write().unwrap();
//  cs.lookup_and_load(mfarity)
//}

//#[inline]
// pub fn code_reverse_lookup(ip: CodePtr) -> Option<MFArity> {
//  let cs = CODE_SRV.read().unwrap();
//  cs.code_reverse_lookup(ip)
//}

//#[inline]
// pub fn lookup_far_pointer(farp: FarCodePointer) -> Option<CodePtr> {
//  let cs = CODE_SRV.read().unwrap();
//  cs.lookup_far_pointer(farp)
//}
