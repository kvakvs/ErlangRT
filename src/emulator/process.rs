//!
//! Implements Erlang process, an independent computing unit of Erlang with
//! heap, stack, registers, and message queue.
//!
use emulator::mfa;
use rterror;
use term::low_level::Term;
use types::Word;
use emulator::vm::VM;
use emulator::code_srv;

pub struct Process {
  pid: Term,
  parent_pid: Term,
  // heap
  // context: regs stack...
  ip: code_srv::InstrPointer,
}

impl Process {
  // Call only from VM, process must be immediately registered in proc registry for this vm
  pub fn new(vm: &mut VM, pid: Term,
             parent_pid: Term, mfa: &mfa::MFArgs) -> Result<Process, rterror::Error>
  {
    assert!(pid.is_pid());
    assert!(parent_pid.is_pid() || parent_pid.is_nil());
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