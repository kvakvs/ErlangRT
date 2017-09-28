use term::Term;
use types::Word;
use vm::VM;
use mfargs::MFArgs;

pub struct Process {
  pid: Term,
  parent_pid: Term,
  // heap
  // context: regs stack...
}

impl Process {
  // Call only from VM, process must be immediately registered in proc registry for this vm
  pub fn new(pid: Term,
             parent_pid: Term) -> Process
  {
    assert!(pid.is_pid());
    assert!(parent_pid.is_pid() || parent_pid.is_nil());
    Process { pid, parent_pid }
  }

  pub fn jump(&mut self, vm: &VM, mfa: &MFArgs) {
    // TODO: Find mfa in code server and set IP to it
  }
}