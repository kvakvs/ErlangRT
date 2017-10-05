//! `module` module handles Erlang modules as collections of functions,
//! literals and attributes.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync;

use defs::Word;
use emulator::funarity::FunArity;
use emulator::function;
use emulator::instr_pointer::InstrPointer;
use emulator::mfa::IMFArity;
use fail::{Hopefully, Error};
use term::lterm::LTerm;

pub type Ptr = sync::Arc<RefCell<Module>>;
pub type Weak = sync::Weak<RefCell<Module>>;

/// Cross-function label pointer inside the module.
// TODO: Change the BTree to something reasonable
pub struct CodeLabel {
  pub fun: function::Weak,
  pub offset: Word,
}

/// Represents a module with collection of functions. Modules are refcounted
/// and can be freed early if the situation allows.
pub struct Module {
  name: LTerm,
  /// Map to refcounted functions
  pub funs: BTreeMap<FunArity, function::Ptr>,
  // TODO: attrs
  // TODO: lit table
  pub labels: BTreeMap<Word, CodeLabel>,
}

impl Module {
  /// Create an empty module wrapped in atomic refcounted refcell.
  pub fn new(name: LTerm) -> Ptr {
    sync::Arc::new(RefCell::new(
      Module{
        name,
        funs: BTreeMap::new(),
        labels: BTreeMap::new(),
      }
    ))
  }

  pub fn name(&self) -> LTerm { self.name }

  pub fn lookup(&self, mfa: &IMFArity) -> Hopefully<InstrPointer> {
    let fa = mfa.get_funarity();
    match self.funs.get(&fa) {
      Some(fptr) =>
        Ok(InstrPointer::new(fptr.clone(), 0)),
      None => {
        let msg = format!("Function not found {} in {}", fa, self.name);
        Err(Error::FunctionNotFound(msg))
      }
    }
  }
}
