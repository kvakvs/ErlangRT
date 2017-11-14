//!
//! Implements virtual machine, as a collection of processes and their
//! registrations, schedulers, ETS tables and atom table etc.
//!

use rt_defs::Word;
use emulator::mfa::{MFArgs};
use emulator::process::Process;
use emulator::scheduler::{Prio, Scheduler};
use fail::{Hopefully};
use term::lterm::*;


//fn module() -> &'static str { "vm: " }

///
/// VM environment, heaps, tables, processes all goes here.
/// Atoms are a global API in `atom.rs`.
/// Code server is a global API in `code_srv.rs`.
///
pub struct VM {
  /// Pid counter increments every time a new process is spawned
  pid_counter: Word,

  pub scheduler: Scheduler,
}

impl VM {

  /// Create a VM, multiple VMs can be created but atom table and code server
  /// will be shared (global).
  pub fn new() -> VM {
    VM {
      pid_counter: 0,
      scheduler: Scheduler::new(),
    }
  }

  /// Spawn a new process, create a new pid, register the process and jump to the MFA
  pub fn create_process(&mut self,
                        parent: LTerm,
                        mfargs: &MFArgs,
                        prio: Prio) -> Hopefully<LTerm> {
    let pid_c = self.pid_counter;
    self.pid_counter += 1;

    let pid = LTerm::make_pid(pid_c);
    let mfarity = mfargs.get_mfarity();
    match Process::new(pid, parent, &mfarity, prio) {
      Ok(p0) => {
        self.scheduler.add(pid, p0);
        Ok(pid)
      },
      Err(e) => Err(e)
    }
  }

  /// Run the VM loop (one time slice), call this repeatedly to run forever.
  /// Time slice ends when a current process yields or when reduction count
  /// reaches zero.
  /// Returns: false if VM has quit, true if can continue.
  pub fn tick(&mut self) -> bool {
    self.dispatch()
  }

}
