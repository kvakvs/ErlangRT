//! Code related to task scheduling and priorities.
use std::collections::{LinkedList, HashMap, HashSet};

use defs::Word;
use emulator::process;
use term::lterm::LTerm;


#[derive(Debug, Clone)]
pub enum Prio {
  /// Runs when no more jobs to take or at 8x disadvantage to normal
  Low = 0,
  /// Most of user processes run at this priority
  Normal = 1,
  /// Takes priority always over everything else
  High = 2,
}


/// Enum identifies current registration of the process
#[derive(Debug, Clone)]
pub enum Queue {
  None,
  //PendingTimers,
  High,
  Normal,
  Low,
  TimedWait,
  InfiniteWait,
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

      wait_inf: HashSet::new(),
      wait_timed: HashSet::new(),

      processes: HashMap::new(),
    }
  }


  pub fn add(&mut self, pid: LTerm, proc_: process::Ptr) {
    {
      let mut p = proc_.lock().unwrap();
      self.queue(pid, p.prio.clone());
    }
    self.processes.insert(pid, proc_);
  }


  pub fn queue(&mut self, pid: LTerm, prio: Prio) {

  }


  pub fn next(&mut self) {

  }


  pub fn lookup(&self, pid: LTerm) -> Option<&process::Ptr> {
    self.processes.get(&pid)
  }
}
