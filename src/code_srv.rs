// Code server loads modules and stores them in memory, handles code lookups
//
use std::collections::BTreeMap;
use std::path::Path;

use mfargs;
use util::reader;
use rterror;
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
      search_path: vec!["priv/".to_string()],
    }
  }

  // Find module:function/arity, load if not found
  pub fn lookup(&self, mfa: &mfargs::IMFArity) -> Result<InstrPointer, rterror::Error> {
    let e = rterror::Error::FileNotFound("hello world".to_string());
    Err(e)
  }

  pub fn load(&mut self, filename: &str) -> Result<(), rterror::Error> {
    let first_filename = find_first_that_exists(self.search_path, filename);
    let mut r = reader::Reader::new(&first_filename);
    Ok(())
  }
}

fn find_first_that_exists(search_path: Vec<String>, filename: &str) -> Option<Path> {
  for s in search_path {
    let full_path = format!("{}/{}.erl", s, filename);
    let p = Path::new(&full_path);
    if p.exists() { return Some(p) }
  }
  None
}