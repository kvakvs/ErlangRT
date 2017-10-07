//!
//! Implements virtual machine, as a collection of processes and their
//! registrations, schedulers, ETS tables and atom table etc.
//!
use std::collections::BTreeMap;
use std::path::PathBuf;

use beam::loader;
use defs::Word;
use emulator::atom;
use emulator::code_srv;
use emulator::code::InstrPointer;
use emulator::mfa;
use emulator::module;
use emulator::process::Process;
use fail::{Hopefully, Error};
use term::lterm::LTerm;

fn module() -> &'static str { "vm: " }

//
// VM environment, heaps, atoms, tables, processes all goes here
//
pub struct VM {
  /// Pid counter increments every time a new process is spawned
  pid_counter: Word,

  /// Dict of pids to process boxes
  processes: BTreeMap<LTerm, Process>,

  code_srv: code_srv::CodeServer,
}

impl VM {
  pub fn new() -> VM {
    VM {
      pid_counter: 0,
      processes: BTreeMap::new(),
      code_srv: code_srv::CodeServer::new()
    }
  }

  // Spawn a new process, create a new pid, register the process and jump to the MFA
  pub fn create_process(&mut self,
                        parent: LTerm,
                        mfa: &mfa::MFArgs) -> Hopefully<LTerm> {
    let pid_c = self.pid_counter;
    self.pid_counter += 1;
    let pid = LTerm::make_pid(pid_c);
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

  /// Mutable lookup, will load module if lookup fails the first time
  pub fn code_lookup(&mut self, mfa: &mfa::IMFArity) -> Hopefully<InstrPointer>
  {
    // Try lookup once, then load if not found
    match self.code_srv.lookup(mfa) {
      Ok(ip) => return Ok(ip),
      Err(_e) => {
        let mod_name = atom::to_str(mfa.get_mod());
        let found_mod = self.code_srv.find_module_file(&mod_name).unwrap();

        self.try_load_module(&found_mod)?;
      }
    };
    // Try lookup again
    match self.code_srv.lookup(mfa) {
      Ok(ip) => Ok(ip),
      Err(_e) => {
        let mod_str = atom::to_str(mfa.get_mod());
        let fun_str = atom::to_str(mfa.get_fun());
        let msg = format!("{}Func undef: {}:{}/{}",
                          module(), mod_str, fun_str, mfa.get_arity());
        Err(Error::FunctionNotFound(msg))
      }
    }
  }

  /// Internal function: runs 3 stages of module loader and returns an atomic
  /// refc (Arc) module pointer or an error
  fn try_load_module(&mut self,
                     mod_file_path: &PathBuf) -> Hopefully<module::Ptr>
  {
    // Delegate the loading task to BEAM or another loader
    let mut loader = loader::Loader::new();
    // Phase 1: Preload data structures
    loader.load(&mod_file_path).unwrap();
    loader.load_stage2();
    match loader.load_finalize() {
      Ok(mod_ptr) => {
        self.code_srv.module_loaded(mod_ptr.clone());
        Ok(mod_ptr)
      },
      Err(e) => Err(e)
    }
  }
}