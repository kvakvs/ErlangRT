//!
//! Implements Erlang process, an independent computing unit of Erlang with
//! heap, stack, registers, and message queue.
//!
use emulator::heap::{Heap, DEFAULT_PROC_HEAP};
use emulator::mfa;
use emulator::runtime_ctx;
use emulator::scheduler;
use emulator::vm::VM;
use fail::Hopefully;
use term::lterm::LTerm;

//use std::sync;


//pub type Ptr = sync::Arc<sync::RwLock<Process>>;
//pub type Weak = sync::Weak<sync::RwLock<Process>>;


pub struct Process {
  pub pid: LTerm,
  parent_pid: LTerm,

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

  pub heap: Heap,
}

impl Process {
  // Call this only from VM, the new process must be immediately registered
  // in proc registry for this VM
  pub fn new(vm: &mut VM, pid: LTerm, parent_pid: LTerm, mfa: &mfa::MFArgs,
             prio: scheduler::Prio) -> Hopefully<Process> {
    assert!(pid.is_local_pid());
    assert!(parent_pid.is_local_pid() || parent_pid.is_nil());

    // Process must start with some code location
    match vm.code_lookup(mfa) {
      Ok(ip) => {
        let p = Process {
          pid, parent_pid, prio,
          current_queue: scheduler::Queue::None,
          timeslice_result: scheduler::SliceResult::None,
          fail_value: LTerm::non_value(),
          context: runtime_ctx::Context::new(ip),
          heap: Heap::new(DEFAULT_PROC_HEAP),
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
  pub fn jump(&mut self, vm: &mut VM, mfa: &mfa::MFArgs) -> Hopefully<()> {
    // TODO: Find mfa in code server and set IP to it
    match vm.code_lookup(mfa) {
      Ok(ip) => {
        self.context.ip = ip;
        Ok(())
      },
      Err(e) => Err(e)
    }
  }
}