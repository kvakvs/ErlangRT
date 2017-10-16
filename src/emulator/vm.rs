//!
//! Implements virtual machine, as a collection of processes and their
//! registrations, schedulers, ETS tables and atom table etc.
//!
use std::path::PathBuf;
use std::sync::Arc;

use beam::loader;
//use beam::vm_loop;
use defs::Word;
use emulator::atom;
use emulator::code;
use emulator::code_srv;
use emulator::mfa::{MFArity, MFArgs};
use emulator::module;
use emulator::process::Process;
use emulator::scheduler::{Prio, Scheduler};
use fail::{Hopefully, Error};
use term::lterm::LTerm;


fn module() -> &'static str { "vm: " }

//
// VM environment, heaps, atoms, tables, processes all goes here
//
pub struct VM {
  /// Pid counter increments every time a new process is spawned
  pid_counter: Word,

  code_srv: code_srv::CodeServer,

  pub scheduler: Scheduler,
}

impl VM {
  pub fn new() -> VM {
    VM {
      pid_counter: 0,
      code_srv: code_srv::CodeServer::new(),
      scheduler: Scheduler::new(),
    }
  }

  // Spawn a new process, create a new pid, register the process and jump to the MFA
  pub fn create_process(&mut self, parent: LTerm, mfargs: &MFArgs,
                        prio: Prio) -> Hopefully<LTerm> {
    let pid_c = self.pid_counter;
    self.pid_counter += 1;
    let pid = LTerm::make_pid(pid_c);
    let mfarity = mfargs.get_mfarity();
    match Process::new(self, pid, parent, &mfarity, prio) {
      Ok(p0) => {
        self.scheduler.add(pid, p0);
        Ok(pid)
      },
      Err(e) => Err(e)
    }
  }

  /// Run the VM loop (one time slice), call this repeatedly to run forever.
  /// Returns: false if VM quit, true if can continue
  pub fn tick(&mut self) -> bool {
    self.dispatch()
  }

  /// Mutable lookup, will load module if lookup fails the first time
  pub fn code_lookup(&mut self, mfarity: &MFArity) -> Hopefully<code::CodePtr>
  {
    // Try lookup once, then load if not found
    match self.code_srv.lookup(mfarity) {
      Ok(ip) => return Ok(ip),
      Err(_e) => {
        let mod_name = atom::to_str(mfarity.m);
        let found_mod = self.code_srv.find_module_file(&mod_name).unwrap();

        self.try_load_module(&found_mod)?;
      }
    };
    // Try lookup again
    match self.code_srv.lookup(mfarity) {
      Ok(ip) => Ok(ip),
      Err(_e) => {
        let mod_str = atom::to_str(mfarity.m);
        let fun_str = atom::to_str(mfarity.f);
        let msg = format!("{}Func undef: {}:{}/{}",
                          module(), mod_str, fun_str, mfarity.arity);
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
    loader.load(mod_file_path).unwrap();
    loader.load_stage2();
    match loader.load_finalize() {
      Ok(mod_ptr) => {
        self.code_srv.module_loaded(Arc::clone(&mod_ptr));
        Ok(mod_ptr)
      },
      Err(e) => Err(e)
    }
  }
}
