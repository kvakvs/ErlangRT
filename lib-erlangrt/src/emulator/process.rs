//! Implements Erlang process, an independent computing unit of Erlang with
//! heap, stack, registers, and message queue.

use crate::{
  defs::exc_type::ExceptionType,
  emulator::{
    code_srv::CodeServer,
    heap::{copy_term, Designation, Heap},
    mailbox::ProcessMailbox,
    mfa::{ModFunArgs, ModFunArity},
    process_flags::ProcessFlags,
    process_registry::ProcessRegistry,
    runtime_ctx,
    scheduler::{self, Scheduler},
    spawn_options::SpawnOptions,
  },
  fail::RtResult,
  term::value::*,
};
use core::ptr;
use crate::emulator::heap::heap_trait::THeap;

//#[allow(dead_code)]
//#[derive(Debug, Eq, PartialEq, Copy, Clone)]
// pub enum ProcessError {
//  None,
//  Exception(ExceptionType, Term),
//}

// impl fmt::Display for ProcessError {
//  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//    match *self {
//      None => write!(f, "NoError"),
//      Some((exc_type, exc_reason)) => match exc_type {
//        ExceptionType::Exit => write!(f, "exit({})", exc_reason),
//        ExceptionType::Throw => write!(f, "throw({})", exc_reason),
//        ExceptionType::Error => write!(f, "error({})", exc_reason),
//        ExceptionType::Panic => write!(f, "panic({})", exc_reason),
//      },
//    }
//  }
//}

pub struct Process {
  pub pid: Term,

  // Scheduling and fail state
  /// Scheduling priority (selects the runqueue when this process is scheduled)
  pub prio: scheduler::Prio,

  /// Current scheduler queue where this process is registered
  pub current_queue: scheduler::Queue,
  pub owned_by_scheduler: *mut Scheduler,

  // Execution Context, etc.
  /// Runtime context with registers, instruction pointer etc
  pub context: runtime_ctx::Context,

  // Memory
  heap: Heap,
  pub mailbox: ProcessMailbox,

  // Error handling
  /// Record result of last scheduled timeslice for this process
  /// (updated by the vm loop)
  pub timeslice_result: scheduler::SliceResult,
  /// Error field is set on exception when the execution loop is interrupted
  /// with `DispatchResult::Exception`
  pub error: Option<(ExceptionType, Term)>,
  /// How many catch frames are there on stack
  pub num_catches: isize,

  pub process_flags: ProcessFlags,
}

impl Process {
  // Call this only from VM, the new process must be immediately registered
  // in proc registry for this VM
  pub fn new(
    pid: Term,
    _parent_pid: Term,
    mfarity: &ModFunArity,
    spawn_opts: &SpawnOptions,
    code_server: &mut CodeServer,
  ) -> RtResult<Process> {
    assert!(pid.is_local_pid());
    assert!(_parent_pid.is_local_pid() || _parent_pid == Term::nil());

    // Process must start with some code location
    match code_server.lookup_beam_code_and_load(mfarity) {
      Ok(ip) => {
        let p = Process {
          pid,
          process_flags: spawn_opts.process_flags,

          // Scheduling
          prio: spawn_opts.prio,
          current_queue: scheduler::Queue::None,
          timeslice_result: scheduler::SliceResult::None,
          owned_by_scheduler: ptr::null_mut(),

          // Memory
          heap: Heap::new(Designation::ProcessHeap),
          mailbox: ProcessMailbox::new(),

          // Execution
          context: runtime_ctx::Context::new(ip),

          error: None,
          num_catches: 0,
        };
        Ok(p)
        // Ok(sync::Arc::new(sync::RwLock::new(p)))
      }
      Err(e) => Err(e),
    }
  }

  #[inline]
  pub fn get_heap(&self) -> &THeap { &self.heap as &THeap }

  #[inline]
  pub fn get_heap_mut(&mut self) -> &mut THeap {
    // &self.heap as &mut THeap
    let heap_ref = &mut self.heap;
    heap_ref as &mut THeap
  }

  /// Copy args from mfargs-MFA-something into new process heap and set the
  /// registers to the arguments passed to spawn.
  pub fn set_spawn_args(&mut self, mfargs: &ModFunArgs) -> RtResult<()> {
    let mut xindex = 0;
    mfargs.for_each_arg(|arg| -> RtResult<()> {
      self.context.set_x(xindex, arg);
      xindex += 1;
      Ok(())
    })
  }

  /// Returns true if there was an error or exception during the last timeslice.
  #[inline]
  pub fn is_failed(&self) -> bool {
    self.error.is_some()
  }

  #[inline]
  pub fn clear_exception(&mut self) {
    self.error = None;
  }

  #[allow(dead_code)]
  pub fn jump(
    &mut self,
    mfarity: &ModFunArity,
    code_server: &mut CodeServer,
  ) -> RtResult<()> {
    // TODO: Find mfa in code server and set IP to it
    match code_server.lookup_beam_code_and_load(mfarity) {
      Ok(ip) => {
        self.context.ip = ip;
        Ok(())
      }
      Err(e) => Err(e),
    }
  }

  pub fn set_exception(&mut self, exc_type: ExceptionType, reason: Term) {
    // panic!("{}{} set_error {}", module(), self.pid, e);
    self.error = Some((exc_type, reason));
  }

  //  pub fn clear_error(&mut self) {
  //    self.error = ProcessError::None;
  //  }

  /// Copy a message and put into process mailbox.
  pub fn deliver_message(
    &mut self,
    proc_reg: &mut ProcessRegistry,
    message: Term,
  ) -> RtResult<()> {
    let m1 = copy_term::copy_to(message, &mut self.heap)?;
    self.mailbox.put(m1);

    // Notify our current scheduler that a new message has come to possibly wake
    // up from infinite or timed wait.
    unsafe {
      (*self.owned_by_scheduler).notify_new_incoming_message(proc_reg, self);
    }
    Ok(())
  }

  /// Ugly hack to mut-borrow the context without making borrow checker sad.
  /// We guarantee that this borrow will not outlive the process, or we will pay
  /// the price debugging the SIGSEGV.
  #[inline]
  pub fn get_context_p(&self) -> *mut runtime_ctx::Context {
    let p = &self.context as *const runtime_ctx::Context;
    p as *mut runtime_ctx::Context
  }
}
