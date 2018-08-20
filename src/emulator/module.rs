//! `module` module handles Erlang modules as collections of functions,
//! literals and attributes.
use std::collections::BTreeMap;

use emulator::gen_atoms;
use rt_defs::{Word, WORD_BYTES};
use emulator::code::{CodePtr, Code};
use emulator::code_srv::module_id::VersionedModuleId;
use emulator::funarity::FunArity;
use emulator::function::{FunEntry, CallableLocation};
use emulator::export::Export;
use emulator::heap::{Heap};
use emulator::mfa::MFArity;
use fail::{Hopefully, Error};
use term::lterm::LTerm;


pub type Ptr = Box<Module>;

//pub type RawPtr = *const Module;
//pub type MutRawPtr = *mut Module;


pub type FunTable = BTreeMap<FunArity, Export>;


/// Represents a module with collection of functions. Modules are refcounted
/// and can be freed early if the situation allows.
#[derive(Debug)]
pub struct Module {
  pub mod_id: VersionedModuleId,

  /// Map to functions
  pub funs: FunTable,

  pub lambdas: Vec<FunEntry>,

  // TODO: attrs
  // TODO: lit table
  pub code: Code,
  pub lit_heap: Heap, // set by module loader
}

impl Module {
  /// Create an empty module wrapped in atomic refcounted refcell.
  pub fn new(mod_id: &VersionedModuleId) -> Ptr {
    Box::new(
      Module {
        code: Vec::new(),
        funs: BTreeMap::new(),
        lit_heap: Heap::new(1),
        mod_id: *mod_id,
        lambdas: Vec::new(),
      }
    )
  }


  /// Get module name field
  pub fn name(&self) -> LTerm {
    self.mod_id.module()
  }


  /// Find a `m:f/arity` in the functions table, `m` is checked to be equal to
  /// this module's name.
  pub fn lookup(&self, mfa: &MFArity) -> Hopefully<CodePtr> {
    assert_eq!(self.name(), mfa.m);

    let fa = mfa.get_funarity();
    //println!("mod Lookup {}/{}", fa.f, fa.arity);
    self.lookup_fa(&fa)
  }


  /// Find a `f/arity` in the functions table.
  pub fn lookup_fa(&self, fa: &FunArity) -> Hopefully<CodePtr> {
    match self.funs.get(fa) {
      Some(export) => {
        match export.dst {
          CallableLocation::Code(code_p) => {
            let p = &self.code[code_p.offset] as *const Word;
            Ok(CodePtr(p))
          },
          _ => panic!("Only code pointers allowed in module funs table")
        }
      },
      None => {
        let msg = format!("Function not found {} in {}",
                          fa, self.mod_id.module());
        Err(Error::FunctionNotFound(msg))
      }
    }
  }


  /// Check whether IP belongs to this module's code range, and if so, try and
  /// find the MFA for the code location.
  // TODO: Use some smart range tree or binary search or something
  pub fn code_reverse_lookup(&self, ip: CodePtr) -> Option<MFArity> {
    if !ip.belongs_to(&self.code) {
      return None
    }

    // Find a function with closest code offset less than ip

    // some sane starting value: Code size in Words
    let mut min_dist = self.code.len() + 1;
    let mut fa = FunArity::new(gen_atoms::UNDEFINED, 0);

    let code_begin = &self.code[0] as *const Word;
    assert!(ip.get_ptr() > code_begin);
    let ip_offset = (ip.get_ptr() as usize - code_begin as usize) / WORD_BYTES;

    for (key, export) in &self.funs {
      //let &CodeOffset(fn_offset) = fn_offset0;
      match export.dst {
        CallableLocation::Code(far_ptr) => {
          if ip_offset >= far_ptr.offset {
            let dist = ip_offset - far_ptr.offset;
            if dist < min_dist {
              min_dist = dist;
              fa = key.clone();
            }
          }
        },
        _ => panic!("Code reverse lookup: Only code offsets can be in funs table")
      }
    }

    let mfa = MFArity::new_from_funarity(self.mod_id.module(), &fa);
    Some(mfa)
  }
}
