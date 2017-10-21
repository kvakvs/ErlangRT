//!
//! Implements virtual machine, as a collection of processes and their
//! registrations, schedulers, ETS tables and atom table etc.
//!
use std::path::PathBuf;

use defs::Word;
use emulator::atom;
use emulator::code::CodePtr;
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

  //code_srv: code_srv::CodeServer,

  pub scheduler: Scheduler,
}

impl VM {
  pub fn new() -> VM {
    VM {
      pid_counter: 0,
      //code_srv: code_srv::CodeServer::new(),
      scheduler: Scheduler::new(),
    }
  }

  // Spawn a new process, create a new pid, register the process and jump to the MFA
  pub fn create_process(&mut self,
                        parent: LTerm,
                        mfargs: &MFArgs,
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

}
