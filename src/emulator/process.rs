//!
//! Implements Erlang process, an independent computing unit of Erlang with
//! heap, stack, registers, and message queue.
//!
use emulator::code::{InstrPointer};
use emulator::mfa;
use emulator::vm::VM;
use fail::Hopefully;
use term::lterm::LTerm;

pub struct Process {
  pid: LTerm,
  parent_pid: LTerm,
  // heap
  // context: regs stack...
  ip: InstrPointer,
}

impl Process {
  // Call this only from VM, the new process must be immediately registered
  // in proc registry for this VM
  pub fn new(vm: &mut VM,
             pid: LTerm,
             parent_pid: LTerm,
             mfa: &mfa::MFArgs) -> Hopefully<Process> {
    assert!(pid.is_local_pid());
    assert!(parent_pid.is_local_pid() || parent_pid.is_nil());
    match vm.code_lookup(mfa) {
      Ok(ip) => Ok(Process { pid, parent_pid, ip }),
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