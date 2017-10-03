//!
//! Implements virtual machine, as a collection of processes and their
//! registrations, schedulers, ETS tables and atom table etc.
//!
use std::collections::BTreeMap;
use std::vec::Vec;
use std::path::PathBuf;

use beam::loader; // this is TODO: changeable BEAM loader
use emulator::code_srv;
use emulator::mfa;
use emulator::module;
use emulator::process::Process;
use rterror;
use term::low_level::Term;
use defs::Word;

fn module() -> &'static str { "vm: " }

//
// VM environment, heaps, atoms, tables, processes all goes here
//
pub struct VM {
  // Direct mapping string to atom index
  atoms: BTreeMap<String, Word>,
  // Reverse mapping atom index to string (sorted by index)
  atoms_r: Vec<String>,

  // Pid counter increments every time a new process is spawned
  pid_counter: Word,
  // Dict of pids to process boxes
  processes: BTreeMap<Term, Process>,

  code_srv: code_srv::CodeServer,
}

impl VM {
  pub fn new() -> VM {
    VM {
      atoms: BTreeMap::new(),
      atoms_r: Vec::new(),
      pid_counter: 0,
      processes: BTreeMap::new(),
      code_srv: code_srv::CodeServer::new()
    }
  }

  // Allocate new atom in the atom table or find existing. Pack the atom index
  // as an immediate2 Term
  pub fn atom(&mut self, val: &str) -> Term {
    if self.atoms.contains_key(val) {
      return Term::make_atom(self.atoms[val]);
    }

    let index = self.atoms_r.len();

    let val1 = String::from(val);
    self.atoms.entry(val1).or_insert(index);

    let val2 = String::from(val);
    self.atoms_r.push(val2);

    Term::make_atom(index)
  }

  // Spawn a new process, create a new pid, register the process and jump to the MFA
  pub fn create_process(&mut self, parent: Term, mfa: &mfa::MFArgs)
    -> Result<Term, rterror::Error> {
    let pid_c = self.pid_counter;
    self.pid_counter += 1;
    let pid = Term::make_pid(pid_c);
    match Process::new(self, pid, parent, mfa) {
      Ok(p0) => {
        self.processes.insert(pid, p0);
        Ok(pid)
      },
      Err(e) => return Err(e)
    }
  }

  /// Run the VM loop (one time slice), call this repeatedly to run forever.
  /// Returns: false if VM quit, true if can continue
  pub fn tick(&mut self) -> bool {
    true
  }

  pub fn atom_to_str(&self, atom: Term) -> String {
    assert!(atom.is_atom());
    self.atoms_r[atom.atom_index()].to_string()
  }

  /// Mutable lookup, will load module if lookup fails the first time
  pub fn code_lookup(&mut self, mfa: &mfa::IMFArity)
    -> Result<code_srv::InstrPointer, rterror::Error>
  {
    // Try lookup once, then load if not found
    match self.code_srv.lookup(mfa) {
      Some(ip) => return Ok(ip),
      None => {
        let mod_name = self.atom_to_str(mfa.get_mod());
        let found_mod = self.code_srv.find_module_file(&mod_name).unwrap();

        self.try_load_module(&found_mod)?;
      }
    };
    // Try lookup again
    match self.code_srv.lookup(mfa) {
      Some(ip) => Ok(ip),
      None => {
        let mod_str = self.atom_to_str(mfa.get_mod());
        let fun_str = self.atom_to_str(mfa.get_fun());
        let msg = format!("{}Func undef: {}:{}/{}",
                          module(), mod_str, fun_str, mfa.get_arity());
        Err(rterror::Error::CodeLoadingFailed(msg))
      }
    }
  }

  /// Internal function: runs 3 stages of module loader and returns an atomic
  /// refc (Arc) module pointer or an error
  fn try_load_module(&mut self, mod_file_path: &PathBuf)
                     -> Result<module::Ptr, rterror::Error>
  {
    // Delegate the loading task to BEAM or another loader
    let mut loader = loader::Loader::new();
    // Phase 1: Preload data structures
    loader.load(&mod_file_path);
    loader.load_stage2(self);
    match loader.load_finalize() {
      Ok(mod_ptr) => {
        self.code_srv.module_loaded(mod_ptr.clone());
        Ok(mod_ptr)
      },
      Err(e) => Err(e)
    }
  }
}