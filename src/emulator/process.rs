//!
//! Implements Erlang process, an independent computing unit of Erlang with
//! heap, stack, registers, and message queue.
//!

use defs::Word;
use emulator::code_srv;
use emulator::heap::{Heap, DEFAULT_PROC_HEAP};
use emulator::mfa::MFArity;
use emulator::runtime_ctx;
use emulator::scheduler;
use fail::Hopefully;
use term::lterm::LTerm;


pub enum ErrorType {
  None,
  Exit,
  Throw,
  Error,
}


pub struct Process {
  pub pid: LTerm,
  //parent_pid: LTerm,

  //
  // Scheduling and fail state
  //

  /// Scheduling priority (selects the runqueue when this process is scheduled)
  pub prio: scheduler::Prio,
  /// Current scheduler queue where this process is registered
  pub current_queue: scheduler::Queue,
  /// Record result of last scheduled timeslice for this process
  /// (updated by the vm loop)
  pub timeslice_result: scheduler::SliceResult,
  pub fail_value: LTerm,

  /// Runtime context with registers, instruction pointer etc
  pub context: runtime_ctx::Context,
  /// How many X registers in the context are currently used
  pub live: Word,

  pub heap: Heap,

  //
  // Error handling
  //
  pub error_type: ErrorType,
  pub error_reason: LTerm,
}


impl Process {
  // Call this only from VM, the new process must be immediately registered
  // in proc registry for this VM
  pub fn new(pid: LTerm, _parent_pid: LTerm, mfarity: &MFArity,
             prio: scheduler::Prio) -> Hopefully<Process> {
    assert!(pid.is_local_pid());
    assert!(_parent_pid.is_local_pid() || _parent_pid.is_nil());

    // Process must start with some code location
    match code_srv::lookup_and_load(mfarity) {
      Ok(ip) => {
        let p = Process {
          pid,
          //parent_pid: LTerm::nil(),
          prio,
          current_queue: scheduler::Queue::None,
          timeslice_result: scheduler::SliceResult::None,
          fail_value: LTerm::non_value(),
          heap: Heap::new(DEFAULT_PROC_HEAP),

          context: runtime_ctx::Context::new(ip),
          live: 0,

          error_type: ErrorType::None,
          error_reason: LTerm::nil(),
        };
        Ok(p)
        //Ok(sync::Arc::new(sync::RwLock::new(p)))
      },
      Err(e) => Err(e)
    }
  }


  /// Returns true if there was an error or exception during the last timeslice.
  #[inline]
  pub fn is_failed(&self) -> bool {
    self.fail_value.is_value()
  }


  #[allow(dead_code)]
  pub fn jump(&mut self, mfarity: &MFArity) -> Hopefully<()> {
    // TODO: Find mfa in code server and set IP to it
    match code_srv::lookup_and_load(mfarity) {
      Ok(ip) => {
        self.context.ip = ip;
        Ok(())
      },
      Err(e) => Err(e)
    }
  }


  pub fn exit(&mut self, rsn: LTerm) -> LTerm {
    self.set_error(ErrorType::Exit, rsn)
  }


  pub fn throw(&mut self, rsn: LTerm) -> LTerm {
    self.set_error(ErrorType::Throw, rsn)
  }


  pub fn error(&mut self, rsn: LTerm) -> LTerm {
    self.set_error(ErrorType::Error, rsn)
  }


  /// Sets error state from an opcode or a BIF. VM will hopefully check this
  /// immediately and finish the process or catch the error.
  fn set_error(&mut self, t: ErrorType, rsn: LTerm) -> LTerm {
    self.error_type = t;
    self.error_reason = rsn;
    LTerm::non_value()
  }


  pub fn clear_error(&mut self) {
    self.error_type = ErrorType::None;
    self.error_reason = LTerm::nil();
  }
}
