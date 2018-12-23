//! Implements Erlang process, an independent computing unit of Erlang with
//! heap, stack, registers, and message queue.

use crate::{
  defs::{ExceptionType, Word},
  emulator::{
    code_srv::CodeServer,
    heap::{copy_term, Heap, DEFAULT_PROC_HEAP},
    mfa::MFArity,
    runtime_ctx, scheduler,
  },
  fail::RtResult,
  term::lterm::*,
};
use core::fmt;

fn module() -> &'static str {
  "process: "
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ProcessError {
  None,
  Exception(ExceptionType, LTerm),
}

impl fmt::Display for ProcessError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ProcessError::None => write!(f, "NoError"),
      ProcessError::Exception(exc_type, exc_reason) => match exc_type {
        ExceptionType::Exit => write!(f, "exit({})", exc_reason),
        ExceptionType::Throw => write!(f, "throw({})", exc_reason),
        ExceptionType::Error => write!(f, "error({})", exc_reason),
      },
    }
  }
}

pub struct Process {
  pub pid: LTerm,
  // parent_pid: LTerm,

  // Scheduling and fail state
  /// Scheduling priority (selects the runqueue when this process is scheduled)
  pub prio: scheduler::Prio,
  /// Current scheduler queue where this process is registered
  pub current_queue: scheduler::Queue,

  /// Runtime context with registers, instruction pointer etc
  pub context: runtime_ctx::Context,
  /// How many X registers in the context are currently used
  pub live: Word,

  pub heap: Heap,
  mailbox: Vec<LTerm>, // TODO: Some structure on proc heap?
  mailbox_read_index: usize,

  // Error handling
  /// Record result of last scheduled timeslice for this process
  /// (updated by the vm loop)
  pub timeslice_result: scheduler::SliceResult,
  pub error: ProcessError,
}

impl Process {
  // Call this only from VM, the new process must be immediately registered
  // in proc registry for this VM
  pub fn new(
    pid: LTerm,
    _parent_pid: LTerm,
    mfarity: &MFArity,
    prio: scheduler::Prio,
    code_server: &mut CodeServer,
  ) -> RtResult<Process> {
    assert!(pid.is_local_pid());
    assert!(_parent_pid.is_local_pid() || _parent_pid == LTerm::nil());

    // Process must start with some code location
    match code_server.lookup_and_load(mfarity) {
      Ok(ip) => {
        let p = Process {
          pid,
          // parent_pid: nil(),
          prio,
          current_queue: scheduler::Queue::None,
          timeslice_result: scheduler::SliceResult::None,
          heap: Heap::new(DEFAULT_PROC_HEAP),
          mailbox: Vec::with_capacity(32),
          mailbox_read_index: 0,

          context: runtime_ctx::Context::new(ip),
          live: 0,

          error: ProcessError::None,
        };
        Ok(p)
        // Ok(sync::Arc::new(sync::RwLock::new(p)))
      }
      Err(e) => Err(e),
    }
  }

  /// Returns true if there was an error or exception during the last timeslice.
  #[inline]
  pub fn is_failed(&self) -> bool {
    self.error != ProcessError::None
  }

  #[allow(dead_code)]
  pub fn jump(
    &mut self,
    mfarity: &MFArity,
    code_server: &mut CodeServer,
  ) -> RtResult<()> {
    // TODO: Find mfa in code server and set IP to it
    match code_server.lookup_and_load(mfarity) {
      Ok(ip) => {
        self.context.ip = ip;
        Ok(())
      }
      Err(e) => Err(e),
    }
  }

  pub fn exception(&mut self, exc: ExceptionType, rsn: LTerm) -> LTerm {
    self.set_error(ProcessError::Exception(exc, rsn))
  }

  /// Sets error state from an opcode or a BIF. VM will hopefully check this
  /// immediately and finish the process or catch the error.
  fn set_error(&mut self, e: ProcessError) -> LTerm {
    panic!("{}{} set_error {}", module(), self.pid, e);
    //    self.error = e;
    //    LTerm::non_value()
  }

  //  pub fn clear_error(&mut self) {
  //    self.error = ProcessError::None;
  //  }

  /// Copy a message and put into process mailbox.
  pub fn deliver_message(&mut self, message: LTerm) {
    let m1 = copy_term::copy_to(message, &mut self.heap);
    self.mailbox.push(m1);
  }

  /// Read message at the current receive pointer.
  pub fn recv_current_message(&self) -> Option<LTerm> {
    let mri = self.mailbox_read_index;
    if mri >= self.mailbox.len() {
      return None;
    }
    let val = self.mailbox[mri];
    debug_assert!(val.is_value());
    Some(val)
  }

  pub fn recv_step_over(&mut self) {
    let mut mri = self.mailbox_read_index;
    let max_mri = self.mailbox.len();
    // Increase mail receive index over nonvalues (received values) until
    // we hit the end of the mailbox
    while mri < max_mri && self.mailbox[mri].is_value() {
      mri += 1;
    }
    self.mailbox_read_index = mri;
  }
}
