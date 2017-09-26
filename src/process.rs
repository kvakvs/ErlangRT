use term::Term;
use types::Word;
use vm::VM;
use mfargs::MFArgs;

pub struct Process<'a> {
  vm: &'a VM<'a>,
  pid: Term,
  parent_pid: Term,
  // heap
  // context: regs stack...
}

type ProcessBox<'a> = Box<Process<'a>>;

impl<'a> Process<'a> {
  // Call only from VM, process must be immediately registered in proc registry for this vm
  pub fn new(vm: &'a VM, pid: Term, parent_pid: Term, mfa: &MFArgs) -> Process<'a> {
    assert!(pid.is_pid());
    assert!(parent_pid.is_pid() || parent_pid.is_nil());
    Process { vm, pid, parent_pid }
  }

  pub fn apply(&mut self, mfa: &MFArgs) {

  }
}