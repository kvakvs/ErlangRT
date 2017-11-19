//! `module` module handles Erlang modules as collections of functions,
//! literals and attributes.
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use emulator::gen_atoms;
use rt_defs::{Word, WORD_BYTES};
use emulator::code::{CodePtr, CodeOffset, Code};
use emulator::funarity::FunArity;
use emulator::heap::{Heap};
use emulator::mfa::MFArity;
use fail::{Hopefully, Error};
use term::lterm::*;

pub type Ptr = Arc<Mutex<Module>>;
//pub type Weak = sync::Weak<RefCell<Module>>;

pub type FunTable = BTreeMap<FunArity, CodeOffset>;

/// Represents a module with collection of functions. Modules are refcounted
/// and can be freed early if the situation allows.
#[derive(Debug)]
pub struct Module {
  name: LTerm,
  /// Map to refcounted functions
  pub funs: FunTable,
  // TODO: attrs
  // TODO: lit table
  pub code: Code,
  pub lit_heap: Heap, // set by module loader
}

impl Module {
  /// Create an empty module wrapped in atomic refcounted refcell.
  pub fn new(name: LTerm) -> Ptr {
    Arc::new(Mutex::new(
      Module {
        code: Vec::new(),
        funs: BTreeMap::new(),
        lit_heap: Heap::new(1),
        name,
      }
    ))
  }


  /// Get module name field
  pub fn name(&self) -> LTerm { self.name }


  /// Find a funarity or mfarity in the functions table.
  pub fn lookup(&self, mfa: &MFArity) -> Hopefully<CodePtr> {
    let fa = mfa.get_funarity();
    //println!("mod Lookup {}/{}", fa.f, fa.arity);

    match self.funs.get(&fa) {
      Some(c_offset) => {
        let CodeOffset(offset) = *c_offset;
        let p = &self.code[offset] as *const Word;
        Ok(CodePtr(p))
      },
      None => {
        let msg = format!("Function not found {} in {}", fa, self.name);
        Err(Error::FunctionNotFound(msg))
      }
    }
  }


  /// Check whether IP belongs to this module's code range, and if so, try and
  /// find the MFA for the code location.
  // TODO: Use some smart range tree or binary search or something
  pub fn code_reverse_lookup(&self, ip: &CodePtr) -> Option<MFArity> {
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
    //println!("crl: ip_offs {}", ip_offset);

    for (key, fn_offset0) in &self.funs {
      let &CodeOffset(fn_offset) = fn_offset0;
      if ip_offset >= fn_offset {
        let dist = ip_offset - fn_offset;
        //println!("crl: mindist {}, dist {}, fa {}", min_dist, dist, key);
        if dist < min_dist {
          min_dist = dist;
          fa = key.clone();
        }
      }
    }

    let mfa = MFArity::new_from_funarity(self.name, &fa);
    Some(mfa)
  }
}
