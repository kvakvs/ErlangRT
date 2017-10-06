//! `module` module handles Erlang modules as collections of functions,
//! literals and attributes.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync;

//use defs::Word;
use emulator::funarity::FunArity;
use emulator::code::{InstrPointer, CodeOffset};
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
}

impl Module {
  /// Create an empty module wrapped in atomic refcounted refcell.
  pub fn new(name: LTerm) -> Ptr {
    sync::Arc::new(RefCell::new(
      Module{
        name,
        funs: BTreeMap::new(),
      }
    ))
  }


  pub fn name(&self) -> LTerm { self.name }


  pub fn lookup(&self, mfa: &IMFArity) -> Hopefully<InstrPointer> {
    let fa = mfa.get_funarity();
    match self.funs.get(&fa) {
      Some(offset) =>
        Ok(InstrPointer::new(self.name, offset.clone())),
      None => {
        let msg = format!("Function not found {} in {}", fa, self.name);
        Err(Error::FunctionNotFound(msg))
      }
    }
  }
}
