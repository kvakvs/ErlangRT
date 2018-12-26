//! Implements virtual machine, as a collection of processes and their
//! registrations, schedulers, ETS tables and atom table etc.

use crate::{
  defs::Word,
  emulator::{
    code_srv::CodeServer,
    mfa::MFASomething,
    process::Process,
    scheduler::{Prio, Scheduler},
  },
  fail::RtResult,
  term::lterm::*,
};

// fn module() -> &'static str { "vm: " }

/// VM environment, heaps, tables, processes all goes here.
/// Atoms are a global API in `atom.rs`.
/// Code server is a global API in `code_srv.rs`.
pub struct VM {
  /// Pid counter increments every time a new process is spawned
  pid_counter: Word,

  /// Contains all loaded modules and manages versions
  pub code_server: CodeServer,

  pub scheduler: Scheduler,
}

impl VM {
  /// Create a VM, multiple VMs can be created but atom table and code server
  /// will be shared (global).
  pub fn new() -> VM {
    VM {
      code_server: CodeServer::new(),
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

  /// Dirty trick to not have to dynamically borrow code server via
  /// `RefCell<Box<>>` because code server lives just as long as the VM itself.
  pub fn get_code_server_p(&self) -> *mut CodeServer {
    let p = &self.code_server as *const CodeServer;
    p as *mut CodeServer
  }

  /// Spawn a new process, create a new pid, register the process and jump to
  /// the MFA specified. Arguments are copies into the new process heap and
  /// stored into the registers.
  pub fn create_process(
    &mut self,
    parent: LTerm,
    mfargs: &MFASomething,
    prio: Prio,
  ) -> RtResult<LTerm> {
    let pid_c = self.pid_counter;
    self.pid_counter += 1;

    let pid = LTerm::make_local_pid(pid_c);
    let mfarity = mfargs.get_mfarity();
    let cs = self.get_code_server_p();
    let mut p0 = Process::new(pid, parent, &mfarity, prio, unsafe { &mut (*cs) })?;
    p0.set_spawn_args(&mfargs);
    self.scheduler.register_new_process(pid, p0);
    Ok(pid)
  }

  /// Run the VM loop (one time slice), call this repeatedly to run forever.
  /// Time slice ends when a current process yields or when reduction count
  /// reaches zero.
  #[inline]
  pub fn tick(&mut self) -> RtResult<bool> {
    self.dispatch()
  }
}
