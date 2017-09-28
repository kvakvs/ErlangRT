// Code server loads modules and stores them in memory, handles code lookups
//
use std::collections::BTreeMap;

use mfargs;
use term::Term;
use types::Word;
use std::sync::Arc;

type ModulePtr = Arc<Box<Module>>;
type InstrIndex = Word;

// Defines a code position in module
pub struct InstrPointer {
  mod_id: ModulePtr,
  instr_index: InstrIndex,
}

impl InstrPointer {
  pub fn new(mod_id: ModulePtr, instr_index: InstrIndex) -> InstrPointer {
    InstrPointer { mod_id, instr_index }
  }
}

pub struct Module {
  name: Term,
  code: Vec<Word>,
}

pub struct CodeServer {
  // Mapping {atom(): module()}
  mods: BTreeMap<Term, Module>,
  search_path: Vec<String>,
}

impl CodeServer {
  pub fn new() -> CodeServer {
    CodeServer {
      mods: BTreeMap::new(),
      search_path: Vec::new(),
    }
  }

  // Find module:function/arity, load if not found
  pub fn lookup(&mut self, mfa: &mfargs::IMFArity) -> Option<InstrPointer> {
    None
  }
}
