//! Implements virtual machine, as a collection of processes and their
//! registrations, schedulers, ETS tables and atom table etc.
//!

use crate::{
  defs::Word,
  emulator::{
    code_srv::CodeServer,
    mfa::MFArgs,
    process::Process,
    scheduler::{Prio, Scheduler},
  },
  fail::RtResult,
  term::lterm::*,
};
use std::cell::RefCell;

// fn module() -> &'static str { "vm: " }

/// VM environment, heaps, tables, processes all goes here.
/// Atoms are a global API in `atom.rs`.
/// Code server is a global API in `code_srv.rs`.
pub struct VM {
  /// Pid counter increments every time a new process is spawned
  pid_counter: Word,

  /// Contains all loaded modules and manages versions
  pub code_server: RefCell<Box<CodeServer>>,

  scheduler: Scheduler,
}

impl VM {
  /// Create a VM, multiple VMs can be created but atom table and code server
  /// will be shared (global).
  pub fn new() -> VM {
    VM {
      code_server: RefCell::new(Box::new(CodeServer::new())),
      pid_counter: 0,
      scheduler: Scheduler::new(),
    }
  }

  /// Dirty trick to not have to dynamically borrow scheduler via
  /// `RefCell<Box<>>` because schedulers live just as long as the VM itself.
  pub fn get_scheduler_p(&self) -> *mut Scheduler {
    let p = &self.scheduler as *const Scheduler;
    p as *mut Scheduler
  }

  /// Spawn a new process, create a new pid, register the process and jump to the MFA
  pub fn create_process(
    &mut self,
    parent: LTerm,
    mfargs: &MFArgs,
    prio: Prio,
  ) -> RtResult<LTerm> {
    let pid_c = self.pid_counter;
    self.pid_counter += 1;

    let pid = LTerm::make_local_pid(pid_c);
    let mfarity = mfargs.get_mfarity();
    match Process::new(
      pid,
      parent,
      &mfarity,
      prio,
      self.code_server.borrow_mut().as_mut(),
    ) {
      Ok(p0) => {
        self.scheduler.add(pid, p0);
        Ok(pid)
      }
      Err(e) => Err(e),
    }
  }

  /// Run the VM loop (one time slice), call this repeatedly to run forever.
  /// Time slice ends when a current process yields or when reduction count
  /// reaches zero.
  /// Returns: false if VM has quit, true if can continue.
  pub fn tick(&mut self) -> RtResult<bool> {
    self.dispatch()
  }
}
