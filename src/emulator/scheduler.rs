//! Code related to task scheduling and priorities.
use std::collections::{LinkedList, HashMap, HashSet};
//use std::sync;

use defs::Word;
use emulator::process;
use emulator::process::Process;
use emulator::gen_atoms;
use term::lterm::LTerm;


fn module() -> &'static str { "scheduler: " }


#[derive(Debug, Clone, Copy)]
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
pub enum Queue {
  None,
  //PendingTimers,
  High,
  Normal,
  Low,
  TimedWait,
  InfiniteWait,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SliceResult {
  None,
  /// Process willingly gave up run queue (ended the timeslice without events)
  Yield,
  /// Process entered infinite or timed wait during the last timeslice
  Wait,
  /// Process normally finished during the last timeslice
  Finished,
  /// Error, exit or throw occured during the last timeslice
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
  queue_low: LinkedList<LTerm>,
  queue_normal: LinkedList<LTerm>,
  queue_high: LinkedList<LTerm>,

  /// A counter used to skip some schedulings for low processes
  advantage_count: Word,

  /// Currently selected process
  current: Option<process::Ptr>,

  /// Wait set for infinitely suspended processes (in endless receive)
  wait_inf: HashSet<LTerm>,
  /// Wait set for timed suspended processes (waiting for a timer)
  wait_timed: HashSet<LTerm>,

  /// Dict of pids to process boxes. Owned by the scheduler
  processes: HashMap<LTerm, process::Ptr>,
}


impl Scheduler {
  pub fn new() -> Scheduler {
    Scheduler{
      queue_low: LinkedList::new(),
      queue_normal: LinkedList::new(),
      queue_high: LinkedList::new(),

      advantage_count: NORMAL_ADVANTAGE,
      current: None,

      wait_inf: HashSet::new(),
      wait_timed: HashSet::new(),

      processes: HashMap::new(),
    }
  }


  /// Register a process `proc_` in the process table and also queue it for
  /// execution. This is invoked by vm when a new process is spawned.
  pub fn add(&mut self, pid: LTerm, proc_: process::Ptr) {
    self.processes.insert(pid, proc_.clone());
    self.queue(pid);
  }


  /// Queue a process by its pid. Will `panic!` if the process doesn't exist
  /// or is already queued.
  pub fn queue(&mut self, pid: LTerm) {
    assert!(pid.is_local_pid());

    let prio = {
      // Lookup the pid
      let p_arc = self.lookup_pid(pid).unwrap();

      // Read-lock the rwlock inside Arc
      let p = p_arc.read().unwrap();

      assert_eq!(p.current_queue, Queue::None);
      p.prio
    };

    match prio {
      Prio::Normal => self.queue_normal.push_back(pid),
      Prio::Low => self.queue_low.push_back(pid),
      Prio::High => self.queue_high.push_back(pid),
    }
  }


  /// Get another process from the run queue for this scheduler.
  pub fn next(&mut self) {
    // FIXME: Ugly clone on self.current
    if let Some(curr_arc) = self.current.clone() {
      let curr = curr_arc.read().unwrap();
      assert_eq!(curr.current_queue, Queue::None);
      let curr_pid = curr.pid;

      match &curr.timeslice_result {
        &SliceResult::Yield => {
          self.queue(curr_pid);
          self.current = None
        },
        &SliceResult::None => {
          self.queue(curr_pid);
          self.current = None
        },
        &SliceResult::Finished => {
          self.exit_process(&curr, gen_atoms::NORMAL)
        },
        &SliceResult::Exception => {
          assert!(curr.is_failed());
          let fail_value = curr.fail_value;
          self.exit_process(&curr, fail_value);
          self.current = None
        },
        &SliceResult::Wait => {},
      }
    } // if self.current
  }


  /// Get a pointer to process, if it exists. Return `None` if we are sorry.
  pub fn lookup_pid(&self, pid: LTerm) -> Option<process::Ptr> {
    assert!(pid.is_local_pid());
    match self.processes.get(&pid) {
      Some(p) => Some(p.clone()),
      None => None
    }
  }


  pub fn exit_process(&mut self, p: &Process, reason: LTerm) {
    // assert that process is not in any queue
    assert_eq!(p.current_queue, Queue::None);

    // root process exits with halt()
    // assert!(p.get_registered_name() != atom::INIT);

    // TODO: ets tables
    // TODO: notify monitors
    // TODO: cancel known timers who target this process
    // TODO: notify links
    // TODO: unregister name if registered
    // TODO: if pending timers - become zombie and sit in pending timers queue
    let pid = p.pid;

    println!("{}Scheduler::exit_process {} reason={}, result x0=?",
             module(), pid, reason //, p.runtime_ctx.regs[0]
            );

    //  m_inf_wait.erase(p);
    //  m_timed_wait.erase(p);
    assert!(!self.queue_normal.contains(&pid));
    assert!(!self.queue_low.contains(&pid));
    assert!(!self.queue_high.contains(&pid));
    self.processes.remove(&pid);
  }
}
