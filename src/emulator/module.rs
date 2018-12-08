//! `module` module handles Erlang modules as collections of functions,
//! literals and attributes.
use crate::emulator::{
  code::{Code, CodePtr},
  code_srv::module_id::VersionedModuleId,
  funarity::FunArity,
  function::FunEntry,
  gen_atoms,
};
//use emulator::export::Export;
use crate::{
  emulator::{heap::Heap, mfa::MFArity},
  fail::{Error, RtResult},
  defs::{Word, WORD_BYTES},
  term::lterm::LTerm,
};
use std::collections::BTreeMap;


pub type Ptr = Box<Module>;


/// Stores f/arity mapping to offset in code.
pub type ModuleFunTable = BTreeMap<FunArity, usize>;


/// Represents a module with collection of functions. Modules are refcounted
/// and can be freed early if the situation allows.
#[derive(Debug)]
pub struct Module {
  pub mod_id: VersionedModuleId,

  /// Map to functions
  pub funs: ModuleFunTable,

  pub lambdas: Vec<FunEntry>,

  // TODO: attrs
  // TODO: lit table
  pub code: Code,
  pub lit_heap: Heap, // set by module loader
}

impl Module {
  /// Create an empty module wrapped in atomic refcounted refcell.
  pub fn new(mod_id: &VersionedModuleId) -> Ptr {
    Box::new(Module {
      code: Vec::new(),
      funs: BTreeMap::new(),
      lit_heap: Heap::new(1),
      mod_id: *mod_id,
      lambdas: Vec::new(),
    })
  }


  /// Get module name field
  pub fn name(&self) -> LTerm {
    self.mod_id.module()
  }


  /// Find a `m:f/arity` in the functions table, `m` is checked to be equal to
  /// this module's name.
  pub fn lookup(&self, mfa: &MFArity) -> RtResult<CodePtr> {
    assert_eq!(self.name(), mfa.m);

    let fa = mfa.get_funarity();
    //println!("mod Lookup {}/{}", fa.f, fa.arity);
    self.lookup_fa(&fa)
  }


  /// Find a `f/arity` in the functions table.
  pub fn lookup_fa(&self, fa: &FunArity) -> RtResult<CodePtr> {
    match self.funs.get(fa) {
      Some(offset) => {
        let p = &self.code[*offset] as *const Word;
        Ok(CodePtr::new(p))
      }
      None => {
        let msg = format!("Function not found {} in {}", fa, self.mod_id.module());
        Err(Error::FunctionNotFound(msg))
      }
    }
  }


  /// Check whether IP belongs to this module's code range, and if so, try and
  /// find the MFA for the code location.
  // TODO: Use some smart range tree or binary search or something
  pub fn code_reverse_lookup(&self, ip: CodePtr) -> Option<MFArity> {
    if !ip.belongs_to(&self.code) {
      return None;
    }

    // Find a function with closest code offset less than ip

    // some sane starting value: Code size in Words
    let mut min_dist = self.code.len() + 1;
    let mut fa = FunArity::new(gen_atoms::UNDEFINED, 0);

    let code_begin = &self.code[0] as *const Word;
    assert!(ip.get() > code_begin);
    let ip_offset = (ip.get() as usize - code_begin as usize) / WORD_BYTES;

    for (key, export_offset) in &self.funs {
      //let &CodeOffset(fn_offset) = fn_offset0;
      if ip_offset >= *export_offset {
        let dist = ip_offset - *export_offset;
        if dist < min_dist {
          min_dist = dist;
          fa = key.clone();
        }
      }
    }

    let mfa = MFArity::new_from_funarity(self.mod_id.module(), &fa);
    Some(mfa)
  }
}
