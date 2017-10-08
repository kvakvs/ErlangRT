//! `module` module handles Erlang modules as collections of functions,
//! literals and attributes.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync;

use defs::Word;
use emulator::funarity::FunArity;
use emulator::code::{CodePtr, CodeOffset, Code};
use emulator::mfa::IMFArity;
use fail::{Hopefully, Error};
use term::lterm::LTerm;

pub type Ptr = sync::Arc<RefCell<Module>>;
pub type Weak = sync::Weak<RefCell<Module>>;

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
}

impl Module {
  /// Create an empty module wrapped in atomic refcounted refcell.
  pub fn new(name: LTerm) -> Ptr {
    sync::Arc::new(RefCell::new(
      Module{
        name,
        code: Vec::new(),
        funs: BTreeMap::new(),
      }
    ))
  }


  /// Get module name field
  pub fn name(&self) -> LTerm { self.name }


  /// Find a funarity or mfarity in the functions table.
  pub fn lookup(&self, mfa: &IMFArity) -> Hopefully<CodePtr> {
    let fa = mfa.get_funarity();
    //println!("mod Lookup {}/{}", fa.f, fa.arity);

    match self.funs.get(&fa) {
      Some(c_offset) => {
        let CodeOffset::Val(offset) = *c_offset;
        let p = &self.code[offset] as *const Word;
        Ok(CodePtr::Ptr(p))
      },
      None => {
        let msg = format!("Function not found {} in {}", fa, self.name);
        Err(Error::FunctionNotFound(msg))
      }
    }
  }
}
