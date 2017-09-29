use mfargs;
use rterror;
use term::Term;
use types::Word;
use vm::VM;
use code_srv;

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
             parent_pid: Term, mfa: &mfargs::MFArgs) -> Result<Process, rterror::Error>
  {
    assert!(pid.is_pid());
    assert!(parent_pid.is_pid() || parent_pid.is_nil());
    match vm.code_lookup(mfa) {
      Ok(ip) => Ok(Process { pid, parent_pid, ip }),
      Err(e) => Err(e)
    }
  }

  pub fn jump(&mut self, vm: &mut VM, mfa: &mfargs::MFArgs) -> Result<(), rterror::Error> {
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