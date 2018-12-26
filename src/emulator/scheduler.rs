//! Code related to task scheduling and priorities.
use crate::{
  defs::{ExceptionType, Word},
  emulator::{
    gen_atoms,
    process::{Process, ProcessError},
  },
  term::lterm::*,
};
use std::collections::{HashMap, VecDeque};

fn module() -> &'static str {
  "scheduler: "
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Prio {
  /// Runs when no more jobs to take or at 8x disadvantage to normal
  Low = 0,
  /// Most of user processes run at this priority
  Normal = 1,
  /// Takes priority always over everything else
  High = 2,
}

/// Enum identifies current registration of the process
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Queue {
  None,
  // PendingTimers,
  High,
  Normal,
  Low,
  TimedWait,
  InfiniteWait,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
pub enum SliceResult {
  None,
  /// Process willingly gave up run queue (ended the timeslice without events)
  Yield,
  /// Process entered infinite or timed wait during the last timeslice
  Wait,
  /// Process normally finished during the last timeslice
  Finished,
  /// Error, exit or throw occured during the last timeslice, error is stored
  /// in the process, field `error`
  Exception,
}

/// How many Normal processes can be scheduled before Low gets to run.
const NORMAL_ADVANTAGE: Word = 8;

/// Maintains run queues for different priorities and allows queuing processes,
/// suspending processes, work balancing (TODO), etc.
pub struct Scheduler {
  // This is the naive implementation of run queues.
  // A better approach would be to build an intrusive double linked list through
  // every process in the queue (as done by the original ERTS).
  queue_low: VecDeque<LTerm>,
  queue_normal: VecDeque<LTerm>,
  queue_high: VecDeque<LTerm>,
  timed_wait: HashMap<LTerm, ()>,
  infinite_wait: HashMap<LTerm, ()>,

  /// A counter used to skip some schedulings for low processes
  advantage_count: Word,

  /// Currently selected process
  current: Option<LTerm>,

  //  /// Wait set for infinitely suspended processes (in endless receive)
  //  wait_inf: HashSet<LTerm>,
  //  /// Wait set for timed suspended processes (waiting for a timer)
  //  wait_timed: HashSet<LTerm>,
  /// Dict of pids to process boxes. Owned by the scheduler
  processes: HashMap<LTerm, Process>,
}

impl Scheduler {
  pub fn new() -> Self {
    Self {
      queue_low: VecDeque::new(),
      queue_normal: VecDeque::new(),
      queue_high: VecDeque::new(),
      timed_wait: HashMap::new(),
      infinite_wait: HashMap::new(),

      advantage_count: 0,
      current: None,

      processes: HashMap::new(),
    }
  }

  /// Register a process `proc_` in the process table and also queue it for
  /// execution. This is invoked by vm when a new process is spawned.
  pub fn register_new_process(&mut self, pid: LTerm, mut proc: Process) {
    proc.owned_by_scheduler = self as *mut Scheduler;
    self.processes.insert(pid, proc);
    self.enqueue(pid);
  }

  /// Queue a process by its pid. Will `panic!` if the process doesn't exist
  /// or is already queued.
  pub fn enqueue(&mut self, pid: LTerm) {
    assert!(pid.is_local_pid());

    let prio = {
      // Lookup the pid
      let p = self.lookup_pid(pid).unwrap();
      assert_eq!(
        p.current_queue,
        Queue::None,
        "Process must not be in any queue when queuing, now in {:?}",
        p.current_queue
      );
      p.prio
    };

    match prio {
      Prio::Normal => self.queue_normal.push_back(pid),
      Prio::Low => self.queue_low.push_back(pid),
      Prio::High => self.queue_high.push_back(pid),
    }
  }

  /// Queue a process by its pid into either timed_wait or infinite_wait queue.
  #[inline]
  pub fn enqueue_wait(&mut self, infinite: bool, pid: LTerm) {
    assert!(pid.is_local_pid());

    if infinite {
      self.infinite_wait.insert(pid, ());
    } else {
      self.timed_wait.insert(pid, ());
    }
  }

  /// Get another process from the run queue for this scheduler.
  /// Returns: `Option(pid)`
  pub fn next_process(&mut self) -> Option<LTerm> {
    if let Some(prev_pid) = self.current {
      self.next_process_finalize_previous(prev_pid);
    }
    self.next_process_duties();

    // Now try and find another process to run
    loop {
      // See if any are waiting in realtime (high) priority queue
      if let Some(next_pid) = self.next_process_pick_from_the_queues() {
        self.current = Some(next_pid);
        break;
      }
    }

    self.current
  }

  /// Look through the queues and find some queue with highest priority where
  /// a process is waiting to be selected.
  /// Advantage counter allows running lower queues even if a higher is running.
  fn next_process_pick_from_the_queues(&mut self) -> Option<LTerm> {
    if !self.queue_high.is_empty() {
      return self.queue_high.pop_front();
    } else if self.advantage_count < NORMAL_ADVANTAGE {
      if !self.queue_normal.is_empty() {
        return self.queue_normal.pop_front();
      } else if !self.queue_low.is_empty() {
        return self.queue_low.pop_front();
      }
      self.advantage_count += 1;
    } else {
      if !self.queue_low.is_empty() {
        return self.queue_low.pop_front();
      } else if !self.queue_normal.is_empty() {
        return self.queue_normal.pop_front();
      }
      self.advantage_count = 0;
    };
    return None;
  }

  /// When time has come to select next running process, first we take a look
  /// at the previous process, what happened to it.
  #[inline]
  fn next_process_finalize_previous(&mut self, curr_pid: LTerm) {
    // Extract the last running process from the process registry
    let curr = self.lookup_pid(curr_pid).unwrap();
    debug_assert_eq!(
      curr.current_queue,
      Queue::None,
      "Finalizing previous process which is not dequeued, now in {:?}",
      curr.current_queue
    );

    match curr.timeslice_result {
      SliceResult::Yield | SliceResult::None => {
        self.enqueue(curr_pid);
        self.current = None
      }

      SliceResult::Finished => {
        let err = ProcessError::Exception(ExceptionType::Exit, gen_atoms::NORMAL);
        self.terminate_process(curr_pid, err)
      }

      SliceResult::Exception => {
        let curr = self.lookup_pid(curr_pid).unwrap();
        assert!(curr.is_failed());
        let p_error = curr.error;
        self.terminate_process(curr_pid, p_error);
        self.current = None
      }

      SliceResult::Wait => {
        self.enqueue_wait(true, curr_pid);
      }
    }
  }

  #[inline]
  fn next_process_duties(&self) {
    // TODO: monotonic clock
    // TODO: wait lists
    // TODO: network checks
  }

  /// Get a read-only process, if it exists. Return `None` if we are sorry.
  pub fn lookup_pid(&self, pid: LTerm) -> Option<&Process> {
    assert!(pid.is_local_pid());
    self.processes.get(&pid)
  }

  /// Get a reference to process, if it exists. Return `None` if we are sorry.
  pub fn lookup_pid_mut(&mut self, pid: LTerm) -> Option<&mut Process> {
    assert!(pid.is_local_pid());
    self.processes.get_mut(&pid)
  }

  /// Assuming that the error was not caught, begin process termination routine.
  pub fn terminate_process(&mut self, pid: LTerm, e: ProcessError) {
    // assert that process is not in any queue
    {
      let p = self.lookup_pid_mut(pid).unwrap();
      assert_eq!(p.current_queue, Queue::None);
    }

    // root process exits with halt()
    // assert!(p.get_registered_name() != atom::INIT);

    // TODO: ets tables
    // TODO: notify monitors
    // TODO: cancel known timers who target this process
    // TODO: notify links
    // TODO: unregister name if registered
    // TODO: if pending timers - become zombie and sit in pending timers queue
    println!(
      "{}exit_process {} e={}, result x0=?",
      module(),
      pid,
      e //, p.runtime_ctx.regs[0]
    );

    //  m_inf_wait.erase(p);
    //  m_timed_wait.erase(p);
    assert!(!self.queue_normal.contains(&pid));
    assert!(!self.queue_low.contains(&pid));
    assert!(!self.queue_high.contains(&pid));
    self.processes.remove(&pid);
  }

  /// Called by `Process` when a new message is received. Checks whether the
  /// process was placed in one of waiting queues and wakes it up.
  #[inline]
  pub fn notify_new_incoming_message(&mut self, proc: &mut Process) {
    match proc.current_queue {
      Queue::InfiniteWait => {
        self.infinite_wait.remove(&proc.pid);
        self.enqueue(proc.pid);
      },
      Queue::TimedWait => {
        self.timed_wait.remove(&proc.pid);
        self.enqueue(proc.pid);
      },
      _other => {},
    }
  }
}
