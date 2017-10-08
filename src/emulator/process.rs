//!
//! Implements Erlang process, an independent computing unit of Erlang with
//! heap, stack, registers, and message queue.
//!
use emulator::code::{InstrPointer};
use emulator::mfa;
use emulator::scheduler;
use emulator::vm::VM;
use fail::Hopefully;
use term::lterm::LTerm;

use std::sync;


pub type Ptr = sync::Mutex<Process>;
pub type Weak = sync::Mutex<Process>;


pub struct Process {
  pub pid: LTerm,
  parent_pid: LTerm,

  /// Scheduling priority
  pub prio: scheduler::Prio,
  /// Scheduler queue
  pub current_queue: scheduler::Queue,

  // heap
  // Runtime context: regs stack...
  ip: InstrPointer,
}

impl Process {
  // Call this only from VM, the new process must be immediately registered
  // in proc registry for this VM
  pub fn new(vm: &mut VM, pid: LTerm, parent_pid: LTerm, mfa: &mfa::MFArgs,
             prio: scheduler::Prio) -> Hopefully<Ptr> {
    assert!(pid.is_local_pid());
    assert!(parent_pid.is_local_pid() || parent_pid.is_nil());

    // Process must start with some code location
    match vm.code_lookup(mfa) {
      Ok(ip) => {
        let p = Process {
          pid, parent_pid, ip, prio,
          current_queue: scheduler::Queue::None,
        };
        Ok(sync::Mutex::new(p))
      },
      Err(e) => Err(e)
    }
  }

  #[allow(dead_code)]
  pub fn jump(&mut self, vm: &mut VM, mfa: &mfa::MFArgs) -> Hopefully<()> {
    // TODO: Find mfa in code server and set IP to it
    match vm.code_lookup(mfa) {
      Ok(ip) => {
        self.ip = ip;
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}