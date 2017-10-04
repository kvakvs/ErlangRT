//!
//! Implements Erlang process, an independent computing unit of Erlang with
//! heap, stack, registers, and message queue.
//!
use emulator::mfa;
use rterror;
use term::lterm::LTerm;
use emulator::vm::VM;
use emulator::code_srv;

pub struct Process {
  pid: LTerm,
  parent_pid: LTerm,
  // heap
  // context: regs stack...
  ip: code_srv::InstrPointer,
}

impl Process {
  // Call only from VM, process must be immediately registered in proc registry for this vm
  pub fn new(vm: &mut VM, pid: LTerm,
             parent_pid: LTerm, mfa: &mfa::MFArgs) -> Result<Process, rterror::Error>
  {
    assert!(pid.is_local_pid());
    assert!(parent_pid.is_local_pid() || parent_pid.is_nil());
    match vm.code_lookup(mfa) {
      Ok(ip) => Ok(Process { pid, parent_pid, ip }),
      Err(e) => Err(e)
    }
  }

  pub fn jump(&mut self, vm: &mut VM, mfa: &mfa::MFArgs) -> Result<(), rterror::Error> {
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